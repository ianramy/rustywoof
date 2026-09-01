// src/error/aggregate_scan_violations.rs

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanViolation {
    pub file_path: String,
    pub line_number: usize,
    pub rule_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ViolationAggregator {
    pub violations: Vec<ScanViolation>,
}

impl ViolationAggregator {
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
        }
    }

    pub fn push(&mut self, violation: ScanViolation) {
        self.violations.push(violation);
    }

    pub fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn count(&self) -> usize {
        self.violations.len()
    }
}

impl fmt::Display for ViolationAggregator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.violations.is_empty() {
            return write!(f, "No violations detected.");
        }
        writeln!(f, "Found {} violation(s):", self.violations.len())?;
        for v in &self.violations {
            writeln!(f, "- [{}] {}:{}", v.rule_name, v.file_path, v.line_number)?;
        }
        Ok(())
    }
}
