// src/supply_chain/remediate.rs

use miette::{IntoDiagnostic, Result};
use std::path::Path;
use std::process::{Command, Stdio};

/// Automates package manager commands to force a secure version of a dependency.
pub fn remediate_vulnerability(package: &str, target_version: &str) -> Result<()> {
    println!(
        "\x1b[1;32m[INFO]\x1b[0m Remediating {} to version {}...",
        package, target_version
    );

    let package_target = format!("{}@{}", package, target_version);

    let (cmd, args) = if Path::new("pnpm-lock.yaml").exists() {
        ("pnpm", vec!["add", &package_target])
    } else if Path::new("yarn.lock").exists() {
        ("yarn", vec!["add", &package_target])
    } else if Path::new("Cargo.toml").exists() {
        ("cargo", vec!["add", &package_target])
    } else {
        ("npm", vec!["install", &package_target])
    };

    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .into_diagnostic()?;

    if status.success() {
        println!("\x1b[1;32m[INFO] Remediation successful. Asset locked to secure version.\x1b[0m");
    } else {
        miette::bail!(
            "\x1b[1;31m[ERROR] Remediation failed.\x1b[0m Ensure your package manager is installed and functioning."
        );
    }

    Ok(())
}
