// tests/ui/test_detect_headless_environment.rs

use rustywoof::ui::detect_headless_environment;
use std::collections::HashMap;

#[test]
fn test_detects_headless_ci_environments() {
    let mut mock_env = HashMap::new();

    assert!(!detect_headless_environment::is_headless_env(
        mock_env.iter()
    ));

    mock_env.insert("CI".to_string(), "true".to_string());
    assert!(detect_headless_environment::is_headless_env(
        mock_env.iter()
    ));

    mock_env.clear();
    mock_env.insert("NO_COLOR".to_string(), "1".to_string());
    assert!(detect_headless_environment::is_headless_env(
        mock_env.iter()
    ));
}
