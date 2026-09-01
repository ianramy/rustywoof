// src/commands/trace_dependency_graph.rs

use crate::graph;
use miette::{IntoDiagnostic, Result};
use std::env;

pub fn run(package: Option<&str>) -> Result<()> {
    let current_dir = env::current_dir().into_diagnostic()?;
    graph::orchestrate_graph_analysis::execute_sniff(package, &current_dir)
}
