// src/ui.rs

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// A consistent, high-fps modern braille spinner sequence
const SPINNER_TICKS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"];

/// Builds the heavy-duty, byte-tracking progress bar for the perimeter scanner.
pub fn build_scanner_pb(total_bytes: u64, is_ci: bool) -> Option<ProgressBar> {
    if is_ci {
        return None;
    }

    let pb = ProgressBar::new(total_bytes);
    pb.set_style(
        ProgressStyle::with_template(
            "\x1b[36mScanning\x1b[0m ├── [{bar:30.cyan/blue}] {percent}% | {bytes} / {total_bytes} @ {bytes_per_sec} | \x1b[90m{msg}\x1b[0m"
        )
        .unwrap()
        .progress_chars("█▉░")
    );
    Some(pb)
}

/// Builds the sleek, suspending spinner for the OSV Threat Intelligence network requests.
pub fn build_osv_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} \x1b[36mOSV Interface:\x1b[0m \x1b[90m{msg}\x1b[0m",
        )
        .unwrap()
        .tick_strings(SPINNER_TICKS),
    );
    spinner
}
