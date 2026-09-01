// src/terminal/detect_tty_capabilities.rs

use std::io::{self, IsTerminal};

pub fn is_stdout_interactive() -> bool {
    io::stdout().is_terminal()
}

pub fn is_stdin_interactive() -> bool {
    io::stdin().is_terminal()
}
