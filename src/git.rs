// src/git.rs

use crate::error::SystemError;
use miette::{Result, WrapErr};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const PRE_COMMIT_HOOK: &str = r#"#!/bin/sh
# Watchdog Perimeter Defense Hook
echo "[INFO] Watchdog evaluating commit perimeter..."
woof check .
if [ $? -ne 0 ]; then
    echo "[CRITICAL] Watchdog halted commit. Review security violations above."
    exit 1
fi
"#;

/// Installs the Watchdog pre-commit guard into the local Git repository.
pub fn deploy_guard() -> Result<()> {
    let hook_path = Path::new(".git/hooks/pre-commit");

    if !Path::new(".git").exists() {
        miette::bail!(
            "[ERROR] Repository not found. Execute this command at the root of a Git repository."
        );
    }

    fs::write(hook_path, PRE_COMMIT_HOOK)
        .map_err(SystemError::GitHookFailed)
        .wrap_err("Insufficient permissions to write hook payload.")?;

    let mut perms = fs::metadata(hook_path)
        .map_err(SystemError::GitHookFailed)?
        .permissions();

    #[cfg(unix)]
    {
        perms.set_mode(0o755);

    fs::set_permissions(hook_path, perms)
        .map_err(SystemError::GitHookFailed)
        .wrap_err("Failed to mark hook payload as executable.")?;
    }

    println!("[INFO] Watchdog perimeter guard successfully deployed.");
    Ok(())
}

/// Removes the Watchdog pre-commit guard.
pub fn remove_guard() -> Result<()> {
    let hook_path = Path::new(".git/hooks/pre-commit");
    if hook_path.exists() {
        fs::remove_file(hook_path)
            .map_err(SystemError::GitHookFailed)
            .wrap_err("Failed to detach Watchdog perimeter guard.")?;
        println!("[WARN] Watchdog perimeter guard detached. Commits are now unmonitored.");
    } else {
        println!("[INFO] No perimeter guard found.");
    }
    Ok(())
}
