// src/scanner/mod.rs

pub mod aggregate_security_diagnostics;
pub mod execute_file_sweep;
pub mod guard_environment_secrets;
pub mod index_target_perimeters;

use crate::config;
use crate::ui::{detect_headless_environment, format_terminal_banners, orchestrate_progress_bars};
use std::io;
use std::path::Path;
use std::time::Instant;

pub fn execute_sweep(target_path: &str, is_ci: bool) -> bool {
    let ignore_paths = match config::load_config() {
        Ok(cfg) => cfg.ignore_paths,
        Err(_) => Vec::new(),
    };

    let target_dir = Path::new(target_path);
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let headless_mode = is_ci || detect_headless_environment::is_headless();

    let _ = guard_environment_secrets::secure_perimeter(
        target_dir,
        headless_mode,
        &mut stdin,
        &mut stdout,
    );

    println!(
        "{}",
        format_terminal_banners::format_info("Indexing perimeter...", headless_mode)
    );
    let metrics = index_target_perimeters::index_directory(target_path, &ignore_paths);

    let orchestrator = orchestrate_progress_bars::ProgressOrchestrator::new(headless_mode);
    let pb = orchestrator.add_byte_tracker(metrics.total_bytes);
    pb.set_message(format!(
        "{}/{} files",
        metrics.total_files, metrics.total_files
    ));

    let start_time = Instant::now();
    let findings = execute_file_sweep::sweep_directory(target_path, &ignore_paths, pb);
    let duration = start_time.elapsed();

    let time_str = if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        format!("{}ms", duration.as_millis())
    };

    let complete_msg = format!(
        "Sweep complete. Analyzed {} files in {}.",
        metrics.total_files, time_str
    );
    println!(
        "{}",
        format_terminal_banners::format_info(&complete_msg, headless_mode)
    );

    if is_ci {
        let json_report =
            aggregate_security_diagnostics::format_json_report(&findings, metrics.total_files);
        println!("{}", json_report);
    } else {
        aggregate_security_diagnostics::print_terminal_report(&findings, metrics.total_files);
    }

    findings.is_empty()
}
