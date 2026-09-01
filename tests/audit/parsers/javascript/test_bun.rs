// tests/audit/parsers/javascript/test_bun.rs

use rustywoof::audit::parsers::javascript::bun::BunParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_bun_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("bun.lockb"), "").unwrap();
    fs::write(
        dir.path().join("package.json"),
        "\"dependencies\": {\n  \"vue\": \"3.3.4\"\n}",
    )
    .unwrap();

    let parser = BunParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse bun dependencies");
    assert_eq!(graph.node_count(), 1);
    let node = graph
        .node_weight(graph.node_indices().next().unwrap())
        .unwrap();
    assert_eq!(node.name, "vue");
    assert_eq!(node.version, "3.3.4");
}
