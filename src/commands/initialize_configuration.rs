// src/commands/initialize_configuration.rs

use crate::commands::LOGO;
use crate::config;
use miette::Result;

pub fn run() -> Result<()> {
    let outcome = config::init_config()?;
    print!("{}", LOGO);
    println!(
        "\x1b[1;90minit ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m\n"
    );
    println!(" \x1b[34mℹ\x1b[0m Welcome to Rustywoof! Let's secure your perimeter...\n");
    println!("\x1b[1mFiles created\x1b[0m\n");
    println!("  \x1b[32m- .woof.toml\x1b[0m");
    println!("    Your primary security configuration.\n");

    if !outcome.imported_files.is_empty() {
        let files_str = outcome.imported_files.join(", ");
        println!(
            "  \x1b[34mℹ\x1b[0m Found existing ignore files (\x1b[36m{}\x1b[0m).",
            files_str
        );
        println!(
            "    Rustywoof automatically imported their rules into `.woof.toml` to save you time.\n"
        );
    }

    println!("\x1b[1mNext Steps\x1b[0m\n");
    println!("  \x1b[36m1.\x1b[0m \x1b[1mDeploy the Git hook\x1b[0m");
    println!(
        "     \x1b[32mwoof hook install\x1b[0m blocks exposed secrets before they can be committed."
    );
    println!("     Learn more at \x1b[34mhttps://ianramy.co.ke/rustywoof/cli/hook\x1b[0m\n");
    println!("  \x1b[36m2.\x1b[0m \x1b[1mSweep your codebase\x1b[0m");
    println!(
        "     \x1b[32mwoof scan\x1b[0m searches your repository for high-entropy tokens and keys.\n"
    );
    println!("  \x1b[36m3.\x1b[0m \x1b[1mAudit your supply chain\x1b[0m");
    println!(
        "     \x1b[32mwoof audit\x1b[0m checks all lockfiles against the OSV threat intelligence feed.\n"
    );
    println!("  \x1b[36m4.\x1b[0m \x1b[1mView available commands\x1b[0m");
    println!("     \x1b[32mwoof --help\x1b[0m displays all available features and flags.\n");

    Ok(())
}
