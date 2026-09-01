// tests/audit/parsers/python/test_poetry.rs

use rustywoof::audit::parsers::python::poetry::PoetryParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_poetry_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("poetry.lock"),
        "[[package]]\nname = \"httpx\"\nversion = \"0.24.1\"\n",
    )
    .unwrap();

    let parser = PoetryParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse poetry.lock");
    assert_eq!(graph.node_count(), 1);
}
