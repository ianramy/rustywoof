// tests/audit/parsers/rust/test_cargo.rs

use rustywoof::audit::parsers::rust::cargo::CargoParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cargo_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.lock"),
        "[[package]]\nname = \"miette\"\nversion = \"5.10.0\"\n",
    )
    .unwrap();

    let parser = CargoParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse Cargo.lock");
    assert_eq!(graph.node_count(), 1);

    let node = graph
        .node_weight(graph.node_indices().next().unwrap())
        .unwrap();
    assert_eq!(node.name, "miette");
    assert_eq!(node.version, "5.10.0");
}
