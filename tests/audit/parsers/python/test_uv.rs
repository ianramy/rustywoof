// tests/audit/parsers/python/test_uv.rs

use rustywoof::audit::parsers::python::uv::UvParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_uv_parser_detects_lockfile() {
    let dir = tempdir().unwrap();
    let parser = UvParser;

    assert!(
        !parser.can_parse(dir.path()),
        "UvParser incorrectly claimed a directory without uv.lock"
    );

    fs::write(dir.path().join("uv.lock"), "version = 1").unwrap();

    assert!(
        parser.can_parse(dir.path()),
        "UvParser failed to detect uv.lock"
    );
}

#[test]
fn test_uv_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    let lock_path = dir.path().join("uv.lock");

    let mock_lock = r#"
[[package]]
name = "fastapi"
version = "0.100.0"
"#;
    fs::write(&lock_path, mock_lock).unwrap();

    let parser = UvParser;
    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse uv.lock");

    assert!(graph.node_count() > 0, "Graph should contain nodes");

    let node = graph
        .node_weight(graph.node_indices().next().unwrap())
        .unwrap();
    assert_eq!(node.name, "fastapi");
    assert_eq!(node.version, "0.100.0");
}
