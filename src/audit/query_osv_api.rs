// src/audit/query_osv_api.rs

use crate::audit::format_vulnerability_diagnostics::VulnerabilityDiagnostic;
use crate::audit::manage_osv_cache;
use miette::{IntoDiagnostic, Result};
use reqwest::blocking::Client;
use serde_json::json;
use std::str::FromStr;

fn map_score_to_severity(score: f64) -> &'static str {
    if score >= 9.0 {
        "CRITICAL"
    } else if score >= 7.0 {
        "HIGH"
    } else if score >= 4.0 {
        "MEDIUM"
    } else if score >= 0.1 {
        "LOW"
    } else {
        "UNKNOWN"
    }
}

fn severity_value(sev: &str) -> u8 {
    match sev.to_uppercase().as_str() {
        "CRITICAL" => 4,
        "HIGH" => 3,
        "MEDIUM" | "MODERATE" => 2,
        "LOW" => 1,
        _ => 0,
    }
}

fn extract_aliases(v: &serde_json::Value) -> Vec<String> {
    let mut all_aliases = Vec::new();
    if let Some(id) = v.get("id").and_then(|id| id.as_str()) {
        all_aliases.push(id.to_string());
    }
    let Some(aliases) = v.get("aliases").and_then(|a| a.as_array()) else {
        return all_aliases;
    };
    for alias in aliases.iter().filter_map(|id| id.as_str()) {
        all_aliases.push(alias.to_string());
    }
    all_aliases
}

fn extract_fixed_versions(v: &serde_json::Value, pkg: &str) -> Vec<String> {
    let mut fixed_versions = Vec::new();
    let Some(affected_arr) = v.get("affected").and_then(|a| a.as_array()) else {
        return fixed_versions;
    };

    for affected in affected_arr {
        let is_target = affected
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            == Some(pkg);
        if !is_target {
            continue;
        }
        let Some(ranges) = affected.get("ranges").and_then(|r| r.as_array()) else {
            continue;
        };

        for range in ranges {
            let Some(events) = range.get("events").and_then(|e| e.as_array()) else {
                continue;
            };
            for event in events {
                if let Some(fixed) = event.get("fixed").and_then(|f| f.as_str()) {
                    fixed_versions.push(fixed.to_string());
                }
            }
        }
    }
    fixed_versions
}

fn parse_vulnerability(
    v: &serde_json::Value,
    pkg: &str,
    ver: &str,
    eco: &str,
    audit_level: Option<&String>,
    base_url: &str,
) -> Option<VulnerabilityDiagnostic> {
    let mut published_date = "Unknown".to_string();
    if let Some(pub_date) = v.get("published").and_then(|p| p.as_str()) {
        published_date = pub_date.split('T').next().unwrap_or(pub_date).to_string();
    }

    let all_aliases = extract_aliases(v);

    let extract_score = |vuln: &serde_json::Value| -> Option<f64> {
        let mut local_max: Option<f64> = None;
        if let Some(sev_arr) = vuln.get("severity").and_then(|s| s.as_array()) {
            for sev in sev_arr {
                if let Some(v_str) = sev.get("score").and_then(|s| s.as_str()) {
                    if let Ok(parsed_v4) = cvss::v4::Vector::from_str(v_str) {
                        local_max = Some(local_max.unwrap_or(0.0).max(parsed_v4.score().value()));
                    } else if let Ok(parsed_v3) = cvss::v3::Base::from_str(v_str) {
                        local_max = Some(local_max.unwrap_or(0.0).max(parsed_v3.score().value()));
                    }
                }
            }
        }
        local_max
    };

    let mut max_numeric_score = extract_score(v);

    if max_numeric_score.is_none() {
        let client = Client::new();
        for alias in &all_aliases {
            if alias.starts_with("CVE-") {
                let alias_url = format!("{}/vulns/{}", base_url, alias);
                if let Ok(resp) = client.get(&alias_url).send()
                    && let Ok(alias_v) = resp.json::<serde_json::Value>()
                    && let Some(score) = extract_score(&alias_v)
                {
                    max_numeric_score = Some(score);
                    break;
                }
            }
        }
    }

    let mut highest_severity = if let Some(score) = max_numeric_score {
        map_score_to_severity(score).to_string()
    } else {
        "UNKNOWN".to_string()
    };

    if highest_severity == "UNKNOWN" {
        let db_sev = v
            .get("database_specific")
            .and_then(|d| d.get("severity"))
            .and_then(|s| s.as_str());
        if let Some(sev) = db_sev {
            highest_severity = match sev.to_lowercase().as_str() {
                s if s.contains("critical") => "CRITICAL".to_string(),
                s if s.contains("high") => "HIGH".to_string(),
                s if s.contains("medium") || s.contains("moderate") => "MEDIUM".to_string(),
                s if s.contains("low") => "LOW".to_string(),
                _ => "UNKNOWN".to_string(),
            };
        }
    }

    if let Some(target_level) = audit_level {
        let target_lower = target_level.to_lowercase();
        let current_lower = highest_severity.to_lowercase();
        if current_lower != "unknown" && !current_lower.contains(&target_lower) {
            return None;
        }
    }

    let mut fixed_versions = extract_fixed_versions(v, pkg);
    fixed_versions.sort();
    fixed_versions.dedup();

    let (fixed_version_display, fixed_version_cmd) = if fixed_versions.is_empty() {
        (None, None)
    } else {
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
        if best_cmd.is_none() {
            best_cmd = Some(fixed_versions.last().unwrap().clone());
        }
        (Some(fixed_versions.join(", ")), best_cmd)
    };

    Some(VulnerabilityDiagnostic {
        package_name: pkg.to_string(),
        version: ver.to_string(),
        ecosystem: eco.to_string(),
        cve_ids: all_aliases,
        severity_level: highest_severity,
        numeric_score: max_numeric_score,
        published_date,
        fixed_version_display,
        fixed_version_cmd,
    })
}

