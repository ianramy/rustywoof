// src/supply_chain/parsers.rs

use crate::error::SystemError;
use miette::{IntoDiagnostic, Result};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct NpmLockfile {
    #[serde(default)]
    packages: std::collections::HashMap<String, NpmPackage>,
}

#[derive(Deserialize)]
struct NpmPackage {
    version: Option<String>,
}

#[derive(Deserialize)]
struct PnpmLockfile {
    #[serde(default)]
    packages: std::collections::HashMap<String, serde::de::IgnoredAny>,
}

/// Sweeps the directory for lockfiles and extracts all dependencies.
pub type DependencyData = (Vec<(String, String, String)>, usize);
pub fn extract_dependencies() -> Result<DependencyData> {
    let mut all_deps: Vec<(String, String, String)> = Vec::new();
    let mut lockfiles_found = 0;

    // 1. Rust (Cargo.lock)
    if Path::new("Cargo.lock").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("Cargo.lock").into_diagnostic()?;
        let parsed: toml::Value =
            toml::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "Cargo.lock".to_string(),
                source: e.into(),
            })?;

        if let Some(packages) = parsed.get("package").and_then(|p| p.as_array()) {
            for pkg in packages {
                if let (Some(name), Some(version)) = (pkg.get("name"), pkg.get("version")) {
                    all_deps.push((
                        name.as_str().unwrap().to_string(),
                        version.as_str().unwrap().to_string(),
                        "crates.io".to_string(),
                    ));
                }
            }
        }
    }

    // 2. Node.js (package-lock.json)
    if Path::new("package-lock.json").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("package-lock.json").into_diagnostic()?;

        let parsed: NpmLockfile =
            serde_json::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "package-lock.json".to_string(),
                source: e.into(),
            })?;

        for (path, details) in parsed.packages {
            if path.is_empty() {
                continue;
            }
            let name = path.split("node_modules/").last().unwrap_or(&path);
            if let Some(version) = details.version {
                all_deps.push((name.to_string(), version, "npm".to_string()));
            }
        }
    }

    // 3. Python (poetry.lock)
    if Path::new("poetry.lock").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("poetry.lock").into_diagnostic()?;
        let parsed: toml::Value =
            toml::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "poetry.lock".to_string(),
                source: e.into(),
            })?;

        if let Some(packages) = parsed.get("package").and_then(|p| p.as_array()) {
            for pkg in packages {
                if let (Some(name), Some(version)) = (pkg.get("name"), pkg.get("version")) {
                    all_deps.push((
                        name.as_str().unwrap().to_string(),
                        version.as_str().unwrap().to_string(),
                        "PyPI".to_string(),
                    ));
                }
            }
        }
    }

    // 4. Python (requirements.txt)
    if Path::new("requirements.txt").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("requirements.txt").into_diagnostic()?;

        let req_regex =
            Regex::new(r"^([a-zA-Z0-9_\-]+)(?:[=><~^]+)\s*([a-zA-Z0-9_\-\.]+)").unwrap();

        for line in content.lines() {
            let clean_line = line.split(';').next().unwrap_or(line).trim();
            if clean_line.is_empty() || clean_line.starts_with('#') {
                continue;
            }
            if let Some(captures) = req_regex.captures(clean_line) {
                all_deps.push((
                    captures[1].trim().to_string(),
                    captures[2].trim().to_string(),
                    "PyPI".to_string(),
                ));
            }
        }
    }

    // 5. pnpm (pnpm-lock.yaml)
    if Path::new("pnpm-lock.yaml").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("pnpm-lock.yaml").into_diagnostic()?;

        let parsed: PnpmLockfile =
            serde_norway::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "pnpm-lock.yaml".to_string(),
                source: e.into(),
            })?;

        for (path, _) in parsed.packages {
            if path.is_empty() || !path.contains('@') {
                continue;
            }

            let parts: Vec<&str> = path.trim_start_matches('/').rsplitn(2, '@').collect();
            if parts.len() == 2 {
                let mut version = parts[0];
                let name = parts[1];

                if let Some(clean_version) = version.split('(').next() {
                    version = clean_version;
                }
                all_deps.push((name.to_string(), version.to_string(), "npm".to_string()));
            }
        }
    }

    // 6. Yarn (yarn.lock)
    if Path::new("yarn.lock").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("yarn.lock").into_diagnostic()?;

        let mut current_pkg = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if !line.starts_with(' ') && line.contains('@') && line.ends_with(':') {
                let clean_line = trimmed.trim_matches('"').trim_matches(':');

                // Collapsed `rfind` and `idx > 0` check here:
                if let Some(idx) = clean_line.rfind('@')
                    && idx > 0
                {
                    current_pkg = clean_line[..idx].to_string();
                }
            } else if trimmed.starts_with("version ") && !current_pkg.is_empty() {
                let version = trimmed.replace("version ", "").replace('\"', "");
                all_deps.push((current_pkg.clone(), version, "npm".to_string()));
                current_pkg.clear();
            }
        }
    }

    if lockfiles_found == 0 {
        return Err(SystemError::NoLockfilesFound.into());
    }

    Ok((all_deps, lockfiles_found))
}
