// tests/detector/test_define_core_signatures.rs

use rustywoof::detector::define_core_signatures::{CORE_RULES, Severity};

#[test]
fn test_core_signatures_are_loaded_with_severities() {
    assert!(
        !CORE_RULES.is_empty(),
        "The core rule registry should not be empty."
    );

    let aws_rule = CORE_RULES
        .iter()
        .find(|r| r.name == "AWS Access Key")
        .unwrap();
    assert_eq!(aws_rule.prefix, "AKIA", "AWS rule prefix mismatch.");
    assert_eq!(
        aws_rule.severity,
        Severity::Critical,
        "AWS rule must be classified as Critical severity."
    );
}
