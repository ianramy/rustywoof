// src/ui/prompt_user_remediation.rs

use crate::ui::style_terminal_output::{bold, dim};
use miette::Result;
use std::io::{BufRead, Write};

pub fn ask_confirm_with_default<R: BufRead, W: Write>(
    msg: &str,
    default_yes: bool,
    is_headless: bool,
    reader: &mut R,
    writer: &mut W,
) -> Result<bool> {
    let hint = if default_yes { "[Y/n]" } else { "[y/N]" };

    if is_headless {
        writeln!(
            writer,
            "[INFO] Headless environment detected. Auto-{} interactive prompt: {}",
            if default_yes {
                "accepting"
            } else {
                "declining"
            },
            msg
        )
        .unwrap_or_default();
        return Ok(default_yes);
    }

    write!(writer, "{} {}: ", bold(msg), dim(hint)).unwrap_or_default();
    writer.flush().unwrap_or_default();

    let mut response = String::new();
    let bytes_read = reader.read_line(&mut response).unwrap_or(0);

    if bytes_read == 0 {
        writeln!(writer).unwrap_or_default();
        return Ok(default_yes);
    }

    let response = response.trim().to_lowercase();
    if response.is_empty() {
        return Ok(default_yes);
    }

    Ok(matches!(response.as_str(), "y" | "yes"))
}

pub fn ask_confirm<R: BufRead, W: Write>(
    msg: &str,
    is_headless: bool,
    reader: &mut R,
    writer: &mut W,
) -> Result<bool> {
    ask_confirm_with_default(msg, false, is_headless, reader, writer)
}
