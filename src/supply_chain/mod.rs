// src/supply_chain/mod.rs

use crate::error::SystemError;
use miette::{Diagnostic, IntoDiagnostic, Report, Result};
use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::path::Path;
use thiserror::Error;

// --- Beautiful Miette Diagnostics for Supply Chain ---

#[derive(Error, Debug)]
#[error("[CRITICAL] Compromised Dependency: {package_name}@{version}")]
pub struct VulnerabilityDiagnostic {
    pub package_name: String,
    pub version: String,
    pub ecosystem: String,
    pub cve_ids: String,
    pub remediation: String,
}

impl Diagnostic for VulnerabilityDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(format!(
            "woof::osv::{}",
            self.ecosystem.to_lowercase()
        )))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(self.remediation.clone()))
    }
}

// --- The OSV Batch Query Logic ---

fn batch_query_osv(dependencies: &[(String, String, String)]) -> Result<bool> {
    if dependencies.is_empty() {
        return Ok(true);
    }

    let client = Client::new();
    let queries: Vec<_> = dependencies.iter().map(|(name, version, ecosystem)| {
        json!({ "version": version, "package": { "name": name, "ecosystem": ecosystem } })
    }).collect();

    let response = client
        .post("https://api.osv.dev/v1/querybatch")
        .json(&json!({ "queries": queries }))
        .send()
        .into_diagnostic()?;

    let response_json: serde_json::Value = response.json().into_diagnostic()?;
    let mut is_clean = true;

    if let Some(results) = response_json.get("results").and_then(|r| r.as_array()) {
        for (index, result) in results.iter().enumerate() {
            if let Some(vulns) = result.get("vulns").and_then(|v| v.as_array()) {
                if !vulns.is_empty() {
                    is_clean = false;
                    let (pkg, ver, eco) = &dependencies[index];

                    // Extract CVE IDs if available
                    let cves: Vec<&str> = vulns
                        .iter()
                        .filter_map(|v| v.get("aliases").and_then(|a| a.as_array()))
                        .flat_map(|a| a.iter().filter_map(|id| id.as_str()))
                        .filter(|id| id.starts_with("CVE-") || id.starts_with("GHSA-"))
                        .collect();

                    let cve_string = if cves.is_empty() {
                        "Unknown CVE".to_string()
                    } else {
                        cves.join(", ")
                    };

                    println!(
                        "{:?}",
                        Report::new(VulnerabilityDiagnostic {
                            package_name: pkg.clone(),
                            version: ver.clone(),
                            ecosystem: eco.clone(),
                            cve_ids: cve_string.clone(),
                            remediation: format!(
                                "Run `woof remediate {} <secure_version>` or update lockfile. Associated threats: {}",
                                pkg, cve_string
                            ),
                        })
                    );
                }
            }
        }
    }
    Ok(is_clean)
}

// --- Multi-Ecosystem Parser ---

pub fn audit_dependencies() -> Result<bool> {
    println!("[INFO] Initiating multi-ecosystem lockfile audit...");
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
        let parsed: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "package-lock.json".to_string(),
                source: e.into(),
            })?;

        if let Some(packages) = parsed.get("packages").and_then(|p| p.as_object()) {
            for (path, details) in packages {
                if path.is_empty() {
                    continue;
                }
                let name = path.split("node_modules/").last().unwrap_or(path);
                if let Some(version) = details.get("version").and_then(|v| v.as_str()) {
                    all_deps.push((name.to_string(), version.to_string(), "npm".to_string()));
                }
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

    // 4. Python (requirements.txt) - Basic fallback
    if Path::new("requirements.txt").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("requirements.txt").into_diagnostic()?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split("==").collect();
            if parts.len() == 2 {
                all_deps.push((
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                    "PyPI".to_string(),
                ));
            }
        }
    }

    // 5. pnpm (pnpm-lock.yaml)
    if Path::new("pnpm-lock.yaml").exists() {
        lockfiles_found += 1;
        let content = fs::read_to_string("pnpm-lock.yaml").into_diagnostic()?;

        let parsed: serde_json::Value =
            serde_yml::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "pnpm-lock.yaml".to_string(),
                source: e.into(),
            })?;

        // pnpm-lock.yaml structure puts dependencies under "importers" or "packages"
        // This is a basic extraction assuming v6/v9 lockfile format
        if let Some(packages) = parsed.get("packages").and_then(|p| p.as_object()) {
            for (path, _) in packages {
                // pnpm packages often look like "/@biomejs/biome@1.9.4" or "/react@18.2.0"
                if path.is_empty() || !path.contains('@') {
                    continue;
                }

                // Extremely basic parsing
                let parts: Vec<&str> = path.trim_start_matches('/').rsplitn(2, '@').collect();
                if parts.len() == 2 {
                    let version = parts[0];
                    let name = parts[1];
                    all_deps.push((
                        name.to_string(),
                        version.to_string(),
                        "npm".to_string(),
                    ));
                }
            }
        }
    }

    if lockfiles_found == 0 {
        return Err(SystemError::NoLockfilesFound.into());
    }

    println!(
        "[INFO] Extracted {} dependencies across {} ecosystems. Querying OSV database...",
        all_deps.len(),
        lockfiles_found
    );

    let is_clean = batch_query_osv(&all_deps)?;

    if is_clean {
        println!("[INFO] Audit complete. Zero supply chain vulnerabilities detected.");
    }

    Ok(is_clean)
}

use std::process::{Command, Stdio};

/// Automates package manager commands to force a secure version of a dependency.
pub fn remediate_vulnerability(package: &str, target_version: &str) -> Result<()> {
    println!("[INFO] Remediating {} to version {}...", package, target_version);
    let package_target = format!("{}@{}", package, target_version);

    let (cmd, args) = if Path::new("pnpm-lock.yaml").exists() {
        ("pnpm", vec!["add", &package_target])
    } else if Path::new("yarn.lock").exists() {
        ("yarn", vec!["add", &package_target])
    } else if Path::new("Cargo.toml").exists() {
        ("cargo", vec!["add", &package_target])
    } else {
        ("npm", vec!["install", &package_target])
    };

    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .into_diagnostic()?;

    if status.success() {
        println!("[INFO] Remediation successful. Asset locked to secure version.");
    } else {
        miette::bail!("[ERROR] Remediation failed. Ensure your package manager is installed and functioning.");
    }

    Ok(())
}
