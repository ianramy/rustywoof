// tests/graph/test_orchestrate_graph_analysis.rs

use rustywoof::graph::orchestrate_graph_analysis;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_execute_sniff_runs_without_panicking() {
    let dir = tempdir().expect("Failed to create temp directory");
    let lockfile_path = dir.path().join("Cargo.lock");

    let mut file = File::create(&lockfile_path).expect("Failed to create dummy Cargo.lock");
    writeln!(
        file,
        "[[package]]\nname = \"dummy-pkg\"\nversion = \"1.0.0\""
    )
    .expect("Failed to write to dummy Cargo.lock");

    let result =
        orchestrate_graph_analysis::execute_sniff(Some("non-existent-package-12345"), dir.path());

    result.expect("Sniff orchestrator failed or panicked during standard execution");
}
