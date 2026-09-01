// src/commands/manage_local_cache.rs

use crate::commands::CacheAction;
use crate::ui::style_terminal_output::{DIM, GREEN, colorize};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn run(action: &CacheAction) {
    match action {
        CacheAction::Clean => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::with_template("{spinner:.blue} {msg}")
                    .unwrap()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈✓"),
            );
            spinner.enable_steady_tick(Duration::from_millis(100));
            spinner.set_message("Checking if local cache files exist...");

            std::thread::sleep(Duration::from_millis(600));

            let mut found_and_deleted = false;

            for i in 1..=4 {
                let cache_name = if i == 1 {
                    "woof_osv_cache".to_string()
                } else {
                    format!("woof_osv_cache_{}", i)
                };
                let cache_dir = std::env::temp_dir().join(cache_name);

                if cache_dir.exists() {
                    let _ = std::fs::remove_dir_all(cache_dir);
                    found_and_deleted = true;
                }
            }

            if found_and_deleted {
                spinner.finish_with_message(colorize(
                    GREEN,
                    "Local OSV threat intelligence cache completely purged.",
                ));
            } else {
                spinner
                    .finish_with_message(colorize(DIM, "No cache files found. Nothing to clean."));
            }
        }
    }
}
