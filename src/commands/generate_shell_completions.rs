// src/commands/generate_shell_completions.rs

use clap::CommandFactory;
use clap_complete::Shell;

pub fn run(shell: Shell) {
    let mut cmd = crate::commands::Cli::command();
    let bin_name = cmd.get_name().to_string();
    crate::terminal::generate_shell_completions::generate_to_writer(
        shell,
        &mut cmd,
        &bin_name,
        &mut std::io::stdout(),
    );
}
