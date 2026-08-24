// src/updater.rs

use miette::{Result, miette};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io;
use tempfile::NamedTempFile;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Struct to parse the GitHub API JSON response
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

pub fn execute_update() -> Result<()> {
    // ---------------------------------------------------------
    // CASE 1: Strict Package Manager Detection
    // ---------------------------------------------------------
    let current_exe =
        env::current_exe().map_err(|e| miette!("Failed to get current executable path: {}", e))?;
    let exe_path_str = current_exe.to_string_lossy().to_lowercase();

    if exe_path_str.contains("node_modules")
        || exe_path_str.contains(".nvm")
        || exe_path_str.contains("npm")
        || exe_path_str.contains("yarn")
        || exe_path_str.contains("pnpm")
        || exe_path_str.contains("bun")
    {
        println!("[\x1b[33mWARN\x1b[0m] Perimeter defense tool was installed via NPM/Node.");
        println!("  Run: npm install -g @ianramy/rustywoof@latest");
        println!("  or: yarn global add @ianramy/rustywoof@latest");
        println!("  or: pnpm install -g @ianramy/rustywoof@latest");
        println!("  or: bun install -g @ianramy/rustywoof@latest");
        return Ok(());
    } else if exe_path_str.contains(".cargo") {
        println!("[\x1b[33mWARN\x1b[0m] Perimeter defense tool was installed via Cargo.");
        println!("  Run: cargo install rustywoof");
        return Ok(());
    } else if exe_path_str.contains(".venv")
        || exe_path_str.contains("site-packages")
        || exe_path_str.contains("pip")
    {
        println!("[\x1b[33mWARN\x1b[0m] Perimeter defense tool was installed via Python/Pip.");
        println!("  Run: pip install --upgrade rustywoof");
        return Ok(());
    } else if exe_path_str.contains("homebrew")
        || exe_path_str.contains("linuxbrew")
        || exe_path_str.contains("cellar")
    {
        println!("[\x1b[33mWARN\x1b[0m] Perimeter defense tool was installed via Homebrew.");
        println!("  Run: brew upgrade rustywoof");
        return Ok(());
    }

    println!("[INFO] Checking for perimeter defense updates...");

    // ---------------------------------------------------------
    // CASE 2: Dynamic GitHub API Resolution & Version Check
    // ---------------------------------------------------------
    let target = env!("TARGET");
    let current_version = env!("CARGO_PKG_VERSION");

    // Allow overriding for local testing, otherwise hit GitHub API
    let download_url = if let Ok(url) = std::env::var("WOOF_UPDATE_URL") {
        url
    } else {
        let client = reqwest::blocking::Client::new();
        let api_url = "https://api.github.com/repos/ianramy/rustywoof/releases/latest";

        // GitHub API requires a User-Agent
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

        // Don't update if already on the latest version
        if latest_version == current_version {
            println!(
                "\n\x1b[32m✓\x1b[0m [INFO] Perimeter defense tool is already operating on the latest version (v{}).",
                current_version
            );
            return Ok(());
        }

        // Find the asset that matches the user's OS TARGET
        let asset = release
            .assets
            .iter()
            .find(|a| a.name.contains(target))
            .ok_or_else(|| miette!("No release asset found for your architecture: {}", target))?;

        println!(
            "[INFO] New version found: v{}. Downloading...",
            latest_version
        );
        asset.browser_download_url.clone()
    };

    let is_tar_gz = download_url.ends_with(".tar.gz") || download_url.ends_with(".tgz");
    let is_zip = download_url.ends_with(".zip");

    // ---------------------------------------------------------
    // CASE 3: Download & Extract
    // ---------------------------------------------------------
    let mut response = reqwest::blocking::get(&download_url)
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

    let binary_to_install = if is_tar_gz {
        extract_tar_gz(&tmp_download_file)?
    } else if is_zip {
        extract_zip(&tmp_download_file)?
    } else {
        tmp_download_file // Assume raw binary
    };

    // ---------------------------------------------------------
    // CASE 4: Set Executable Permissions (Unix Only)
    // ---------------------------------------------------------
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(binary_to_install.path())
            .map_err(|e| miette!("Failed to read temp binary metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(binary_to_install.path(), perms)
            .map_err(|e| miette!("Failed to set executable permissions: {}", e))?;
    }

    // ---------------------------------------------------------
    // CASE 5: The Atomic Swap
    // ---------------------------------------------------------
    self_replace::self_replace(binary_to_install.path())
        .map_err(|e| miette!("Failed to safely replace executable: {}", e))?;

    println!("\n\x1b[32m✓\x1b[0m [SUCCESS] Rustywoof successfully updated!");
    println!("Please restart your terminal or re-run your command to use the new engine.");

    Ok(())
}

// ... keep extract_tar_gz and extract_zip exactly as they were in the previous snippet ...

/// Helper function to extract `woof` or `woof.exe` from a .tar.gz archive
fn extract_tar_gz(archive_file: &NamedTempFile) -> Result<NamedTempFile> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let tar = GzDecoder::new(
        File::open(archive_file).map_err(|e| miette!("Could not open downloaded file: {}", e))?,
    );
    let mut archive = Archive::new(tar);

    for entry in archive
        .entries()
        .map_err(|e| miette!("Could not read archive entries: {}", e))?
    {
        let mut entry = entry.map_err(|e| miette!("Error reading entry: {}", e))?;
        let path = entry
            .path()
            .map_err(|e| miette!("Invalid path in archive: {}", e))?;
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();

        if file_name == "woof" || file_name == "woof.exe" {
            let mut extracted_bin = NamedTempFile::new()
                .map_err(|e| miette!("Failed to create temp bin file: {}", e))?;
            io::copy(&mut entry, &mut extracted_bin)
                .map_err(|e| miette!("Failed to extract binary: {}", e))?;
            return Ok(extracted_bin);
        }
    }

    Err(miette!(
        "Could not find the 'woof' executable inside the downloaded .tar.gz archive."
    ))
}

/// Helper function to extract `woof` or `woof.exe` from a .zip archive (common for Windows)
fn extract_zip(archive_file: &NamedTempFile) -> Result<NamedTempFile> {
    use zip::ZipArchive;

    let file =
        File::open(archive_file).map_err(|e| miette!("Could not open downloaded zip: {}", e))?;
    let mut archive =
        ZipArchive::new(file).map_err(|e| miette!("Could not read zip archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| miette!("Error reading zip entry: {}", e))?;

        if file.name().ends_with("woof") || file.name().ends_with("woof.exe") {
            let mut extracted_bin = NamedTempFile::new()
                .map_err(|e| miette!("Failed to create temp bin file: {}", e))?;
            io::copy(&mut file, &mut extracted_bin)
                .map_err(|e| miette!("Failed to extract binary: {}", e))?;
            return Ok(extracted_bin);
        }
    }

    Err(miette!(
        "Could not find the 'woof.exe' executable inside the downloaded .zip archive."
    ))
}
