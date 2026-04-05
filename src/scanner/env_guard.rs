// src/scanner/env_guard.rs

use miette::Result;
use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Evaluates the repository perimeter to ensure environment configurations are not exposed.
pub fn secure_perimeter() -> Result<()> {
    let env_exists = Path::new(".env").exists();
    let gitignore_exists = Path::new(".gitignore").exists();

    if !env_exists {
        return Ok(());
    }

    if !gitignore_exists {
        println!("\n[WARN] Watchdog detected a `.env` file, but no `.gitignore` exists.");
        let lang_ignore = detect_and_generate_gitignore();
        if prompt_auto_fix("Generate a secure .gitignore to quarantine the .env file?", 3) {
            fs::write(".gitignore", lang_ignore).expect("Failed to write .gitignore");
            println!("[INFO] Perimeter secured. `.gitignore` generated and `.env` quarantined.");
        }
        return Ok(());
    }

    // Check if .env is properly ignored
    let gitignore_content = fs::read_to_string(".gitignore").unwrap_or_default();
    let is_ignored = gitignore_content.lines().any(|line| line.trim() == ".env" || line.trim() == ".env.*");

    if !is_ignored {
        println!("\n[CRITICAL] Watchdog found a `.env` file that is NOT tracked by `.gitignore`.");
        if prompt_auto_fix("Append `.env` to `.gitignore` automatically to prevent leakage?", 3) {
            let mut file = OpenOptions::new()
                .append(true)
                .open(".gitignore")
                .expect("Failed to open .gitignore");

            writeln!(file, "\n# Watchdog Perimeter Defense\n.env\n.env.*\n").unwrap();
            println!("[INFO] Guard deployed. `.env` has been added to `.gitignore`.");
        }
    }

    Ok(())
}

/// A non-blocking terminal prompt that defaults to 'Yes' after `timeout_secs`.
/// Designed to prevent CI/CD pipelines or absent users from freezing the terminal.
fn prompt_auto_fix(message: &str, timeout_secs: u64) -> bool {
    // If not running in an interactive terminal (e.g., CI/CD pipeline), auto-fix immediately.
    if !io::stdin().is_terminal() {
        println!("[INFO] Non-interactive environment detected. Auto-applying perimeter fix.");
        return true;
    }

    print!("{} [Y/n] (Auto-yes in {}s): ", message, timeout_secs);
    io::stdout().flush().unwrap();

    let (tx, rx) = mpsc::channel();

    // Spawn a background thread to wait for user STDIN
    thread::spawn(move || {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let _ = tx.send(input.trim().to_lowercase());
        }
    });

    // Block the main thread for a maximum of `timeout_secs`
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(input) => {
            input.is_empty() || input == "y" || input == "yes"
        }
        Err(_) => {
            println!("\n[INFO] Timeout reached. Watchdog auto-applying defensive measure.");
            true
        }
    }
}

/// Analyzes project structure to generate a framework-appropriate .gitignore
fn detect_and_generate_gitignore() -> String {
    let mut ignore_content = String::from("# Watchdog Auto-Generated Perimeter\n.env\n.env.*\n");

    if Path::new("package.json").exists() {
        ignore_content.push_str("node_modules/\ndist/\nbuild/\n.next/\n");
    }
    if Path::new("Cargo.toml").exists() {
        ignore_content.push_str("target/\n");
    }
    if Path::new("requirements.txt").exists() || Path::new("pyproject.toml").exists() {
        ignore_content.push_str("__pycache__/\n*.pyc\nvenv/\n.env/\n");
    }

    ignore_content
}
