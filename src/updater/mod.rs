// src/updater/mod.rs

//! Handles the `woof update` command. Detects how rustywoof was installed and
//! performs the update automatically. Falls back to a direct GitHub release
//! download and atomic binary swap only when no package manager is detected.

mod archive_extraction;
mod github_release_update;
mod package_manager_update;

use miette::Result;
use package_manager_update::detect_package_manager;

/// Runs the full update flow with zero manual steps required from the user:
/// automatically invokes the detected package manager, or falls back to
/// downloading and atomically installing the latest GitHub release binary.
pub fn execute_update() -> Result<()> {
    if let Some(manager) = detect_package_manager()? {
        return package_manager_update::run_automatic_update(manager);
    }

    github_release_update::run_github_release_update()
}