fn merge_diagnostics(diagnostics: Vec<VulnerabilityDiagnostic>) -> Vec<VulnerabilityDiagnostic> {
    let mut merged: Vec<VulnerabilityDiagnostic> = Vec::new();

    for diag in diagnostics {
        let mut found = false;
        for m in &mut merged {
            if m.package_name == diag.package_name
                && m.version == diag.version
                && m.ecosystem == diag.ecosystem
            {
                for id in &diag.cve_ids {
                    if !m.cve_ids.contains(id) {
                        m.cve_ids.push(id.clone());
                    }
                }

                let m_val = severity_value(&m.severity_level);
                let d_val = severity_value(&diag.severity_level);

                if d_val > m_val {
                    m.severity_level = diag.severity_level.clone();
                }

                if let Some(d_score) = diag.numeric_score {
                    if let Some(m_score) = m.numeric_score {
                        if d_score > m_score {
                            m.numeric_score = Some(d_score);
                        }
                    } else {
                        m.numeric_score = Some(d_score);
                    }
                }

                if m.fixed_version_cmd.is_none() && diag.fixed_version_cmd.is_some() {
                    m.fixed_version_cmd = diag.fixed_version_cmd.clone();
                    m.fixed_version_display = diag.fixed_version_display.clone();
                }

                found = true;
                break;
            }
        }

        if !found {
            merged.push(diag);
        }
    }

    for m in &mut merged {
        m.cve_ids.sort_by(|a, b| {
            let a_score = if a.starts_with("CVE-") {
                0
            } else if a.starts_with("GHSA-") {
                1
            } else if a.starts_with("PYSEC-") || a.starts_with("RUSTSEC-") {
                2
            } else {
                3
            };
            let b_score = if b.starts_with("CVE-") {
                0
            } else if b.starts_with("GHSA-") {
                1
            } else if b.starts_with("PYSEC-") || b.starts_with("RUSTSEC-") {
                2
            } else {
                3
            };
            a_score.cmp(&b_score).then(a.cmp(b))
        });
        m.cve_ids.dedup();
    }

    merged
}

