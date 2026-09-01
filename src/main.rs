// src/main.rs

use miette::Result;
use rustywoof::commands;
use std::process;

fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .context_lines(3)
                .build(),
        )
    }))
    .unwrap_or_default();

    // Delegate execution to the command router
    let exit_code = commands::execute()?;

    if exit_code != 0 {
        process::exit(exit_code);
    }

    Ok(())
}
