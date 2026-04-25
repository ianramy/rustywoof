// src/updater.rs

use miette::{Result, miette};
use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use std::sync::mpsc;
use std::thread;

/// Updates the CLI to the latest version. Uses `miette` for beautiful error reporting.
pub fn execute_update() -> Result<()> {
    println!("[INFO] Checking for perimeter defense updates...");

    let status = Update::configure()
        .repo_owner("ianramy")
        .repo_name("rustywoof")
        .bin_name("woof")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .identifier(".tar.gz")
        .build()
        .map_err(|e| miette!("Failed to configure the update engine: {}", e))?
        .update()
        .map_err(|e| miette!("Failed to download and install the update: {}", e))?;

    if status.updated() {
        println!(
            "\n\x1b[32m✓\x1b[0m [SUCCESS] Rustywoof successfully updated from v{} to v{}",
            cargo_crate_version!(),
            status.version()
        );
        println!("Please restart your terminal or re-run your command to use the new engine.");
    } else {
        println!(
            "\n\x1b[32m✓\x1b[0m [INFO] Perimeter defense is already operating on the latest version (v{}).",
            cargo_crate_version!()
        );
    }

    Ok(())
}

/// Spawns a non-blocking background thread to check for updates.
/// Returns a Receiver that will eventually hold the new version string if one exists.
pub fn spawn_update_checker() -> mpsc::Receiver<Option<String>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // We silently ignore errors here. If the network is down, we just don't notify them.
        if let Ok(updater) = Update::configure()
            .repo_owner("ianramy")
            .repo_name("rustywoof")
            .bin_name("woof")
            .current_version(cargo_crate_version!())
            .build()
        {
            if let Ok(latest_release) = updater.get_latest_release() {
                // Check if the remote version is strictly greater than our current version
                if self_update::version::bump_is_greater(
                    cargo_crate_version!(),
                    &latest_release.version,
                )
                .unwrap_or(false)
                {
                    let _ = tx.send(Some(latest_release.version));
                    return;
                }
            }
        }
        let _ = tx.send(None);
    });

    rx
}
