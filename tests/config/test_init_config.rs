// tests/config/test_init_config.rs

use rustywoof::config::init_config;
use std::env;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_init_pulls_ignore_files() {
    let dir = tempdir().unwrap();
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&dir).unwrap();

    fs::write(".gitignore", "dist/\n*.log\n# comment\n").unwrap();
    fs::write(".npmignore", "coverage/\n").unwrap();

    let outcome = init_config().unwrap();

    assert!(
        outcome.imported_files.contains(&".gitignore".to_string()),
        "Should detect .gitignore"
    );
    assert!(
        outcome.imported_files.contains(&".npmignore".to_string()),
        "Should detect .npmignore"
    );

    assert!(outcome.toml_content.contains("\"dist/\""));
    assert!(outcome.toml_content.contains("\"*.log\""));
    assert!(outcome.toml_content.contains("\"coverage/\""));
    assert!(outcome.toml_content.contains("\"node_modules/\""));

    env::set_current_dir(original_dir).unwrap();
}
