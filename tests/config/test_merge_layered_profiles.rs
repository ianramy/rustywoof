// tests/config/test_merge_layered_profiles.rs

use rustywoof::config::merge_layered_profiles;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_environment_variable_overrides_toml() {
    let dir = tempdir().unwrap();
    let local_toml = dir.path().join(".woof.toml");

    let toml_content = r#"
    min_entropy = 3.0
    ignore_paths = ["target/"]
    "#;
    fs::write(&local_toml, toml_content).unwrap();

    let mock_env = vec![("WOOF_MIN_ENTROPY".to_string(), "4.5".to_string())];

    let resolved_config = merge_layered_profiles::resolve(&local_toml, None, mock_env)
        .expect("Failed to resolve layered configuration");

    assert_eq!(
        resolved_config.min_entropy, 4.5,
        "Environment variable WOOF_MIN_ENTROPY failed to override TOML setting."
    );
}
#[test]
fn test_hierarchical_global_and_local_merging() {
    let dir = tempdir().unwrap();

    let global_toml = dir.path().join("global.toml");
    let global_content = r#"
    min_entropy = 2.0
    ignore_paths = ["global_ignore/"]
    "#;
    fs::write(&global_toml, global_content).unwrap();

    let local_toml = dir.path().join("local.toml");
    let local_content = r#"
    min_entropy = 4.0
    "#;
    fs::write(&local_toml, local_content).unwrap();

    let resolved_config = merge_layered_profiles::resolve(&local_toml, Some(&global_toml), vec![])
        .expect("Failed to resolve layered configuration");

    assert_eq!(
        resolved_config.min_entropy, 4.0,
        "Local configuration failed to override the global min_entropy."
    );

    assert_eq!(
        resolved_config.ignore_paths,
        vec!["global_ignore/".to_string()],
        "Global ignore_paths were not inherited by the local configuration."
    );
}
