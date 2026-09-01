// tests/error/test_aggregate_scan_violations.rs

use rustywoof::error::aggregate_scan_violations::{ScanViolation, ViolationAggregator};

#[test]
fn test_aggregates_multiple_violations() {
    let mut aggregator = ViolationAggregator::new();

    assert!(
        aggregator.is_empty(),
        "Aggregator should be empty on initialization"
    );

    aggregator.push(ScanViolation {
        file_path: "src/auth.rs".to_string(),
        line_number: 42,
        rule_name: "Hardcoded AWS Key".to_string(),
    });

    aggregator.push(ScanViolation {
        file_path: "config/prod.json".to_string(),
        line_number: 12,
        rule_name: "High Entropy String".to_string(),
    });

    assert_eq!(
        aggregator.count(),
        2,
        "Aggregator should contain exactly 2 violations"
    );
    assert!(
        !aggregator.is_empty(),
        "Aggregator should not be empty after pushes"
    );

    let text_report = aggregator.to_string();
    assert!(
        text_report.contains("Hardcoded AWS Key"),
        "Report missing rule name"
    );
    assert!(
        text_report.contains("config/prod.json"),
        "Report missing file path"
    );
}
