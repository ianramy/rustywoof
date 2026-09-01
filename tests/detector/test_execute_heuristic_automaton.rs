// tests/detector/test_execute_heuristic_automaton.rs

use rustywoof::detector::execute_heuristic_automaton;

#[test]
fn test_automaton_extracts_secrets_within_bounds() {
    let buffer = "let config = { \n key: 'AKIAIOSFODNN7EXAMPLE', \n region: 'us-east-1' \n};";

    let findings = execute_heuristic_automaton::scan_buffer(buffer);

    assert_eq!(findings.len(), 1, "Expected exactly 1 secret finding.");
    let finding = &findings[0];

    assert_eq!(finding.rule.name, "AWS Access Key");
    assert_eq!(finding.matched_text, "AKIAIOSFODNN7EXAMPLE");
    assert_eq!(finding.start_offset, 23);
    assert_eq!(finding.end_offset, 43);
}

#[test]
fn test_automaton_ignores_partial_or_invalid_matches() {
    let buffer = "const id = 'AKIASHORT';";
    let findings = execute_heuristic_automaton::scan_buffer(buffer);

    assert!(
        findings.is_empty(),
        "Expected 0 findings for incomplete patterns."
    );
}
