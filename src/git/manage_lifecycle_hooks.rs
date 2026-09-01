// src/git/manage_lifecycle_hooks.rs

use crate::error::segregate_domain_diagnostics::GitDiagnostic;
use miette::{Result, WrapErr};
use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    PreCommit,
    PrePush,
}

impl HookType {
    pub fn filename(&self) -> &'static str {
        match self {
            HookType::PreCommit => "pre-commit",
            HookType::PrePush => "pre-push",
        }
    }
}

const GUARD_HOOK_SCRIPT: &str = r#"#!/bin/sh
# Watchdog Perimeter Defense Hook

# [FIX] Check if woof is in the PATH before enforcing the hook
if ! command -v woof >/dev/null 2>&1; then
    echo "[WARN] Watchdog (woof) is not installed or not in PATH."
    echo "[WARN] Skipping perimeter evaluation. Please install Rustywoof to secure commits."
    exit 0
fi

echo "[INFO] Watchdog evaluating commit perimeter..."
woof check .
if [ $? -ne 0 ]; then
    echo "[CRITICAL] Watchdog halted commit. Review security violations above."
    exit 1
fi
"#;

pub fn deploy_guard(repo_root: &Path, hook_type: HookType) -> Result<()> {
    let hooks_dir = repo_root.join(".git").join("hooks");
    let hook_path = hooks_dir.join(hook_type.filename());

    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir)
            .map_err(GitDiagnostic::GitHookFailed)
            .wrap_err("Failed to create Git hooks directory.")?;
    }

    fs::write(&hook_path, GUARD_HOOK_SCRIPT)
        .map_err(GitDiagnostic::GitHookFailed)
        .wrap_err("Insufficient permissions to write hook payload.")?;

    let mut perms = fs::metadata(&hook_path)
        .map_err(GitDiagnostic::GitHookFailed)?
        .permissions();

    #[cfg(unix)]
    {
        perms.set_mode(0o755);

        fs::set_permissions(&hook_path, perms)
            .map_err(GitDiagnostic::GitHookFailed)
            .wrap_err("Failed to mark hook payload as executable.")?;
    }

    println!(
        "[INFO] Watchdog {} guard successfully deployed.",
        hook_type.filename()
    );
    Ok(())
}

pub fn remove_guard(repo_root: &Path, hook_type: HookType) -> Result<()> {
    let hook_path = repo_root
        .join(".git")
        .join("hooks")
        .join(hook_type.filename());
    if hook_path.exists() {
        fs::remove_file(hook_path)
            .map_err(GitDiagnostic::GitHookFailed)
            .wrap_err("Failed to detach Watchdog perimeter guard.")?;
        println!(
            "[WARN] Watchdog {} guard detached. Commits are now unmonitored.",
            hook_type.filename()
        );
    } else {
        println!(
            "[INFO] No perimeter guard found for {}.",
            hook_type.filename()
        );
    }

    Ok(())
}
