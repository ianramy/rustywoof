// src/supply_chain/osv.rs

use miette::{Diagnostic, IntoDiagnostic, Report, Result};
use reqwest::blocking::Client;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;
use std::{env, fs};

#[derive(Debug)]
pub struct VulnerabilityDiagnostic {
    pub package_name: String,
    pub version: String,
    pub ecosystem: String,
    pub cve_ids: String,
    pub fixed_version_display: Option<String>,
    pub fixed_version_cmd: Option<String>,
}

impl std::error::Error for VulnerabilityDiagnostic {}

impl std::fmt::Display for VulnerabilityDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rem_color, remediate_avail) = if self.fixed_version_display.is_some() {
            ("\x1b[32m", "yes") // Green
        } else {
            ("\x1b[31m", "no") // Red
        };

        let fixed_ver = self
            .fixed_version_display
            .as_deref()
            .unwrap_or("None available");
        let fixed_color = if self.fixed_version_display.is_some() {
            "\x1b[32m"
        } else {
            "\x1b[31m"
        };

        write!(
            f,
            "\x1b[1;31mVulnerability detected in supply chain\x1b[0m\n\n  \x1b[36m-\x1b[0m package: \x1b[1m{}\x1b[0m\n  \x1b[36m-\x1b[0m version: \x1b[33m{}\x1b[0m\n  \x1b[36m-\x1b[0m ecosystem: {}\n  \x1b[36m-\x1b[0m vulnerability: \x1b[1;31m{}\x1b[0m\n  \x1b[36m-\x1b[0m severity: \x1b[1;31mHigh\x1b[0m\n  \x1b[36m-\x1b[0m remediate available: {}{}\x1b[0m\n  \x1b[36m-\x1b[0m fixed version: {}{}\x1b[0m",
            self.package_name,
            self.version,
            self.ecosystem,
            self.cve_ids,
            rem_color,
            remediate_avail,
            fixed_color,
            fixed_ver
        )
    }
}

impl Diagnostic for VulnerabilityDiagnostic {
    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        let sniff_cmd = format!("\x1b[33mwoof sniff {}\x1b[0m", self.package_name);
        let rem_cmd = match &self.fixed_version_cmd {
            Some(v) => format!("\x1b[32mwoof remediate {} {}\x1b[0m", self.package_name, v),
            None => "\x1b[31mManual lockfile update required\x1b[0m".to_string(),
        };

        let help_text = format!(
            "\x1b[1;33mwarn:\x1b[0m Upgrading may break peer dependencies or parent requirements.\n      Analyze dependency paths first.\n      Command: {}\n\n\x1b[1;32mhelp:\x1b[0m Apply the fixed version:\n      Command: {}",
            sniff_cmd, rem_cmd
        );
        Some(Box::new(help_text))
    }
}

