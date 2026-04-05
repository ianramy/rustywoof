// src/scanner/mod.rs

pub mod env_guard;

use crate::detector::entropy;
use crate::detector::rules::{CORE_RULES, RULE_MATCHER};
use ignore::WalkBuilder;
use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Maximum allowable file size to scan (5 Megabytes).
/// Prevents Out-Of-Memory (OOM) crashes on large artifacts like database dumps.
const MAX_FILE_SIZE_BYTES: u64 = 5 * 1024 * 1024;

#[derive(Error, Debug)]
#[error("[CRITICAL] Compromised Asset Detected: {asset_type}")]
pub struct SecurityDiagnostic {
    pub asset_type: String,
    pub err_code: String,
    pub remediation: String,
    pub src: NamedSource<String>,
    pub err_span: SourceSpan,
}

impl Diagnostic for SecurityDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(self.err_code.clone()))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(self.remediation.clone()))
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.src)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(std::iter::once(
            miette::LabeledSpan::new_with_span(
                Some("Exposure found here".to_string()),
                self.err_span.clone(),
            ),
        )))
    }
}

/// Executes a multi-threaded perimeter sweep of the target directory.
pub fn execute_sweep(target_path: &str, is_ci: bool) -> bool {
    // Ensure the .env perimeter is secure before we begin file traversal
    if let Err(e) = env_guard::secure_perimeter() {
        println!("{:?}", e);
    }

    // Handle the config load gracefully
    let config = match crate::config::load_config() {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);
            crate::config::Config::default()
        }
    };
    let mut builder = WalkBuilder::new(target_path);

    builder
        .hidden(false)
        .filter_entry(|e| e.file_name() != ".git")
        .ignore(false);
    for path in config.ignore_paths {
        builder.add_ignore(path);
    }

    let walker = builder.build_parallel();
    let scanned_count = Arc::new(AtomicUsize::new(0));
    let diagnostics: Arc<Mutex<Vec<SecurityDiagnostic>>> = Arc::new(Mutex::new(Vec::new()));

    if !is_ci {
        println!(
            "[INFO] Watchdog perimeter sweep initiated on directory: {}",
            target_path
        );
    }

    walker.run(|| {
        let scanned_count = scanned_count.clone();
        let diagnostics = diagnostics.clone();

        Box::new(move |result| {
            if let Ok(entry) = result {
                if !entry.file_type().map_or(false, |ft| ft.is_file()) {
                    return ignore::WalkState::Continue;
                }

                // Memory Safety: Skip massive files
                if let Ok(metadata) = entry.metadata() {
                    if metadata.len() > MAX_FILE_SIZE_BYTES {
                        return ignore::WalkState::Continue;
                    }
                }

                // If read_to_string fails, it's likely a binary file. Gracefully skip.
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    scanned_count.fetch_add(1, Ordering::Relaxed);

                    // High-Speed Filter: Scan for all prefixes simultaneously in O(n) time
                    let matches = RULE_MATCHER.find_iter(&content);

                    for mat in matches {
                        let rule = &CORE_RULES[mat.pattern().as_usize()];

                        if let Some(regex_match) = rule.pattern.find(&content[mat.start()..]) {
                            let absolute_start = mat.start() + regex_match.start();
                            let length = regex_match.end() - regex_match.start();
                            let matched_secret = regex_match.as_str();

                            // Use our orphaned entropy function!
                            let entropy_score = entropy::calculate_shannon_entropy(matched_secret.as_bytes());

                            let mut safe_content = content.clone();
                            let redaction = "*".repeat(length);
                            safe_content.replace_range(absolute_start..(absolute_start + length), &redaction);

                            // Inject the Entropy Score into the remediation text
                            let enriched_remediation = format!(
                                "{} (Calculated Entropy Score: {:.2})",
                                rule.remediation,
                                entropy_score
                            );

                            let diagnostic = SecurityDiagnostic {
                                asset_type: rule.name.to_string(),
                                err_code: rule.error_code.to_string(),
                                remediation: enriched_remediation, // <-- Used here!
                                src: NamedSource::new(entry.path().display().to_string(), safe_content),
                                err_span: (absolute_start, length).into(),
                            };

                            let mut lock = diagnostics.lock().unwrap();
                            lock.push(diagnostic);
                        }
                    }
                }
            }
            ignore::WalkState::Continue
        })
    });

    let total_files = scanned_count.load(Ordering::Relaxed);
    let all_findings = diagnostics.lock().unwrap();

    if is_ci {
        if all_findings.is_empty() {
            println!(
                r#"{{"status": "success", "files_scanned": {}, "threats": 0}}"#,
                total_files
            );
        } else {
            println!(
                r#"{{"status": "failure", "files_scanned": {}, "threats": {}}}"#,
                total_files,
                all_findings.len()
            );
        }
    } else {
        println!("[INFO] Sweep complete. Analyzed {} artifacts.", total_files);
        if all_findings.is_empty() {
            println!("[INFO] Status: SECURE. No cryptographic assets exposed.");
        } else {
            println!(
                "\n[CRITICAL] Perimeter breached! Found {} exposed assets.",
                all_findings.len()
            );
            for finding in all_findings.iter() {
                println!(
                    "{:?}",
                    Report::new(SecurityDiagnostic {
                        asset_type: finding.asset_type.clone(),
                        err_code: finding.err_code.clone(),
                        remediation: finding.remediation.clone(),
                        src: finding.src.clone(),
                        err_span: finding.err_span.clone(),
                    })
                );
            }
        }
    }

    all_findings.is_empty()
}
