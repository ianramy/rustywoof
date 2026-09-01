// tests/scanner/test_execute_file_sweep.rs

use indicatif::ProgressBar;
use rustywoof::scanner::execute_file_sweep;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_mmap_sweep_detects_secrets() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.js");

    let secret_content = "const key = 'AKIAIOSFODNN7EXAMPLX';";
    fs::write(&file_path, secret_content).unwrap();

    let pb = ProgressBar::hidden();
    let findings = execute_file_sweep::sweep_directory(dir.path().to_str().unwrap(), &[], pb);

    assert_eq!(
        findings.len(),
        1,
        "The sweep should have detected exactly 1 secret."
    );
    assert_eq!(
        findings[0].asset_type, "AWS Access Key",
        "Failed to identify rule correctly."
    );
}
