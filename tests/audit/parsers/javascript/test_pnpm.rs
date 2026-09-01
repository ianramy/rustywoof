// tests/audit/parsers/javascript/test_pnpm.rs

use rustywoof::audit::parsers::javascript::pnpm::PnpmParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_pnpm_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("pnpm-lock.yaml"),
        "packages:\n  /axios/1.6.0:\n    resolution: {integrity: sha512-...}",
    )
    .unwrap();

    let parser = PnpmParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse pnpm-lock.yaml");
    assert_eq!(graph.node_count(), 1);
    let node = graph
        .node_weight(graph.node_indices().next().unwrap())
        .unwrap();
    assert_eq!(node.name, "axios");
    assert_eq!(node.version, "1.6.0");
}
