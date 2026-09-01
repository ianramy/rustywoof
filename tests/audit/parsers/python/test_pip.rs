// tests/audit/parsers/python/test_pip.rs

use rustywoof::audit::parsers::python::pip::PipParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_pip_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("requirements.txt"),
        "requests==2.31.0\nDjango>=4.2.0\n",
    )
    .unwrap();

    let parser = PipParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse requirements.txt");
    assert_eq!(graph.node_count(), 2);
}
