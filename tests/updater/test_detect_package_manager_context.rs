// tests/updater/test_detect_package_manager_context.rs

use rustywoof::updater::detect_package_manager_context::{PackageManager, detect_manager};

#[test]
fn test_detects_cargo_context() {
    let context = detect_manager("/home/user/.cargo/bin/woof").unwrap();
    assert_eq!(context, PackageManager::Cargo);
}

#[test]
fn test_detects_npm_context() {
    let context = detect_manager("/usr/local/lib/node_modules/woof/bin").unwrap();
    assert_eq!(context, PackageManager::Npm);
}

#[test]
fn test_returns_none_for_standalone_binary() {
    let context = detect_manager("/usr/local/bin/woof");
    assert!(context.is_none());
}
