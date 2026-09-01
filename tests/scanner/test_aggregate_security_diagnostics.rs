// tests/scanner/test_aggregate_security_diagnostics.rs

use rustywoof::scanner::aggregate_security_diagnostics::format_json_report;
use rustywoof::scanner::execute_file_sweep::SweepFinding;

#[test]
fn test_formats_json_output_for_ci() {
    let findings = vec![SweepFinding {
        file_path: "config.json".to_string(),
        asset_type: "AWS Access Key".to_string(),
        matched_text: "AKIAIOSFODNN7EXAMPLE".to_string(),
        start_offset: 10,
        end_offset: 30,
        error_code: "woof::aws::access_key".to_string(),
        remediation: "Invalidate this key immediately. Rotate credentials.".to_string(),
        entropy: 3.68,
    }];

    let json = format_json_report(&findings, 1);

    assert!(json.contains(r#""status":"failure""#));
    assert!(json.contains(r#""threats":1"#));
}
