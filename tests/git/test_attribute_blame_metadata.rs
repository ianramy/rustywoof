// tests/git/test_attribute_blame_metadata.rs

use rustywoof::git::attribute_blame_metadata;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_extracts_blame_metadata_for_line() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Rusty Dog"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "rusty@woof.com"])
        .current_dir(root)
        .status()
        .unwrap();

    let file_path = root.join("credentials.txt");
    fs::write(&file_path, "line 1\nSECRET_KEY=12345\nline 3\n").unwrap();

    Command::new("git")
        .args(["add", "credentials.txt"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "Add credentials"])
        .current_dir(root)
        .status()
        .unwrap();

    let blame_info = attribute_blame_metadata::get_blame_for_line(root, &file_path, 2)
        .expect("Failed to extract blame metadata");

    assert_eq!(
        blame_info.author, "Rusty Dog",
        "Author name was not correctly parsed from git blame."
    );
    assert_eq!(
        blame_info.author_email, "rusty@woof.com",
        "Author email was not correctly parsed from git blame."
    );
    assert!(
        !blame_info.commit_hash.is_empty(),
        "Commit hash should not be empty."
    );
}
