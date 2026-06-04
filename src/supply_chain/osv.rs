// src/supply_chain/osv.rs

use miette::{Diagnostic, IntoDiagnostic, Report, Result};
use reqwest::blocking::Client;
use serde_json::json;
use std::time::SystemTime;
use std::{env, fs};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("[CRITICAL] Compromised Dependency: {package_name}@{version}")]
pub struct VulnerabilityDiagnostic {
    pub package_name: String,
    pub version: String,
    pub ecosystem: String,
    pub cve_ids: String,
    pub summary: String,
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
        Some(Box::new(format!(
            "Threat: {}\nAction: {}",
            self.summary, self.remediation
        )))
    }
}

pub fn batch_query_osv(dependencies: &[(String, String, String)]) -> Result<bool> {
    if dependencies.is_empty() {
        return Ok(true);
    }

    // Local OSV Caching (12-hour TTL)
    let cache_dir = env::temp_dir().join("woof_osv_cache");
    let _ = fs::create_dir_all(&cache_dir);
    let cache_file = cache_dir.join("osv_results.json");

    let mut response_json: Option<serde_json::Value> = None;

    if cache_file.exists() {
        if let Ok(metadata) = fs::metadata(&cache_file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = SystemTime::now().duration_since(modified) {
                    if duration.as_secs() < 43200 {
                        // 12 hours
                        if let Ok(cached_data) = fs::read_to_string(&cache_file) {
                            response_json = serde_json::from_str(&cached_data).ok();
                        }
                    }
                }
            }
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
            let queries: Vec<_> = dependencies.iter().map(|(name, version, ecosystem)| {
                json!({ "version": version, "package": { "name": name, "ecosystem": ecosystem } })
            }).collect();

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
            if let Some(vulns) = result.get("vulns").and_then(|v| v.as_array()) {
                if !vulns.is_empty() {
                    is_clean = false;
                    let (pkg, ver, eco) = &dependencies[index];

                    let mut all_aliases = Vec::new();
                    let mut threat_summary = String::new();

                    for v in vulns {
                        if threat_summary.is_empty() {
                            if let Some(summary) = v.get("summary").and_then(|s| s.as_str()) {
                                threat_summary = summary.to_string();
                            } else if let Some(details) = v.get("details").and_then(|d| d.as_str())
                            {
                                threat_summary =
                                    format!("{}...", details.chars().take(120).collect::<String>());
                            }
                        }

                        if let Some(id) = v.get("id").and_then(|id| id.as_str()) {
                            all_aliases.push(id);
                        }

                        if let Some(aliases) = v.get("aliases").and_then(|a| a.as_array()) {
                            for alias in aliases.iter().filter_map(|id| id.as_str()) {
                                all_aliases.push(alias);
                            }
                        }
                    }

                    all_aliases.sort();
                    all_aliases.dedup();

                    let cve_string = if all_aliases.is_empty() {
                        "Unknown Threat ID".to_string()
                    } else {
                        all_aliases.join(", ")
                    };

                    let final_summary = if threat_summary.is_empty() {
                        "No detailed summary provided by OSV database.".to_string()
                    } else {
                        threat_summary.replace('\n', " ")
                    };

                    println!(
                        "{:?}",
                        Report::new(VulnerabilityDiagnostic {
                            package_name: pkg.clone(),
                            version: ver.clone(),
                            ecosystem: eco.clone(),
                            cve_ids: cve_string.clone(),
                            summary: final_summary,
                            remediation: format!(
                                "Run `woof remediate {} <secure_version>` or update lockfile.\nIdentifiers: {}",
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
