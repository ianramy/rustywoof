// tests/audit/parsers/javascript/test_package_json.rs

use rustywoof::audit::parsers::javascript::package_json::PackageJsonParser;
use rustywoof::audit::parsers::traits::GraphParser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_package_json_fallback_parser() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("package.json"),
        "\"dependencies\": {\n  \"react\": \"18.2.0\"\n}",
    )
    .unwrap();

    let parser = PackageJsonParser;
    assert!(parser.can_parse(dir.path()));

    let graph = parser
        .parse_graph(dir.path())
        .expect("Failed to parse package.json");
    assert_eq!(graph.node_count(), 1);
}
