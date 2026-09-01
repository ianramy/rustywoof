// src/scanner/execute_file_sweep.rs

use crate::detector::calculate_shannon_entropy;
use crate::detector::execute_heuristic_automaton;
use crate::detector::suppress_false_positives;
use crate::scanner::index_target_perimeters::MAX_FILE_SIZE_BYTES;
use ignore::WalkBuilder;
use indicatif::ProgressBar;
use std::fs::File;
use std::sync::mpsc;

pub struct SweepFinding {
    pub file_path: String,
    pub asset_type: String,
    pub matched_text: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub error_code: String,
    pub remediation: String,
    pub entropy: f32,
}

pub fn sweep_directory(
    target_path: &str,
    ignore_paths: &[String],
    pb: ProgressBar,
) -> Vec<SweepFinding> {
    let mut builder = WalkBuilder::new(target_path);
    builder
        .hidden(false)
        .filter_entry(|e| e.file_name() != ".git")
        .ignore(false);

    if !ignore_paths.is_empty() {
        let mut overrides = ignore::overrides::OverrideBuilder::new(target_path);
        for path in ignore_paths {
            let _ = overrides.add(&format!("!{}", path));
        }
        if let Ok(ov) = overrides.build() {
            builder.overrides(ov);
        }
    }

    let walker = builder.build_parallel();
    let (tx, rx) = mpsc::channel::<SweepFinding>();

    walker.run(|| {
        let tx = tx.clone();
        let pb = pb.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                if !entry.file_type().is_some_and(|ft| ft.is_file()) {
                    return ignore::WalkState::Continue;
                }

                let file = match File::open(entry.path()) {
                    Ok(f) => f,
                    Err(_) => return ignore::WalkState::Continue,
                };

                let metadata = match file.metadata() {
                    Ok(m) => m,
                    Err(_) => return ignore::WalkState::Continue,
                };

                if !metadata.is_file() {
                    return ignore::WalkState::Continue;
                }

                let file_size = metadata.len();
                if file_size > MAX_FILE_SIZE_BYTES {
                    return ignore::WalkState::Continue;
                }

                use std::io::Read;
                let mut buf = Vec::with_capacity(file_size as usize);
                if (&file).take(file_size).read_to_end(&mut buf).is_err() {
                    pb.inc(file_size);
                    return ignore::WalkState::Continue;
                }

                let check_len = std::cmp::min(buf.len(), 512);
                if buf[..check_len].contains(&0) {
                    pb.inc(file_size);
                    return ignore::WalkState::Continue;
                }

                if let Ok(content) = std::str::from_utf8(&buf) {
                    let findings = execute_heuristic_automaton::scan_buffer(content);

                    for finding in findings {
                        if !suppress_false_positives::is_false_positive(&finding, content) {
                            let entropy = calculate_shannon_entropy::calculate_entropy(
                                finding.matched_text.as_bytes(),
                            );

                            let _ = tx.send(SweepFinding {
                                file_path: entry.path().display().to_string(),
                                asset_type: finding.rule.name.to_string(),
                                matched_text: finding.matched_text.clone(),
                                start_offset: finding.start_offset,
                                end_offset: finding.end_offset,
                                error_code: finding.rule.error_code.to_string(),
                                remediation: finding.rule.remediation.to_string(),
                                entropy,
                            });
                        }
                    }
                }

                pb.inc(file_size);
            }
            ignore::WalkState::Continue
        })
    });

    pb.finish();
    drop(tx);
    rx.into_iter().collect()
}
