// src/git/resolve_repository_context.rs

use miette::{Result, miette};
use std::path::{Path, PathBuf};

pub fn find_git_root(starting_dir: &Path) -> Result<PathBuf> {
    let mut current_dir = starting_dir.to_path_buf();

    loop {
        if current_dir.join(".git").exists() {
            return Ok(current_dir);
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => {
                return Err(miette!(
                    "[ERROR] Repository not found. Execute this command inside a Git repository."
                ));
            }
        }
    }
}
