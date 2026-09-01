// tests/git/test_manage_lifecycle_hooks.rs

use rustywoof::git::manage_lifecycle_hooks::{self, HookType};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_deploys_pre_push_guard() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .status()
        .unwrap();

    manage_lifecycle_hooks::deploy_guard(root, HookType::PrePush)
        .expect("Failed to deploy pre-push hook");

    let hook_path = root.join(".git").join("hooks").join("pre-push");
    assert!(
        hook_path.exists(),
        "The pre-push hook was not created on disk"
    );
}
