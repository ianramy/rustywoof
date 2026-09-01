// tests/git/test_audit_commit_history.rs

use rustywoof::git::audit_commit_history;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_extracts_all_commit_hashes() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Tester"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(root)
        .status()
        .unwrap();

    fs::write(root.join("file1.txt"), "data1").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(root)
        .status()
        .unwrap();

    fs::write(root.join("file2.txt"), "data2").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "second"])
        .current_dir(root)
        .status()
        .unwrap();

    let hashes = audit_commit_history::get_all_commit_hashes(root)
        .expect("Failed to extract commit history");

    assert_eq!(hashes.len(), 2, "Expected exactly 2 commit hashes");
    assert_eq!(
        hashes[0].len(),
        40,
        "Expected a full 40-character SHA-1 hash"
    );
    assert_eq!(
        hashes[1].len(),
        40,
        "Expected a full 40-character SHA-1 hash"
    );
}
