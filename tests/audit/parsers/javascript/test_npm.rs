// tests/audit/parsers/javascript/test_npm.rs

use rustywoof::audit::parsers::javascript::npm::NpmParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_npm_parser_extracts_dependencies() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("package-lock.json"),
        "\"node_modules/express\": {\n  \"version\": \"4.18.2\"\n}",
    )
    .unwrap();

    let parser = NpmParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse package-lock");
    assert_eq!(graph.node_count(), 1);
}
