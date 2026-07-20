// src/supply_chain/mod.rs

pub mod osv;
pub mod parsers;
pub mod remediate;

use miette::Result;
pub use remediate::remediate_vulnerability;
use std::time::Instant;

/// Audits project lockfiles against the OSV threat intelligence feed
pub fn audit_dependencies() -> Result<bool> {
    let start_time = Instant::now();
    println!("[INFO] Initiating multi-ecosystem lockfile audit...");

    // 1. Delegate parsing to the parsers module
    let (all_deps, lockfiles_found) = parsers::extract_dependencies()?;

    println!(
        "[INFO] Extracted {} dependencies across {} ecosystems. Querying OSV database...",
        all_deps.len(),
        lockfiles_found
    );

    // 2. Delegate networking and diagnostics to the OSV module
    let is_clean = osv::batch_query_osv(&all_deps)?;

    let duration = start_time.elapsed();
    let time_str = if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        format!("{}ms", duration.as_millis())
    };

    // Ensure the time prints regardless of whether threats were found!
    if is_clean {
        println!(
            "\n\x1b[32m✓\x1b[0m [INFO] Audit complete in {}. Zero supply chain vulnerabilities detected.",
            time_str
        );
    } else {
        println!("\n\x1b[31m×\x1b[0m [INFO] Audit completed in {}.", time_str);
    }

    Ok(is_clean)
}
