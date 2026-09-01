// tests/graph/test_render_dependency_tree.rs

use rustywoof::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use rustywoof::graph::render_dependency_tree::GraphVisualizer;

#[test]
fn test_renders_tree_to_memory_buffer() {
    let mut graph = DependencyGraph::new();

    let n1 = graph.add_node(PackageNode {
        name: "root".to_string(),
        version: "1.0.0".to_string(),
        ecosystem: Ecosystem::Npm,
        is_vulnerable: false,
    });

    let n2 = graph.add_node(PackageNode {
        name: "target".to_string(),
        version: "2.0.0".to_string(),
        ecosystem: Ecosystem::Npm,
        is_vulnerable: false,
    });

    let paths = vec![vec![n1, n2]];
    let mut buffer = Vec::new();

    GraphVisualizer::render_tree_to_writer(&graph, &paths, "target", &mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    assert!(
        output.contains("Dependency trace for"),
        "Missing trace header"
    );

    assert!(
        output.contains("root@\x1b[90m1.0.0\x1b[0m"),
        "Missing root node in tree"
    );

    assert!(
        output.contains("\x1b[1mtarget@2.0.0\x1b[0m"),
        "Missing target node in tree"
    );
}
