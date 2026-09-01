// src/updater/resolve_github_release.rs

use miette::{IntoDiagnostic, Result, miette};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use tempfile::NamedTempFile;

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

pub fn resolve_latest_url(
    api_url: &str,
    current_version: &str,
    target: &str,
) -> Result<Option<String>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(api_url)
        .header(USER_AGENT, format!("rustywoof/{}", current_version))
        .send()
        .into_diagnostic()?;

    if !response.status().is_success() {
        return Err(miette!(
            "GitHub API returned an error: {}",
            response.status()
        ));
    }

    let release: GitHubRelease = response.json().into_diagnostic()?;
    let latest_version = release.tag_name.trim_start_matches('v');

    if latest_version == current_version {
        return Ok(None);
    }

    let asset = release
        .assets
        .into_iter()
        .find(|a| a.name.contains(target))
        .ok_or_else(|| miette!("No release asset found for architecture: {}", target))?;

    Ok(Some(asset.browser_download_url))
}

pub fn download_asset(url: &str) -> Result<NamedTempFile> {
    let mut response = reqwest::blocking::get(url).into_diagnostic()?;

    if !response.status().is_success() {
        return Err(miette!("Failed to download asset: {}", response.status()));
    }

    let mut tmp_file = NamedTempFile::new().into_diagnostic()?;
    response.copy_to(&mut tmp_file).into_diagnostic()?;

    Ok(tmp_file)
}
