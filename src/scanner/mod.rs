// src/scanner/mod.rs

pub mod env_guard;

use crate::detector::entropy;
use crate::detector::rules::{CORE_RULES, RULE_MATCHER};
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use memmap2::MmapOptions;
use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use std::collections::HashMap;
use std::fs::File;
use std::path::Component;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::Instant;
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
                self.err_span,
            ),
        )))
    }
}

/// Executes a multi-threaded perimeter sweep of the target directory.
pub fn execute_sweep(target_path: &str, is_ci: bool) -> bool {
    let start_time = Instant::now();

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

    // ==========================================
    // 1. PRE-FLIGHT INDEXING (Lightning Fast)
    // ==========================================
    let mut total_bytes = 0u64;
    let mut expected_files = 0u64;
    let mut folder_sizes: HashMap<String, u64> = HashMap::new();

    if !is_ci {
        println!("[INFO] Indexing perimeter...");
        let index_walker = builder.build(); // Synchronous walker just for metadata

        for entry in index_walker.flatten() {
            if entry.file_type().is_some_and(|ft| ft.is_file())
                && let Ok(metadata) = entry.metadata()
            {
                let size = metadata.len();

                // Skip massive files in our total calculation
                if size > MAX_FILE_SIZE_BYTES {
                    continue;
                }

                total_bytes += size;
                expected_files += 1;

                // Grab the top-level folder name to accumulate sizes
                if let Some(Component::Normal(folder)) = entry.path().components().next() {
                    let folder_name = folder.to_string_lossy().to_string();
                    *folder_sizes.entry(folder_name).or_insert(0) += size;
                }
            }
        }

        // Sort folders highest to lowest byte count
        let mut sorted_folders: Vec<_> = folder_sizes.into_iter().collect();
        sorted_folders.sort_by_key(|b| std::cmp::Reverse(b.1));

        // Format the top 3 heaviest folders
        let top_folders = sorted_folders
            .into_iter()
            .take(3)
            .map(|(name, size)| format!("{}/ ({:.1} MiB)", name, size as f64 / 1_048_576.0))
            .collect::<Vec<_>>()
            .join(" | ");

        if !top_folders.is_empty() {
            println!("[INFO] Heaviest targets: {}", top_folders);
        }
    }

    let walker = builder.build_parallel();
    let scanned_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel::<SecurityDiagnostic>();

    // 2. Initialize the True Progress Bar using the UI module
    let spinner = crate::ui::build_scanner_pb(total_bytes, is_ci);

    walker.run(|| {
        let scanned_count = scanned_count.clone();
        let tx = tx.clone();

        // 3. Clone the spinner reference so it can be safely moved into multiple worker threads
        let worker_spinner = spinner.clone();

        Box::new(move |result| {
            if let Ok(entry) = result {
                if !entry.file_type().is_some_and(|ft| ft.is_file()) {
                    return ignore::WalkState::Continue;
                }

                // Memory Safety: Skip massive files
                if let Ok(metadata) = entry.metadata()
                    && metadata.len() > MAX_FILE_SIZE_BYTES
                {
                    return ignore::WalkState::Continue;
                }

                let file = match File::open(entry.path()) {
                    Ok(f) => f,
                    Err(_) => return ignore::WalkState::Continue,
                };

                let mmap = match unsafe { MmapOptions::new().map(&file) } {
                    Ok(m) => m,
                    Err(_) => return ignore::WalkState::Continue,
                };

                // Update metrics as soon as the file is safely loaded into memory
                let current_file_count = scanned_count.fetch_add(1, Ordering::Relaxed) + 1;
                if let Some(pb) = &worker_spinner {
                    pb.inc(mmap.len() as u64);
                    pb.set_message(format!("{}/{} files", current_file_count, expected_files));
                }

                // Binary Fast-Fail Heuristic
                let check_len = std::cmp::min(mmap.len(), 512);
                if mmap[..check_len].contains(&0) {
                    return ignore::WalkState::Continue;
                }

                // Ensure the file is valid UTF-8 before running the rules
                if let Ok(content) = std::str::from_utf8(&mmap) {
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
                }
            }
            ignore::WalkState::Continue
        })
    });

    drop(tx);

    let total_files = scanned_count.load(Ordering::Relaxed);
    let all_findings: Vec<SecurityDiagnostic> = rx.into_iter().collect();

    let duration = start_time.elapsed();
    let time_str = if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        format!("{}ms", duration.as_millis())
    };

    if is_ci {
        if all_findings.is_empty() {
            println!(
                r#"{{"status": "success", "files_scanned": {}, "threats": 0, "time": "{}"}}"#,
                total_files, time_str
            );
        } else {
            println!(
                r#"{{"status": "failure", "files_scanned": {}, "threats": {}, "time": "{}"}}"#,
                total_files,
                all_findings.len(),
                time_str
            );
        }
    } else {
        // 4. Terminate the spinner gracefully, leaving it pinned to the terminal
        if let Some(pb) = spinner {
            pb.finish();
        }
        println!(
            "\n[INFO] Sweep complete. Analyzed {} files in {}.",
            total_files, time_str
        );

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
                        err_span: finding.err_span,
                    })
                );
            }
        }
    }

    all_findings.is_empty()
}
