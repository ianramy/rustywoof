// src/main.rs

use clap::{Parser, Subcommand};
use miette::Result;
use std::process;

mod config;
mod detector;
mod error;
mod git;
mod scanner;
mod supply_chain;
pub mod updater;

#[derive(Parser)]
#[command(
    name = "woof",
    author,
    version,
    about = "Enterprise-grade secret scanner and supply chain watchdog.",
    long_about = "Rustywoof (woof) is a high-performance security tool designed to detect exposed credentials and compromised dependencies before they breach your perimeter."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scans a target directory for exposed cryptographic secrets and tokens
    Scan {
        #[arg(help = "The directory path to sweep", default_value = ".")]
        path: String,
    },

    /// CI/CD mode: Executes a strict scan and returns exit code 1 if violations are found
    Check {
        #[arg(help = "The directory path to evaluate", default_value = ".")]
        path: String,
    },

    /// Manages Git pre-commit hooks to prevent secret leakage
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },

    /// Initializes a local .woof.toml configuration file
    Init,

    /// Audits project lockfiles against the OSV threat intelligence feed
    Audit,

    /// Attempts automatic remediation of compromised packages
    Remediate {
        #[arg(help = "The package name to remediate")]
        package: String,
        #[arg(help = "The secure version to target")]
        version: String,
    },

    /// Executes a comprehensive perimeter sweep (Audit + Scan)
    Patrol,

    /// Updates the Rustywoof engine to the latest version
    Update,
}

#[derive(Subcommand)]
enum HookAction {
    /// Deploys the pre-commit guard
    Install,
    /// Detaches the pre-commit guard
    Remove,
}

fn main() -> Result<()> {
    // 1. Fire and forget the background update checker immediately
    let update_receiver = crate::updater::spawn_update_checker();

    // Optional: Setup miette's graphical error handler strictly for terminal environments
    // This ensures colors and formatting are pristine.
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .context_lines(3)
                .build(),
        )
    }))
    .unwrap_or_default();

    let cli = Cli::parse();
    let mut exit_code = 0;

    match &cli.command {
        Commands::Init => {
            config::init_config()?;
        }

        Commands::Hook { action } => match action {
            HookAction::Install => git::deploy_guard()?,
            HookAction::Remove => git::remove_guard()?,
        },

        Commands::Scan { path } => {
            scanner::execute_sweep(path, false);
        }

        Commands::Check { path } => {
            // In CI/CD, any finding should fail the build
            if !scanner::execute_sweep(path, true) {
                exit_code = 1;
            }
        }

        Commands::Audit => {
            if !supply_chain::audit_dependencies()? {
                exit_code = 1;
            }
        }

        Commands::Remediate { package, version } => {
            supply_chain::remediate_vulnerability(package, version)?;
        }

        Commands::Patrol => {
            println!("[INFO] Deploying Watchdog for full perimeter patrol...\n");

            // 1. Audit the Supply Chain
            let clean_deps = match supply_chain::audit_dependencies() {
                Ok(status) => status,
                Err(e) => {
                    println!("{:?}", e);
                    false
                }
            };

            println!("\n--------------------------------------------------\n");

            // 2. Sweep for Secrets (The scanner automatically calls env_guard first)
            let clean_secrets = scanner::execute_sweep(".", false);

            println!("\n--------------------------------------------------");
            if clean_secrets && clean_deps {
                println!("[INFO] Patrol Complete: Perimeter is completely secure.");
                println!("[INFO] You are cleared to commit and push.");
            } else {
                println!("[CRITICAL] Patrol Complete: Threats remain inside the perimeter.");
                println!(
                    "[INFO] Action Required: Review diagnostics above and remediate before pushing."
                );
                exit_code = 1;
            }
        }

        Commands::Update => {
            crate::updater::execute_update()?;
        }
    }

    // 2. Check if the background thread found an update
    if !matches!(cli.command, Commands::Update) {
        if let Ok(Some(new_version)) = update_receiver.try_recv() {
            println!(
                "\n\x1b[33m[NOTICE]\x1b[0m A new engine update (v{}) is available! Run `woof update` to update.",
                new_version
            );
        }
    }

    // 3. Gracefully exit with the correct system code
    if exit_code != 0 {
        process::exit(exit_code);
    }

    Ok(())
}
