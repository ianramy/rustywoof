// src/ui/orchestrate_progress_bars.rs

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::ui::style_terminal_output as styles;

const RADAR_SWEEP: &[&str] = &["◐", "◓", "◑", "◒"];

const SCAN_PULSE: &[&str] = &["◜", "◠", "◝", "◞", "◡", "◟"];

const MAGENTA: &str = "\x1b[35m";

pub struct ProgressOrchestrator {
    multi: MultiProgress,
    is_headless: bool,
}

impl ProgressOrchestrator {
    pub fn new(is_headless: bool) -> Self {
        Self {
            multi: MultiProgress::new(),
            is_headless,
        }
    }

    pub fn add_byte_tracker(&self, total_bytes: u64) -> ProgressBar {
        if self.is_headless {
            return ProgressBar::hidden();
        }

        let pb = self.multi.add(ProgressBar::new(total_bytes));
        pb.set_style(
            ProgressStyle::with_template(&format!(
                "{cyan}Scanning{reset} ├── [{{bar:30.cyan/blue}}] {{percent}}% | {{bytes}} / {{total_bytes}} @ {{bytes_per_sec}} | {dim}{{msg}}{reset}",
                cyan = styles::CYAN,
                dim = styles::DIM,
                reset = styles::RESET,
            ))
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("█▉░"),
        );
        pb
    }

    pub fn add_spinner(&self) -> ProgressBar {
        if self.is_headless {
            return ProgressBar::hidden();
        }

        let spinner = self.multi.add(ProgressBar::new_spinner());
        spinner.enable_steady_tick(Duration::from_millis(80));
        spinner.set_style(
            ProgressStyle::with_template(&format!(
                "{{spinner:.cyan}} {cyan}OSV Interface:{reset} {dim}{{msg}}{reset}",
                cyan = styles::CYAN,
                dim = styles::DIM,
                reset = styles::RESET,
            ))
            .unwrap_or_else(|_| ProgressStyle::default_spinner())
            .tick_strings(RADAR_SWEEP),
        );
        spinner
    }

    pub fn add_vulnerability_spinner(&self) -> ProgressBar {
        if self.is_headless {
            return ProgressBar::hidden();
        }

        let spinner = self.multi.add(ProgressBar::new_spinner());
        spinner.enable_steady_tick(Duration::from_millis(120));
        spinner.set_style(
            ProgressStyle::with_template(&format!(
                "{{spinner:.magenta}} {magenta}Cross-referencing CVEs:{reset} {dim}{{msg}}{reset}",
                magenta = MAGENTA,
                dim = styles::DIM,
                reset = styles::RESET,
            ))
            .unwrap_or_else(|_| ProgressStyle::default_spinner())
            .tick_strings(SCAN_PULSE),
        );
        spinner
    }
}

pub fn finish_clean(pb: &ProgressBar, ok: bool, message: impl Into<String>) {
    let (glyph, color) = if ok {
        ("✓", styles::GREEN)
    } else {
        ("✗", styles::RED)
    };

    pb.set_style(
        ProgressStyle::with_template(&format!(
            "{color}{glyph}{reset} {{msg}}",
            color = color,
            glyph = glyph,
            reset = styles::RESET,
        ))
        .unwrap_or_else(|_| ProgressStyle::default_spinner()),
    );
    pb.finish_with_message(message.into());
}
