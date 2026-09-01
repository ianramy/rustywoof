// tests/graph/test_define_dependency_models.rs

use rustywoof::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};

#[test]
fn test_ecosystem_display_formatting() {
    assert_eq!(Ecosystem::Cargo.to_string(), "Cargo");
    assert_eq!(Ecosystem::Pip.to_string(), "PyPI");
    assert_eq!(Ecosystem::Npm.to_string(), "npm");
}

#[test]
fn test_graph_initialization() {
    let mut graph = DependencyGraph::new();
    let node = PackageNode {
        name: "test-pkg".to_string(),
        version: "1.0.0".to_string(),
        ecosystem: Ecosystem::Cargo,
        is_vulnerable: false,
    };

    let idx = graph.add_node(node);
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.node_weight(idx).unwrap().name, "test-pkg");
}
