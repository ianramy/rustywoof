// tests/audit/parsers/javascript/test_yarn.rs

use rustywoof::audit::parsers::javascript::yarn::YarnParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_yarn_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("yarn.lock"),
        "\"lodash@^4.17.21\":\n  version \"4.17.21\"\n",
    )
    .unwrap();

    let parser = YarnParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse yarn.lock");
    assert_eq!(graph.node_count(), 1);
    let node = graph
        .node_weight(graph.node_indices().next().unwrap())
        .unwrap();
    assert_eq!(node.name, "lodash");
    assert_eq!(node.version, "4.17.21");
}
