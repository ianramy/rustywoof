// src/updater/github_release_update.rs

//! Resolves the latest GitHub release for the current platform, downloads it,
//! and atomically replaces the running executable with the new binary. Used
//! only when no package manager was detected (i.e. a direct binary install).

use miette::{Result, miette};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use tempfile::NamedTempFile;

#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use super::archive_extraction::{extract_tar_gz, extract_zip};

/// Struct to parse the GitHub API JSON response for the latest release.
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

/// A single downloadable asset attached to a GitHub release.
#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// Checks GitHub for a newer release, downloads the matching platform asset,
/// and atomically swaps it in for the currently running executable.
pub fn run_github_release_update() -> Result<()> {
    println!("[INFO] Checking for perimeter defense updates...");

    let target = env!("TARGET");
    let current_version = env!("CARGO_PKG_VERSION");

    let download_url = match std::env::var("WOOF_UPDATE_URL") {
        Ok(url) => url,
        Err(_) => match resolve_latest_asset_url(target, current_version)? {
            Some(url) => url,
            None => return Ok(()), // Already on the latest version.
        },
    };

    let binary_to_install = download_and_extract_binary(&download_url)?;

    #[cfg(unix)]
    mark_binary_executable(&binary_to_install)?;

    self_replace::self_replace(binary_to_install.path())
        .map_err(|e| miette!("Failed to safely replace executable: {}", e))?;

    println!("\n\x1b[32m✓\x1b[0m [SUCCESS] Rustywoof successfully updated!");
    println!("Please restart your terminal or re-run your command to use the new engine.");

    Ok(())
}

/// Queries the GitHub API for the latest release and returns the download URL
/// for this platform's asset, or `None` if already on the latest version.
fn resolve_latest_asset_url(target: &str, current_version: &str) -> Result<Option<String>> {
    let client = reqwest::blocking::Client::new();
    let api_url = "https://api.github.com/repos/ianramy/rustywoof/releases/latest";

    let response = client
        .get(api_url)
        .header(USER_AGENT, format!("rustywoof/{}", current_version))
        .send()
        .map_err(|e| miette!("Network error checking for updates: {}", e))?;

    if !response.status().is_success() {
        return Err(miette!(
            "GitHub API returned an error: {}",
            response.status()
        ));
    }

    let release: GitHubRelease = response
        .json()
        .map_err(|e| miette!("Failed to parse GitHub response: {}", e))?;

    let latest_version = release.tag_name.trim_start_matches('v');

    if latest_version == current_version {
        println!(
            "\n\x1b[32m✓\x1b[0m [INFO] Perimeter defense tool is already operating on the latest version (v{}).",
            current_version
        );
        return Ok(None);
    }

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.contains(target))
        .ok_or_else(|| miette!("No release asset found for your architecture: {}", target))?;

    println!(
        "[INFO] New version found: v{}. Downloading...",
        latest_version
    );

    Ok(Some(asset.browser_download_url.clone()))
}

/// Downloads the asset at `download_url` and extracts the `woof` binary from
/// it if it is an archive, returning a temp file containing the raw binary.
fn download_and_extract_binary(download_url: &str) -> Result<NamedTempFile> {
    let mut response = reqwest::blocking::get(download_url)
        .map_err(|e| miette!("Failed to download update: {}", e))?;

    if !response.status().is_success() {
        return Err(miette!(
            "Failed to download asset. Server returned: {}",
            response.status()
        ));
    }

    let mut tmp_download_file =
        NamedTempFile::new().map_err(|e| miette!("Failed to create temp file: {}", e))?;
    response
        .copy_to(&mut tmp_download_file)
        .map_err(|e| miette!("Failed to write update to disk: {}", e))?;

    if download_url.ends_with(".tar.gz") || download_url.ends_with(".tgz") {
        extract_tar_gz(&tmp_download_file)
    } else if download_url.ends_with(".zip") {
        extract_zip(&tmp_download_file)
    } else {
        Ok(tmp_download_file)
    }
}

/// Sets the extracted binary's permissions to `rwxr-xr-x` so it can be
/// executed once `self_replace` swaps it in for the running process.
#[cfg(unix)]
fn mark_binary_executable(binary: &NamedTempFile) -> Result<()> {
    let mut perms = fs::metadata(binary.path())
        .map_err(|e| miette!("Failed to read temp binary metadata: {}", e))?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(binary.path(), perms)
        .map_err(|e| miette!("Failed to set executable permissions: {}", e))?;
    Ok(())
}
