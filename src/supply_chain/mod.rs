// src/supply_chain/mod.rs

pub mod osv;
pub mod parsers;
pub mod remediate;

use miette::Result;
pub use remediate::remediate_vulnerability;

/// Audits project lockfiles against the OSV threat intelligence feed
pub fn audit_dependencies() -> Result<bool> {
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

    if is_clean {
        println!("[INFO] Audit complete. Zero supply chain vulnerabilities detected.");
    }

    Ok(is_clean)
}
