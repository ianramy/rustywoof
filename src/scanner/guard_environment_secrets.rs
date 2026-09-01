// src/scanner/guard_environment_secrets.rs

use crate::ui::prompt_user_remediation;
use miette::Result;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, Write};
use std::path::Path;

pub fn secure_perimeter<R: BufRead, W: Write>(
    target_dir: &Path,
    is_headless: bool,
    reader: &mut R,
    writer: &mut W,
) -> Result<()> {
    let env_path = target_dir.join(".env");
    let gitignore_path = target_dir.join(".gitignore");

    if !env_path.exists() {
        return Ok(());
    }

    if !gitignore_path.exists() {
        writeln!(
            writer,
            "\n[WARN] Watchdog detected a `.env` file, but no `.gitignore` exists."
        )
        .unwrap_or_default();

        let confirm = prompt_user_remediation::ask_confirm(
            "Generate a secure .gitignore to quarantine the .env file?",
            is_headless,
            reader,
            writer,
        )?;

        if confirm || is_headless {
            fs::write(
                &gitignore_path,
                "# Watchdog Perimeter Defense\n.env\n.env.*\n",
            )
            .unwrap_or_default();
            writeln!(
                writer,
                "[INFO] Perimeter secured. `.gitignore` generated and `.env` quarantined."
            )
            .unwrap_or_default();
        }
        return Ok(());
    }

    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap_or_default();
    let is_ignored = gitignore_content
        .lines()
        .any(|line| line.trim() == ".env" || line.trim() == ".env.*");

    if !is_ignored {
        writeln!(
            writer,
            "\n[CRITICAL] Watchdog found a `.env` file that is NOT tracked by `.gitignore`."
        )
        .unwrap_or_default();

        let confirm = prompt_user_remediation::ask_confirm(
            "Append `.env` to `.gitignore` automatically to prevent leakage?",
            is_headless,
            reader,
            writer,
        )?;

        if confirm || is_headless {
            let mut file = OpenOptions::new()
                .append(true)
                .open(&gitignore_path)
                .unwrap();
            writeln!(file, "\n# Watchdog Perimeter Defense\n.env\n.env.*\n").unwrap_or_default();
            writeln!(
                writer,
                "[INFO] Guard deployed. `.env` has been added to `.gitignore`."
            )
            .unwrap_or_default();
        }
    }

    Ok(())
}
