// src/terminal/generate_shell_completions.rs

use clap::Command;
use clap_complete::{Shell, generate};
use std::io::Write;

pub fn generate_to_writer<W: Write>(shell: Shell, cmd: &mut Command, bin_name: &str, buf: &mut W) {
    generate(shell, cmd, bin_name, buf);
}
