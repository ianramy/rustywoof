// src/ui/format_terminal_banners.rs

use crate::ui::style_terminal_output::{BOLD_RED, CYAN, DIM, RED, YELLOW, colorize};

fn banner(label: &str, color: &str, msg: &str, is_headless: bool) -> String {
    if is_headless {
        format!("[{label}] {msg}")
    } else {
        format!("{} {msg}", colorize(color, &format!("[{label}]")))
    }
}

pub fn format_info(msg: &str, is_headless: bool) -> String {
    banner("INFO", DIM, msg, is_headless)
}

pub fn format_success(msg: &str, is_headless: bool) -> String {
    banner("SUCCESS", CYAN, msg, is_headless)
}

pub fn format_warn(msg: &str, is_headless: bool) -> String {
    banner("WARN", YELLOW, msg, is_headless)
}

pub fn format_error(msg: &str, is_headless: bool) -> String {
    banner("ERROR", RED, msg, is_headless)
}

pub fn format_critical(msg: &str, is_headless: bool) -> String {
    banner("CRITICAL", BOLD_RED, msg, is_headless)
}
