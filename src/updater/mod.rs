// src/updater/mod.rs

pub mod detect_package_manager_context;
pub mod execute_package_manager_update;
pub mod extract_release_archive;
pub mod orchestrate_engine_update;
pub mod replace_host_executable;
pub mod resolve_github_release;

use execute_package_manager_update::SystemUpdateCommandRunner;
use miette::{IntoDiagnostic, Result};
use replace_host_executable::SystemReplacer;
use std::env;

pub fn execute_update() -> Result<()> {
    let current_exe = env::current_exe().into_diagnostic()?;
    let path_str = current_exe.to_string_lossy();
    let api_url = "https://api.github.com/repos/ianramy/rustywoof/releases/latest";

    orchestrate_engine_update::orchestrate_update(
        &path_str,
        api_url,
        env!("CARGO_PKG_VERSION"),
        env!("TARGET"),
        &SystemUpdateCommandRunner,
        &SystemReplacer,
    )
}
