// tests/config/test_validate_compiled_rules.rs

use rustywoof::config::configuration_schema::{Config, CustomRule};
use rustywoof::config::validate_compiled_rules;

#[test]
fn test_validation_fails_on_invalid_regex() {
    let config = Config {
        custom_rules: vec![CustomRule {
            name: "Broken Rule".to_string(),
            pattern: "[unclosed_bracket".to_string(),
        }],
        ..Config::default()
    };

    let result = validate_compiled_rules::execute_validation(&config);
    assert!(
        result.is_err(),
        "Validation should fail when a custom rule contains an invalid regex."
    );
}

#[test]
fn test_validation_fails_on_invalid_entropy() {
    let config = Config {
        min_entropy: 9.0,
        ..Config::default()
    };

    let result = validate_compiled_rules::execute_validation(&config);
    assert!(
        result.is_err(),
        "Validation should fail when min_entropy exceeds maximum theoretical value."
    );
}
