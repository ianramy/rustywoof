// src/scanner/aggregate_security_diagnostics.rs

use crate::scanner::execute_file_sweep::SweepFinding;
use crate::ui::{detect_headless_environment, format_terminal_banners};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("\x1b[1;31m[CRITICAL]\x1b[0m Compromised Asset Detected: {asset_type}")]
struct FindingDiagnostic {
    asset_type: String,

    #[label("Exposure found here")]
    span: SourceSpan,

    #[diagnostic(code("{error_code}"))]
    error_code: String,

    #[diagnostic(help("{remediation} (Calculated Entropy Score: {entropy:.2})"))]
    remediation: String,

    entropy: f32,
}

/// Formats the scan results into a strictly structured JSON string for CI/CD ingestion.
pub fn format_json_report(findings: &[SweepFinding], files_scanned: u64) -> String {
    if findings.is_empty() {
        format!(
            r#"{{"status":"success","files_scanned":{},"threats":0}}"#,
            files_scanned
        )
    } else {
        format!(
            r#"{{"status":"failure","files_scanned":{},"threats":{}}}"#,
            files_scanned,
            findings.len()
        )
    }
}

fn mask_secret(content: &str, start: usize, end: usize) -> String {
    if start >= content.len() || end > content.len() || start >= end {
        return content.to_string();
    }
    let mut masked = String::with_capacity(content.len());
    masked.push_str(&content[..start]);
    masked.push_str(&"*".repeat(end - start));
    masked.push_str(&content[end..]);
    masked
}

pub fn print_terminal_report(findings: &[SweepFinding], files_scanned: u64) {
    let headless = detect_headless_environment::is_headless();

    if findings.is_empty() {
        let msg = format!(
            "Status: SECURE. No cryptographic assets exposed in {} files.",
            files_scanned
        );
        println!(
            "{}",
            format_terminal_banners::format_success(&msg, headless)
        );
    } else {
        let msg = format!(
            "Perimeter breached! Found {} exposed assets.",
            findings.len()
        );
        println!(
            "\n{}",
            format_terminal_banners::format_critical(&msg, headless)
        );

        for finding in findings {
            let raw_content = std::fs::read_to_string(&finding.file_path).unwrap_or_default();
            let masked_content =
                mask_secret(&raw_content, finding.start_offset, finding.end_offset);
            let length = finding.end_offset.saturating_sub(finding.start_offset);

            let diagnostic = FindingDiagnostic {
                asset_type: finding.asset_type.clone(),
                span: (finding.start_offset, length).into(),
                error_code: finding.error_code.clone(),
                remediation: finding.remediation.clone(),
                entropy: finding.entropy,
            };

            let report = miette::Report::new(diagnostic)
                .with_source_code(miette::NamedSource::new(&finding.file_path, masked_content));

            println!("{:?}", report);
        }
    }
}
