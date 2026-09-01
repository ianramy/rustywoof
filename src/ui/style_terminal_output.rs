// src/ui/style_terminal_output.rs

pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[90m";
pub const BOLD: &str = "\x1b[1m";

pub const RED: &str = "\x1b[31m";
pub const BOLD_RED: &str = "\x1b[1;31m";
pub const GREEN: &str = "\x1b[32m";
pub const BOLD_GREEN: &str = "\x1b[1;32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BOLD_YELLOW: &str = "\x1b[1;33m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const LIGHT_GREEN: &str = "\x1b[38;5;148m";
pub const BLUE: &str = "\x1b[34m";
pub const CYAN: &str = "\x1b[36m";

pub struct BoxChars {
    pub horizontal: char,
    pub vertical: char,
    pub tee_up: char,
    pub tee_down: char,
    pub tee_left: char,
    pub tee_right: char,
    pub cross: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

pub const BOX: BoxChars = BoxChars {
    horizontal: '─',
    vertical: '│',
    tee_up: '┴',
    tee_down: '┬',
    tee_left: '┤',
    tee_right: '├',
    cross: '┼',
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
};

pub fn colorize(color: &str, text: &str) -> String {
    format!("{color}{text}{RESET}")
}

pub fn bold(text: &str) -> String {
    colorize(BOLD, text)
}

pub fn dim(text: &str) -> String {
    colorize(DIM, text)
}

pub fn severity_color(level: &str) -> &'static str {
    match level.to_uppercase().as_str() {
        "CRITICAL" => BOLD_RED,
        "HIGH" => ORANGE,
        "MEDIUM" => BOLD_YELLOW,
        "LOW" => YELLOW,
        _ => DIM,
    }
}

pub fn fix_status_color(has_fix: bool) -> &'static str {
    if has_fix { GREEN } else { RED }
}

pub fn horizontal_rule(widths: &[usize], left: char, mid: char, right: char) -> String {
    let segments: Vec<String> = widths
        .iter()
        .map(|w| BOX.horizontal.to_string().repeat(w + 2))
        .collect();
    format!(
        "{DIM}{left}{body}{right}{RESET}",
        left = left,
        body = segments.join(&mid.to_string()),
        right = right
    )
}
