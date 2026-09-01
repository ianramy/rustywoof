// src/git/audit_commit_history.rs

use miette::{IntoDiagnostic, Result, miette};
use std::path::Path;
use std::process::Command;

pub fn get_all_commit_hashes(repo_root: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["log", "--format=%H"])
        .current_dir(repo_root)
        .output()
        .into_diagnostic()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(miette!("Failed to extract commit history: {}", stderr));
    }

    let stdout = String::from_utf8(output.stdout).into_diagnostic()?;
    let hashes: Vec<String> = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(hashes)
}
