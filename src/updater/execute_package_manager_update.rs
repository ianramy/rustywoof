// src/updater/execute_package_manager_update.rs

use super::detect_package_manager_context::PackageManager;
use miette::{IntoDiagnostic, Result, miette};
use std::process::Command;

pub trait UpdateCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<bool>;
}

pub struct SystemUpdateCommandRunner;

impl UpdateCommandRunner for SystemUpdateCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<bool> {
        let status = Command::new(program)
            .args(args)
            .status()
            .into_diagnostic()?;
        Ok(status.success())
    }
}

pub fn run_automatic_update<R: UpdateCommandRunner>(
    manager: PackageManager,
    runner: &R,
) -> Result<()> {
    let candidates: &[(&str, &[&str])] = match manager {
        PackageManager::Npm => &[
            ("npm", &["install", "-g", "@ianramy/rustywoof@latest"]),
            ("yarn", &["global", "add", "@ianramy/rustywoof@latest"]),
            ("pnpm", &["install", "-g", "@ianramy/rustywoof@latest"]),
            ("bun", &["install", "-g", "@ianramy/rustywoof@latest"]),
        ],
        PackageManager::Cargo => &[("cargo", &["install", "rustywoof", "--force"])],
        PackageManager::Pip => &[
            ("pip", &["install", "--upgrade", "rustywoof"]),
            ("pip3", &["install", "--upgrade", "rustywoof"]),
        ],
        PackageManager::Homebrew => &[("brew", &["upgrade", "rustywoof"])],
    };

    for (program, args) in candidates {
        if let Ok(true) = runner.run(program, args) {
            return Ok(());
        }
    }

    Err(miette!(
        "Automatic update failed: none of the candidate commands executed successfully."
    ))
}
