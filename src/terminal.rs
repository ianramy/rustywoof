// src/terminal.rs

use clap::Command;
use clap_complete::{Shell, generate};
use std::io;

/// Generates shell completion scripts and prints them to stdout
pub fn print_completions(shell: Shell, cmd: &mut Command, bin_name: &str) {
    generate(shell, cmd, bin_name, &mut io::stdout());
}