pub fn batch_query_osv(dependencies: &[(String, String, String)]) -> Result<bool> {
    if dependencies.is_empty() {
        return Ok(true);
    }

    let mut hasher = DefaultHasher::new();
    dependencies.hash(&mut hasher);
    let cache_key = hasher.finish();

    // Local OSV Caching (12-hour TTL)
    let cache_dir = env::temp_dir().join("woof_osv_cache");
    let _ = fs::create_dir_all(&cache_dir);
    let cache_file = cache_dir.join(format!("osv_results_{}.json", cache_key));

    let mut response_json: Option<serde_json::Value> = None;

    if cache_file.exists()
        && let Ok(metadata) = fs::metadata(&cache_file)
        && let Ok(modified) = metadata.modified()
        && let Ok(duration) = SystemTime::now().duration_since(modified)
        && duration.as_secs() < 43200
    {
        // 12 hours
        if let Ok(cached_data) = fs::read_to_string(&cache_file) {
            response_json = serde_json::from_str(&cached_data).ok();
        }
    }

    // Network Fallback
    let response_json = match response_json {
        Some(json) => {
            println!("[INFO] Threat intelligence loaded from local cache.");
            json
        }
        None => {
            let client = Client::new();
            let queries: Vec<_> = dependencies
                .iter()
                .map(|(name, version, ecosystem)| {
                    json!({ "version": version, "package": { "name": name, "ecosystem": ecosystem } })
                })
                .collect();

            let response = client
                .post("https://api.osv.dev/v1/querybatch")
                .json(&json!({ "queries": queries }))
                .send()
                .into_diagnostic()?;

            let json: serde_json::Value = response.json().into_diagnostic()?;

            if let Ok(json_string) = serde_json::to_string(&json) {
                let _ = fs::write(&cache_file, json_string);
            }
            json
        }
    };

    let mut is_clean = true;

    if let Some(results) = response_json.get("results").and_then(|r| r.as_array()) {
        for (index, result) in results.iter().enumerate() {
            // Collapse the `is_empty` check into the `if let`
            if let Some(vulns) = result.get("vulns").and_then(|v| v.as_array())
                && !vulns.is_empty()
            {
                is_clean = false;
                let (pkg, ver, eco) = &dependencies[index];

                // Batch query only returns IDs. Fetch full details for remediation.
                let client = Client::new();
                let full_vuln_data = client
                    .post("https://api.osv.dev/v1/query")
                    .json(&json!({ "version": ver, "package": { "name": pkg, "ecosystem": eco } }))
                    .send()
                    .ok()
                    .and_then(|r| r.json::<serde_json::Value>().ok());

                let mut all_aliases = Vec::new();
                let mut fixed_versions = Vec::new();

                // Explicitly get the list of vulnerabilities from either the full query or the batch result
                let vulns_list = full_vuln_data
                    .as_ref()
                    .and_then(|j| j.get("vulns").and_then(|v| v.as_array()))
                    .unwrap_or(vulns);

                for v in vulns_list {
                    if let Some(id) = v.get("id").and_then(|id| id.as_str()) {
                        all_aliases.push(id.to_string());
                    }
                    if let Some(aliases) = v.get("aliases").and_then(|a| a.as_array()) {
                        for alias in aliases.iter().filter_map(|id| id.as_str()) {
                            all_aliases.push(alias.to_string());
                        }
                    }

                    if let Some(affected_arr) = v.get("affected").and_then(|a| a.as_array()) {
                        for affected in affected_arr {
                            if affected
                                .get("package")
                                .and_then(|p| p.get("name"))
                                .and_then(|n| n.as_str())
                                == Some(pkg)
                                && let Some(ranges) =
                                    affected.get("ranges").and_then(|r| r.as_array())
                            {
                                for range in ranges {
                                    if let Some(events) =
                                        range.get("events").and_then(|e| e.as_array())
                                    {
                                        for event in events {
                                            if let Some(fixed) =
                                                event.get("fixed").and_then(|f| f.as_str())
                                            {
                                                fixed_versions.push(fixed.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                all_aliases.sort();
                all_aliases.dedup();
                fixed_versions.sort();
                fixed_versions.dedup();

                let cve_string = if all_aliases.is_empty() {
                    "Unknown Threat ID".to_string()
                } else {
                    all_aliases.join(", ")
                };

                let (fixed_version_display, fixed_version_cmd) = if fixed_versions.is_empty() {
                    (None, None)
                } else {
                    // Safely extract major version ignoring npm prefixes like '^' or '~'
                    let user_major = ver
                        .split(|c: char| !c.is_numeric())
                        .find(|s| !s.is_empty())
                        .unwrap_or("");
                    let mut best_cmd = None;

                    for f in &fixed_versions {
                        let f_major = f
                            .split(|c: char| !c.is_numeric())
                            .find(|s| !s.is_empty())
                            .unwrap_or("");
                        if f_major == user_major {
                            best_cmd = Some(f.clone());
                            break;
                        }
                    }

                    // Fallback to the highest available version if branch matching fails
                    if best_cmd.is_none() {
                        best_cmd = Some(fixed_versions.last().unwrap().clone());
                    }

                    (Some(fixed_versions.join(", ")), best_cmd)
                };

                println!(
                    "{:?}",
                    Report::new(VulnerabilityDiagnostic {
                        package_name: pkg.clone(),
                        version: ver.clone(),
                        ecosystem: eco.clone(),
                        cve_ids: cve_string,
                        fixed_version_display,
                        fixed_version_cmd,
                    })
                );
            }
        }
    }
    Ok(is_clean)
}
