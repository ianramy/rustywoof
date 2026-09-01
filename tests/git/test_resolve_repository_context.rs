// tests/git/test_resolve_repository_context.rs

use rustywoof::git::resolve_repository_context;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_resolves_git_root_from_deep_subdirectory() {
    let dir = tempdir().unwrap();
    let root_path = dir.path();

    let status = Command::new("git")
        .arg("init")
        .current_dir(root_path)
        .status()
        .expect("Failed to execute git init during test setup");
    assert!(status.success(), "Git init failed");

    let deep_sub_dir = root_path.join("src").join("components").join("auth");
    fs::create_dir_all(&deep_sub_dir).unwrap();

    let resolved_root = resolve_repository_context::find_git_root(&deep_sub_dir)
        .expect("Failed to resolve git root from subdirectory");

    assert_eq!(
        resolved_root.canonicalize().unwrap(),
        root_path.canonicalize().unwrap(),
        "Context resolver failed to find the correct repository root"
    );
}

#[test]
fn test_fails_gracefully_when_not_in_git_repository() {
    let dir = tempdir().unwrap();

    let result = resolve_repository_context::find_git_root(dir.path());

    assert!(
        result.is_err(),
        "Context resolver should return an error when no .git directory exists in the parent hierarchy"
    );
}
