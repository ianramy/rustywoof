// src/git/extract_staged_diffs.rs

use miette::{IntoDiagnostic, Result, miette};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_staged_files(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=d"])
        .current_dir(repo_root)
        .output()
        .into_diagnostic()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(miette!(
            "Failed to extract staged files from Git: {}",
            stderr
        ));
    }

    let stdout = String::from_utf8(output.stdout).into_diagnostic()?;

    let files: Vec<PathBuf> = stdout
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| repo_root.join(line))
        .collect();

    Ok(files)
}
