// tests/graph/test_traverse_dependency_paths.rs

use rustywoof::graph::define_dependency_models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use rustywoof::graph::traverse_dependency_paths::GraphPathfinder;

#[test]
fn test_finds_acyclic_paths_to_target() {
    let mut graph = DependencyGraph::new();

    let root = graph.add_node(PackageNode {
        name: "root".into(),
        version: "1.0".into(),
        ecosystem: Ecosystem::Npm,
        is_vulnerable: false,
    });
    let mid = graph.add_node(PackageNode {
        name: "mid".into(),
        version: "1.0".into(),
        ecosystem: Ecosystem::Npm,
        is_vulnerable: false,
    });
    let target = graph.add_node(PackageNode {
        name: "target".into(),
        version: "1.0".into(),
        ecosystem: Ecosystem::Npm,
        is_vulnerable: true,
    });

    let edge = DependencyEdge {
        requirement: "1.0".into(),
        kind: DependencyKind::Runtime,
    };
    graph.add_edge(root, mid, edge.clone());
    graph.add_edge(mid, target, edge.clone());
    graph.add_edge(root, target, edge.clone());

    let pathfinder = GraphPathfinder::new(&graph);
    let paths = pathfinder.find_paths_to("target");

    assert_eq!(
        paths.len(),
        2,
        "Should find exactly two valid paths to the target"
    );
}
