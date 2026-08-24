// src/updater/package_manager_update.rs

//! Detects which package manager installed the running executable and
//! performs the update automatically by invoking that manager directly,
//! rather than printing a command for the user to copy and run themselves.

use miette::{Result, miette};
use std::env;
use std::process::Command;

/// The package manager rustywoof was most likely installed through, inferred
/// from the filesystem path of the currently running executable.
#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Npm,
    Cargo,
    Pip,
    Homebrew,
}

/// Inspects the current executable's path and returns the package manager
/// that most likely installed it, or `None` if it looks like a direct
/// GitHub-release download instead.
pub fn detect_package_manager() -> Result<Option<PackageManager>> {
    let current_exe =
        env::current_exe().map_err(|e| miette!("Failed to get current executable path: {}", e))?;
    let path = current_exe.to_string_lossy().to_lowercase();

    let manager = if path.contains("node_modules")
        || path.contains(".nvm")
        || path.contains("npm")
        || path.contains("yarn")
        || path.contains("pnpm")
        || path.contains("bun")
    {
        Some(PackageManager::Npm)
    } else if path.contains(".cargo") {
        Some(PackageManager::Cargo)
    } else if path.contains(".venv") || path.contains("site-packages") || path.contains("pip") {
        Some(PackageManager::Pip)
    } else if path.contains("homebrew") || path.contains("linuxbrew") || path.contains("cellar") {
        Some(PackageManager::Homebrew)
    } else {
        None
    };

    Ok(manager)
}

/// Runs the appropriate package manager's update command automatically and
/// reports the outcome. Falls back to printing manual instructions only if
/// every candidate binary is missing or every candidate command fails.
pub fn run_automatic_update(manager: PackageManager) -> Result<()> {
    println!("[INFO] Detected package-manager install. Updating automatically...");

    match manager {
        PackageManager::Npm => run_first_working_command(&[
            ("npm", &["install", "-g", "@ianramy/rustywoof@latest"]),
            ("yarn", &["global", "add", "@ianramy/rustywoof@latest"]),
            ("pnpm", &["install", "-g", "@ianramy/rustywoof@latest"]),
            ("bun", &["install", "-g", "@ianramy/rustywoof@latest"]),
        ]),
        PackageManager::Cargo => {
            run_first_working_command(&[("cargo", &["install", "rustywoof", "--force"])])
        }
        PackageManager::Pip => run_first_working_command(&[
            ("pip", &["install", "--upgrade", "rustywoof"]),
            ("pip3", &["install", "--upgrade", "rustywoof"]),
        ]),
        PackageManager::Homebrew => {
            run_first_working_command(&[("brew", &["upgrade", "rustywoof"])])
        }
    }
}

/// Tries each `(program, args)` candidate in order, running the first binary
/// found on `PATH` to completion. Returns `Ok` on the first successful exit
/// status; returns an error listing manual fallback commands only if every
/// candidate is missing or fails.
fn run_first_working_command(candidates: &[(&str, &[&str])]) -> Result<()> {
    for (program, args) in candidates {
        match Command::new(program).args(*args).status() {
            Ok(status) if status.success() => {
                println!(
                    "\n\x1b[32m✓\x1b[0m [SUCCESS] Rustywoof updated via `{} {}`.",
                    program,
                    args.join(" ")
                );
                println!(
                    "Please restart your terminal or re-run your command to use the new engine."
                );
                return Ok(());
            }
            Ok(status) => println!(
                "[\x1b[33mWARN\x1b[0m] `{} {}` exited with status {}. Trying next option.",
                program,
                args.join(" "),
                status
            ),
            Err(_) => continue, // Binary not found on PATH, try the next candidate.
        }
    }

    let manual_hint = candidates
        .iter()
        .map(|(program, args)| format!("  {} {}", program, args.join(" ")))
        .collect::<Vec<_>>()
        .join("\n");

    Err(miette!(
        "Automatic update failed: none of the following commands could be run successfully.\n{}\nPlease run one manually.",
        manual_hint
    ))
}
