// src/scanner/mod.rs

pub mod env_guard;

use crate::detector::entropy;
use crate::detector::rules::{CORE_RULES, RULE_MATCHER};
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use memmap2::MmapOptions;
use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use std::collections::VecDeque;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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

    if !config.ignore_paths.is_empty() {
        let mut overrides = OverrideBuilder::new(target_path);
        for path in config.ignore_paths {
            // Prepend "!" to the path to tell the builder to ignore it
            let _ = overrides.add(&format!("!{}", path));
        }
        if let Ok(ov) = overrides.build() {
            builder.overrides(ov);
        } else {
            println!("[WARN] Failed to compile ignore overrides from .woof.toml");
        }
    }

    let walker = builder.build_parallel();
    let scanned_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel::<SecurityDiagnostic>();

    // Thread-safe rolling queue to hold the last 4 scanned files
    let recent_files: Arc<Mutex<VecDeque<String>>> =
        Arc::new(Mutex::new(VecDeque::with_capacity(4)));

    // 1. Initialize the Progress Bar only if we are not in a CI environment
    let spinner = if !is_ci {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(80));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"]),
        );
        pb.set_message(format!(
            "Initializing perimeter sweep on {}...",
            target_path
        ));
        Some(pb)
    } else {
        None
    };

    walker.run(|| {
        let scanned_count = scanned_count.clone();
        let tx = tx.clone();

        // 2. Clone the spinner and queue references so they can be safely moved into multiple worker threads
        let worker_spinner = spinner.clone();
        let worker_recent = recent_files.clone();

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

                let file = match File::open(entry.path()) {
                    Ok(f) => f,
                    Err(_) => return ignore::WalkState::Continue,
                };

                let mmap = match unsafe { MmapOptions::new().map(&file) } {
                    Ok(m) => m,
                    Err(_) => return ignore::WalkState::Continue,
                };

                // Binary Fast-Fail Heuristic
                let check_len = std::cmp::min(mmap.len(), 512);
                if mmap[..check_len].contains(&0) {
                    return ignore::WalkState::Continue;
                } // Ensure the file is valid UTF-8 before running the rules
                if let Ok(content) = std::str::from_utf8(&mmap) {
                    scanned_count.fetch_add(1, Ordering::Relaxed);

                    let matches = RULE_MATCHER.find_iter(content);

                    for mat in matches {
                        let rule = &CORE_RULES[mat.pattern().as_usize()];

                        if let Some(regex_match) = rule.pattern.find(&content[mat.start()..]) {
                            let absolute_start = mat.start() + regex_match.start();
                            let length = regex_match.end() - regex_match.start();
                            let matched_secret = regex_match.as_str();

                            let entropy_score =
                                entropy::calculate_shannon_entropy(matched_secret.as_bytes());

                            if entropy_score < config.min_entropy {
                                continue;
                            }

                            let mut safe_content = content.to_string();
                            let redaction = "*".repeat(length);
                            safe_content.replace_range(
                                absolute_start..(absolute_start + length),
                                &redaction,
                            );

                            let enriched_remediation = format!(
                                "{} (Calculated Entropy Score: {:.2})",
                                rule.remediation, entropy_score
                            );

                            let diagnostic = SecurityDiagnostic {
                                asset_type: rule.name.to_string(),
                                err_code: rule.error_code.to_string(),
                                remediation: enriched_remediation,
                                src: NamedSource::new(
                                    entry.path().display().to_string(),
                                    safe_content,
                                ),
                                err_span: (absolute_start, length).into(),
                            };

                            let _ = tx.send(diagnostic);
                        }
                    }

                    // 3. Dynamically update the rolling window of the last 4 scanned files
                    if let Some(pb) = &worker_spinner {
                        let mut recent = worker_recent.lock().unwrap();

                        // Keep the window at exactly 4 items to prevent UI tearing
                        if recent.len() >= 4 {
                            recent.pop_front();
                        }

                        // \x1b[32m✓\x1b[0m is the standard ANSI escape code for a green tick
                        recent.push_back(format!("\x1b[32m✓\x1b[0m {}", entry.path().display()));

                        // Join the active queue with newlines and indentation for a clean UI block
                        let display_text = recent.iter().cloned().collect::<Vec<_>>().join("\n  ");
                        pb.set_message(format!("Analyzing perimeter...\n  {}", display_text));
                    }
                }
            }
            ignore::WalkState::Continue
        })
    });

    drop(tx);

    let total_files = scanned_count.load(Ordering::Relaxed);
    let all_findings: Vec<SecurityDiagnostic> = rx.into_iter().collect();

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
        // 4. Terminate the spinner gracefully, replacing the multiline block with a single final summary
        if let Some(pb) = spinner {
            pb.finish_and_clear();
        }
        println!("\n[INFO] Sweep complete. Analyzed {} files.", total_files);

        if all_findings.is_empty() {
            println!("\x1b[32m✓\x1b[0m [INFO] Status: SECURE. No cryptographic assets exposed.");
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
