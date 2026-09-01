// src/detector/execute_heuristic_automaton.rs

use crate::detector::define_core_signatures::{CORE_RULES, SecretRule};
use aho_corasick::AhoCorasick;
use std::sync::LazyLock;

/// Represents a validated secret extracted from a buffer.
pub struct HeuristicFinding<'a> {
    pub rule: &'a SecretRule,
    pub start_offset: usize,
    pub end_offset: usize,
    pub matched_text: String,
}

/// The globally compiled Aho-Corasick automaton for high-speed prefix matching.
pub static RULE_MATCHER: LazyLock<AhoCorasick> = LazyLock::new(|| {
    let prefixes: Vec<&str> = CORE_RULES.iter().map(|rule| rule.prefix).collect();
    AhoCorasick::builder()
        .build(prefixes)
        .expect("Failed to compile Aho-Corasick automaton")
});

/// Sweeps a text buffer for potential secrets. Uses Aho-Corasick for $O(n)$ prefix discovery,
/// then applies a bounded regex check to prevent ReDoS on massive lines.
pub fn scan_buffer(text: &str) -> Vec<HeuristicFinding<'_>> {
    let mut findings = Vec::new();

    for mat in RULE_MATCHER.find_iter(text) {
        let rule = &CORE_RULES[mat.pattern().as_usize()];
        let start = mat.start();

        // Enforce a strict 256-byte window boundary to guarantee fast regex evaluation
        let end_bound = std::cmp::min(text.len(), start + 256);
        let evaluation_window = &text[start..end_bound];

        if let Some(regex_match) = rule.pattern.find(evaluation_window) {
            let match_start = start + regex_match.start();
            let match_end = start + regex_match.end();

            findings.push(HeuristicFinding {
                rule,
                start_offset: match_start,
                end_offset: match_end,
                matched_text: text[match_start..match_end].to_string(),
            });
        }
    }

    findings
}
