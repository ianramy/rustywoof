// src/updater/orchestrate_engine_update.rs

use super::detect_package_manager_context::detect_manager;
use super::execute_package_manager_update::{UpdateCommandRunner, run_automatic_update};
use super::extract_release_archive::extract_binary;
use super::replace_host_executable::ExecutableReplacer;
use super::resolve_github_release::{download_asset, resolve_latest_url};
use miette::Result;

pub fn orchestrate_update<R: UpdateCommandRunner, E: ExecutableReplacer>(
    current_exe_path: &str,
    api_url: &str,
    current_version: &str,
    target: &str,
    runner: &R,
    replacer: &E,
) -> Result<()> {
    if let Some(manager) = detect_manager(current_exe_path) {
        return run_automatic_update(manager, runner);
    }

    if let Some(url) = resolve_latest_url(api_url, current_version, target)? {
        let archive = download_asset(&url)?;
        let is_zip = url.ends_with(".zip");
        let binary = extract_binary(archive.path(), is_zip)?;
        replacer.replace(binary.path())?;
    }

    Ok(())
}
