// tests/detector/test_suppress_false_positives.rs

use rustywoof::detector::define_core_signatures::CORE_RULES;
use rustywoof::detector::execute_heuristic_automaton::HeuristicFinding;
use rustywoof::detector::suppress_false_positives;

#[test]
fn test_suppresses_test_and_mock_contexts() {
    let rule = &CORE_RULES[0];
    let text = "let mock_aws_key = 'AKIAIOSFODNN7EXAMPLE';";

    let finding = HeuristicFinding {
        rule,
        start_offset: 20,
        end_offset: 40,
        matched_text: "AKIAIOSFODNN7EXAMPLE".to_string(),
    };

    assert!(
        suppress_false_positives::is_false_positive(&finding, text),
        "Failed to suppress a mock credential"
    );
}

#[test]
fn test_retains_genuine_secrets() {
    let rule = &CORE_RULES[0];
    let text = "const prod_db_token = 'AKIAIOSFODNN7EXAMPLE';";

    let finding = HeuristicFinding {
        rule,
        start_offset: 23,
        end_offset: 43,
        matched_text: "AKIAIOSFODNN7EXAMPLE".to_string(),
    };

    assert!(
        !suppress_false_positives::is_false_positive(&finding, text),
        "Incorrectly suppressed a genuine production credential"
    );
}
