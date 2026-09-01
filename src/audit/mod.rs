// src/audit/mod.rs

pub mod format_vulnerability_diagnostics;
pub mod manage_osv_cache;
pub mod parsers;
pub mod query_osv_api;
pub mod remediation;

use miette::Result;
use std::time::Instant;

pub use remediation::remediate_vulnerability;

use crate::ui::orchestrate_progress_bars::{ProgressOrchestrator, finish_clean};
use crate::ui::style_terminal_output as styles;

/// Audits project lockfiles against the OSV threat intelligence feed
pub fn audit_dependencies(
    dev: bool,
    prod: bool,
    audit_level: Option<String>,
    interactive: bool,
) -> Result<bool> {
    let start_time = Instant::now();
    println!(
        "{tag} Initiating multi-ecosystem lockfile audit...",
        tag = styles::colorize(styles::DIM, "[INFO]"),
    );

    let (all_deps, lockfiles_found) = parsers::extract_dependencies()?;
    println!(
        "{tag} Extracted {} dependencies across {} ecosystems.",
        all_deps.len(),
        lockfiles_found,
        tag = styles::colorize(styles::DIM, "[INFO]"),
    );

    let orchestrator = ProgressOrchestrator::new(false);
    let scan_spinner = orchestrator.add_vulnerability_spinner();
    scan_spinner.set_message(format!("querying OSV for {} packages", all_deps.len()));

    let is_clean =
        query_osv_api::batch_query_osv(&all_deps, None, dev, prod, audit_level, interactive)?;

    let duration = start_time.elapsed();
    let time_str = if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        format!("{}ms", duration.as_millis())
    };

    if is_clean {
        finish_clean(
            &scan_spinner,
            true,
            format!("Audit complete in {time_str}. Zero supply chain vulnerabilities detected."),
        );
    } else {
        finish_clean(
            &scan_spinner,
            false,
            format!("Audit completed in {time_str}. Vulnerabilities detected — see report above."),
        );
    }

    Ok(is_clean)
}
