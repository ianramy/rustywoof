// tests/scanner/test_guard_environment_secrets.rs

use rustywoof::scanner::guard_environment_secrets;
use std::fs;
use std::io::Cursor;
use tempfile::tempdir;

#[test]
fn test_secures_untracked_env_files() {
    let dir = tempdir().unwrap();
    let env_path = dir.path().join(".env");
    let gitignore_path = dir.path().join(".gitignore");

    fs::write(&env_path, "SECRET=123").unwrap();

    let mut input = Cursor::new(b"y\n");
    let mut output = Vec::new();

    guard_environment_secrets::secure_perimeter(
        dir.path(),
        false, // interactive mode
        &mut input,
        &mut output,
    )
    .expect("Failed to execute perimeter guard");

    assert!(
        gitignore_path.exists(),
        "The guard failed to generate a .gitignore"
    );
    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(
        gitignore_content.contains(".env"),
        "The .gitignore did not quarantine the .env file"
    );
}
