// src/git/attribute_blame_metadata.rs

use miette::{IntoDiagnostic, Result, miette};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct BlameInfo {
    pub commit_hash: String,
    pub author: String,
    pub author_email: String,
}

pub fn get_blame_for_line(
    repo_root: &Path,
    file_path: &Path,
    line_number: usize,
) -> Result<BlameInfo> {
    let line_arg = format!("{},{}", line_number, line_number);

    let output = Command::new("git")
        .args(["blame", "--porcelain", "-L", &line_arg])
        .arg(file_path)
        .current_dir(repo_root)
        .output()
        .into_diagnostic()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(miette!("Failed to extract git blame metadata: {}", stderr));
    }

    let stdout = String::from_utf8(output.stdout).into_diagnostic()?;
    let mut commit_hash = String::new();
    let mut author = String::new();
    let mut author_email = String::new();

    for (i, line) in stdout.lines().enumerate() {
        if i == 0 {
            if let Some(hash) = line.split_whitespace().next() {
                commit_hash = hash.to_string();
            }
        } else if let Some(a) = line.strip_prefix("author ") {
            author = a.to_string();
        } else if let Some(email) = line.strip_prefix("author-mail ") {
            author_email = email.trim_matches(|c| c == '<' || c == '>').to_string();
        }
    }

    if commit_hash.is_empty() || author.is_empty() {
        return Err(miette!(
            "Failed to parse author or commit hash from git blame porcelain output."
        ));
    }

    Ok(BlameInfo {
        commit_hash,
        author,
        author_email,
    })
}
