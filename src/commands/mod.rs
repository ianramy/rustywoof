// src/commands/mod.rs

pub mod audit_supply_chain;
pub mod display_version;
pub mod enforce_strict_check;
pub mod execute_full_patrol;
pub mod generate_shell_completions;
pub mod initialize_configuration;
pub mod manage_git_hooks;
pub mod manage_local_cache;
pub mod remediate_package;
pub mod sweep_secrets;
pub mod trace_dependency_graph;
pub mod update_rustywoof_engine;

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use miette::Result;

pub const LOGO: &str = concat!(
    "\x1b[38;5;208m                  ▒              ▒\x1b[0m\n",
    "\x1b[38;5;208m                 ▓▓▓            ▓▓▓\x1b[0m\n",
    "\x1b[38;5;208m                ▒▒▓▓            ▓▓▒▒\x1b[0m\n",
    "\x1b[38;5;208m               ▒▓▓▓▓▓▓▓       ▓▓▓▓▓▓▓▒\x1b[0m\n",
    "\x1b[38;5;208m               ▒▒▓▓▓▓▓▓       ▓▓▓▓▓▓▒▒\x1b[0m\n",
    "\x1b[38;5;208m       ▓     ▒▒▓▓▓▓▓▓▒▒       ▒▒▓▓▓▓▓▓▒▒    ▓\x1b[0m\n",
    "\x1b[38;5;208m     ▒▓▒▒     ▒▓▓▓▓▓▓▓▓▓     ▓▓▓▓▓▓▓▓▓▒   ▒▒▓▒\x1b[0m          \x1b[1;36m ____                  _                                       __\x1b[0m\n",
    "\x1b[38;5;208m    ▒▒▒▓▓▓▓▓▓   ▓▓▓▓▓▓▓▓▓   ▓▓▓▓▓▓▓▓▓   ▓▓▓▓▓▓▒▒\x1b[0m        \x1b[1;36m|  _ \\  _   _    ___  | |_   _   _  __      __  ___     ___   / _|\x1b[0m\n",
    "\x1b[38;5;208m   ▒▒▓▓▓▓▓▓▓▓▓▓  ▒▓▓▓          ▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓▒▒\x1b[0m       \x1b[1;36m| |_) | | | | | / __| | __| | | | | \\ \\ /\\ / / / _ \\   / _ \\  | |_\x1b[0m\n",
    "\x1b[38;5;208m  ▒▒▓▓▓▓▓▓▓▓▓▓      ▒▒▓▓▓▓▓▓▓▓▓       ▓▓▓▓▓▓▓▓▓▓▒▒\x1b[0m      \x1b[1;36m|  _ <  | |_| | \\__ \\ | |_  | |_| |  \\ V  V / | (_) | | (_) | |  _|\x1b[0m\n",
    "\x1b[38;5;208m   ▒▒▓▓▓▓▓▓▓▓▓▓   ▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓   ▓▓▓▓▓▓▓▓▓▓▒▒\x1b[0m        \x1b[1;36m|_| \\_\\  \\__,_| |___/ \\__|   \\__, |   \\_/\\_/   \\___/   \\___/  |_|\x1b[0m\n",
    "\x1b[38;5;208m    ▒▒▓▓▓▓      ▒▒▒▓▓▓▒▒▒▒▒            ▓▓▓▓▒▒\x1b[0m                                        \x1b[1;36m|___/\x1b[0m\n",
    "\x1b[38;5;208m             ▒▒▓▓▓▓▓▓        ▓▓▓▓▓▒▒▒▒\x1b[0m\n",
    "\x1b[38;5;208m            ▒▒▓▓▓▓▓         ▓▓▓▒▒▒▓▓▓▓▒▒\x1b[0m\n",
    "\x1b[38;5;208m          ▒▒▓▓▓▓▓▓▓▓▓▒▒       ▓▓▓▓▓▓▓▓▓▒▒\x1b[0m\n",
    "\x1b[38;5;208m          ▒▒▓▓▓▓▓▒▒▒▒▒            ▓▓▓▓▓▒▒▒\x1b[0m\n",
    "\x1b[38;5;208m           ▒▓▓▒▒                   ▒▒▓▓▒\x1b[0m\n",
    "\x1b[38;5;208m            ▒▓▓▓▒▒             ▒▒▒▒▓▓▓▒\x1b[0m\n",
    "\x1b[38;5;208m             ▒▓▓▓▓            ▓▓▓▓▓▓\x1b[0m\n"
);

