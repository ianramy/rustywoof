// src/error/format_json_reports.rs

use crate::error::aggregate_scan_violations::ViolationAggregator;
use miette::{Result, miette};

pub fn to_json_string(aggregator: &ViolationAggregator) -> Result<String> {
    serde_json::to_string(aggregator)
        .map_err(|e| miette!("Failed to serialize violations to JSON: {}", e))
}
