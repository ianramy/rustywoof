// tests/error/test_format_json_reports.rs

use rustywoof::error::aggregate_scan_violations::{ScanViolation, ViolationAggregator};
use rustywoof::error::format_json_reports;

#[test]
fn test_formats_violations_as_json() {
    let mut aggregator = ViolationAggregator::new();
    aggregator.push(ScanViolation {
        file_path: "api.key".to_string(),
        line_number: 1,
        rule_name: "Generic API Key".to_string(),
    });

    let json_output =
        format_json_reports::to_json_string(&aggregator).expect("Failed to serialize to JSON");

    assert!(
        json_output.contains(r#""file_path":"api.key""#),
        "JSON missing file_path key/value"
    );
    assert!(
        json_output.contains(r#""line_number":1"#),
        "JSON missing line_number key/value"
    );
    assert!(
        json_output.contains(r#""rule_name":"Generic API Key""#),
        "JSON missing rule_name key/value"
    );
}