#[derive(Parser)]
#[command(
    name = "woof",
    author,
    version,
    about = "Enterprise-grade secret scanner and supply chain watchdog.",
    long_about = "Rustywoof (woof) is a high-performance security tool designed to detect exposed credentials and compromised dependencies before they breach your perimeter.",
    before_help = LOGO
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scans a target directory for exposed cryptographic secrets and tokens
    Scan {
        #[arg(
            help = "The directory path to sweep",
            long_help = "Recursively searches the specified directory for high-entropy tokens, private keys, and hardcoded credentials. Defaults to the current directory.",
            default_value = "."
        )]
        path: String,
    },
    /// CI/CD mode: Executes a strict scan and returns exit code 1 if violations are found
    Check {
        #[arg(
            help = "The directory path to evaluate",
            long_help = "Designed for CI/CD pipelines. Runs a strict scan and returns a non-zero exit code (1) to fail the build if any secrets are detected.",
            default_value = "."
        )]
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
    Audit {
        #[arg(long, help = "Audit only development dependencies")]
        dev: bool,
        #[arg(long, help = "Audit only production dependencies")]
        prod: bool,
        #[arg(
            long,
            help = "Filter results by severity (low, moderate, high, critical)",
            long_help = "Filter vulnerabilities by severity level. Useful for setting minimum thresholds in automated checks."
        )]
        audit_level: Option<String>,
        #[arg(
            short,
            long,
            help = "Interactive mode for manual review and selection of advisories to fix"
        )]
        interactive: bool,
    },
    /// Attempts automatic remediation of compromised packages
    Remediate {
        #[arg(help = "The package name to remediate")]
        package: String,
        #[arg(help = "The secure version to target")]
        version: String,
    },
    /// Analyzes the dependency graph to trace paths leading to a specific package
    Sniff {
        #[arg(
            help = "The target package name to trace (optional)",
            long_help = "Maps out how a specific dependency was included in your project by walking the lockfile graph."
        )]
        package: Option<String>,
    },
    /// Executes a comprehensive perimeter sweep (Audit + Scan)
    Patrol,
    /// Manages local cache state
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },
    /// Generates shell completion scripts for woof
    Generate {
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Displays version information
    Version,
    /// Updates the Rustywoof engine to the latest version
    Update,
}

#[derive(Subcommand)]
pub enum HookAction {
    Install,
    Remove,
}

#[derive(Subcommand)]
pub enum CacheAction {
    Clean,
}

pub fn execute() -> Result<i32> {
    let cli = Cli::parse();
    let mut exit_code = 0;

    match &cli.command {
        Commands::Init => {
            initialize_configuration::run()?;
        }
        Commands::Hook { action } => {
            manage_git_hooks::run(action)?;
        }
        Commands::Scan { path } => {
            sweep_secrets::run(path);
        }
        Commands::Check { path } => {
            if !enforce_strict_check::run(path) {
                exit_code = 1;
            }
        }
        Commands::Audit {
            dev,
            prod,
            audit_level,
            interactive,
        } => {
            if !audit_supply_chain::run(*dev, *prod, audit_level.clone(), *interactive)? {
                exit_code = 1;
            }
        }
        Commands::Remediate { package, version } => {
            remediate_package::run(package, version)?;
        }
        Commands::Sniff { package } => {
            trace_dependency_graph::run(package.as_deref())?;
        }
        Commands::Patrol => {
            if !execute_full_patrol::run() {
                exit_code = 1;
            }
        }
        Commands::Cache { action } => {
            manage_local_cache::run(action);
        }
        Commands::Generate { shell } => {
            generate_shell_completions::run(*shell);
        }
        Commands::Version => {
            display_version::run();
        }
        Commands::Update => {
            update_rustywoof_engine::run()?;
        }
    }

    Ok(exit_code)
}
