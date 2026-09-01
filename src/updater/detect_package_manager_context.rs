// src/updater/detect_package_manager_context.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Cargo,
    Pip,
    Homebrew,
}

pub fn detect_manager(executable_path: &str) -> Option<PackageManager> {
    let path = executable_path.to_lowercase();

    if path.contains("node_modules")
        || path.contains(".nvm")
        || path.contains("npm")
        || path.contains("yarn")
        || path.contains("pnpm")
        || path.contains("bun")
    {
        Some(PackageManager::Npm)
    } else if path.contains(".cargo") {
        Some(PackageManager::Cargo)
    } else if path.contains(".venv") || path.contains("site-packages") || path.contains("pip") {
        Some(PackageManager::Pip)
    } else if path.contains("homebrew") || path.contains("linuxbrew") || path.contains("cellar") {
        Some(PackageManager::Homebrew)
    } else {
        None
    }
}
