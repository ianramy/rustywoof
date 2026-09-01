// tests/git/test_extract_staged_diffs.rs

use rustywoof::git::extract_staged_diffs;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_extracts_only_staged_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let status = Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(root)
        .status()
        .expect("Failed to execute git init");
    assert!(status.success(), "Git init failed");

    let staged_file = root.join("staged_secret.txt");
    fs::write(&staged_file, "super_secret_token_123").unwrap();
    Command::new("git")
        .args(["add", "staged_secret.txt"])
        .current_dir(root)
        .status()
        .expect("Failed to git add");

    let unstaged_file = root.join("unstaged_benign.txt");
    fs::write(&unstaged_file, "hello world").unwrap();

    let staged_files =
        extract_staged_diffs::get_staged_files(root).expect("Failed to extract staged files");

    assert_eq!(
        staged_files.len(),
        1,
        "Expected exactly 1 staged file, got {}",
        staged_files.len()
    );
    assert_eq!(
        staged_files[0],
        root.join("staged_secret.txt"),
        "The extracted file path does not match the staged file"
    );
}
