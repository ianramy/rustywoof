// src/detector/suppress_false_positives.rs

use crate::detector::execute_heuristic_automaton::HeuristicFinding;

const SUPPRESSION_KEYWORDS: &[&str] = &["example", "test", "dummy", "mock", "fake", "placeholder"];

const CONTEXT_WINDOW: usize = 200;

pub fn is_false_positive(finding: &HeuristicFinding<'_>, full_buffer: &str) -> bool {
    let start = finding.start_offset.saturating_sub(CONTEXT_WINDOW);
    let end = std::cmp::min(full_buffer.len(), finding.end_offset + CONTEXT_WINDOW);

    let prefix = &full_buffer[start..finding.start_offset];
    let suffix = &full_buffer[finding.end_offset..end];

    let context = format!("{} {}", prefix, suffix).to_lowercase();

    for keyword in SUPPRESSION_KEYWORDS {
        if context.contains(keyword) {
            return true;
        }
    }

    false
}