pub fn batch_query_osv(
    dependencies: &[(String, String, String)],
    base_url_override: Option<&str>,
    _dev: bool,
    _prod: bool,
    audit_level: Option<String>,
    _interactive: bool,
) -> Result<bool> {
    if dependencies.is_empty() {
        return Ok(true);
    }

    let base_url = base_url_override.unwrap_or("https://api.osv.dev/v1");
    let orchestrator =
        crate::ui::orchestrate_progress_bars::ProgressOrchestrator::new(!_interactive);
    let spinner = orchestrator.add_spinner();
    spinner.set_message("Hashing dependencies for cache lookup...");

    let cache_key = manage_osv_cache::generate_cache_key(dependencies);
    let cache_file = manage_osv_cache::get_cache_path(cache_key);
    let mut response_json: Option<serde_json::Value> = None;

    if let Some(cached_data) = manage_osv_cache::read_cache_if_valid(&cache_file, 43200) {
        spinner.set_message("Reading local threat cache...");
        response_json = serde_json::from_str(&cached_data).ok();
    }

    let response_json = match response_json {
        Some(json) => {
            spinner.suspend(|| {
                println!("\x1b[34mℹ\x1b[0m \x1b[90mThreat intelligence loaded from local cache.\x1b[0m");
                println!("  \x1b[90mHint: Run `woof cache clean` to force a real-time OSV database sync.\x1b[0m\n");
            });
            spinner.set_message("Parsing cached database response...");
            json
        }
        None => {
            spinner.set_message("Querying remote OSV database...");
            let client = Client::new();
            let queries: Vec<_> = dependencies
                .iter()
                .map(|(name, version, ecosystem)| {
                    json!({ "version": version, "package": { "name": name, "ecosystem": ecosystem } })
                })
                .collect();

            let querybatch_url = format!("{}/querybatch", base_url);
            let response = client
                .post(&querybatch_url)
                .json(&json!({ "queries": queries }))
                .send()
                .into_diagnostic()?;

            spinner.set_message("Parsing database response...");
            let json: serde_json::Value = response.json().into_diagnostic()?;
            if let Ok(json_string) = serde_json::to_string(&json) {
                let _ = manage_osv_cache::write_cache(&cache_file, &json_string);
            }
            json
        }
    };

    let mut is_clean = true;
    let mut total_vulns = 0;
    let mut counts = std::collections::HashMap::new();
    counts.insert("critical", 0);
    counts.insert("high", 0);
    counts.insert("medium", 0);
    counts.insert("low", 0);
    counts.insert("unknown", 0);

    let Some(results) = response_json.get("results").and_then(|r| r.as_array()) else {
        spinner.finish_with_message("Threat intelligence analysis complete.");
        return Ok(is_clean);
    };

    for (index, result) in results.iter().enumerate() {
        spinner.set_message(format!(
            "Cross-referencing package {}/{}...",
            index + 1,
            results.len()
        ));

        let Some(vulns) = result.get("vulns").and_then(|v| v.as_array()) else {
            continue;
        };
        if vulns.is_empty() {
            continue;
        }

        let (pkg, ver, eco) = &dependencies[index];
        spinner.set_message(format!("Fetching full remediation data for {}...", pkg));

        let client = Client::new();
        let query_url = format!("{}/query", base_url);
        let full_vuln_data = client
            .post(&query_url)
            .json(&json!({ "version": ver, "package": { "name": pkg, "ecosystem": eco } }))
            .send()
            .ok()
            .and_then(|r| r.json::<serde_json::Value>().ok());

        let vulns_list = full_vuln_data
            .as_ref()
            .and_then(|j| j.get("vulns").and_then(|v| v.as_array()))
            .unwrap_or(vulns);
        let mut raw_diagnostics = Vec::new();

        for v in vulns_list {
            if let Some(diag) =
                parse_vulnerability(v, pkg, ver, eco, audit_level.as_ref(), base_url)
            {
                raw_diagnostics.push(diag);
            }
        }

        let merged = merge_diagnostics(raw_diagnostics);

        for diagnostic in merged {
            is_clean = false;
            total_vulns += 1;

            let static_sev_key = match diagnostic.severity_level.to_lowercase().as_str() {
                "critical" => "critical",
                "high" => "high",
                "medium" | "moderate" => "medium",
                "low" => "low",
                _ => "unknown",
            };

            let counter = counts.entry(static_sev_key).or_insert(0);
            *counter += 1;

            spinner.suspend(|| {
                println!("{}", diagnostic);
                if let Some(help) = miette::Diagnostic::help(&diagnostic) {
                    println!(
                        "\n  \x1b[36mhelp:\x1b[0m {}\n",
                        help.to_string().replace('\n', "\n        ")
                    );
                }
            });
        }
    }

    spinner.finish_with_message("Threat intelligence analysis complete.");

    if !is_clean {
        let mut footer_parts = Vec::new();
        for sev in ["critical", "high", "medium", "low", "unknown"] {
            let cnt = *counts.get(sev).unwrap_or(&0);
            if cnt > 0 {
                let sev_color = match sev {
                    "critical" => "\x1b[1;31m",
                    "high" => "\x1b[38;5;208m",
                    "medium" => "\x1b[1;33m",
                    "low" => "\x1b[38;5;148m",
                    _ => "\x1b[1;90m",
                };
                footer_parts.push(format!("{}{cnt} {sev}\x1b[0m", sev_color));
            }
        }

        println!("\n{} vulnerabilities found", total_vulns);
        println!("Severity: {}", footer_parts.join(", "));
    }

    Ok(is_clean)
}
