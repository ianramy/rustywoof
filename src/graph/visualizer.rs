// src/graph/visualizer.rs

use crate::graph::models::DependencyGraph;
use miette::{IntoDiagnostic, Result};
use petgraph::graph::NodeIndex;
use ptree::{TreeBuilder, print_tree};
use std::collections::HashMap;

/// Renders the discovered dependency paths into a terminal UI tree.
pub struct GraphVisualizer;

impl GraphVisualizer {
    /// Takes the full graph and a list of target paths, filtering out the noise
    /// and printing only the relevant dependency tree to stdout.
    pub fn print_sniff_tree(
        graph: &DependencyGraph,
        paths: &[Vec<NodeIndex>],
        target_name: &str,
    ) -> Result<()> {
        if paths.is_empty() {
            println!(
                "  ℹ Package '{}' not found in the dependency graph.",
                target_name
            );
            return Ok(());
        }

        println!(
            "\n  \x1b[1;32m[!]\x1b[0m Dependency trace for '\x1b[1m{}\x1b[0m':\n",
            target_name
        );

        let mut trie: TrieNode = TrieNode::new();
        for path in paths {
            trie.insert(path, graph);
        }

        let mut builder = TreeBuilder::new("\x1b[1;36mWorkspace Roots\x1b[0m".to_string());
        Self::build_ptree(&trie, &mut builder);

        let tree = builder.build();
        print_tree(&tree).into_diagnostic()?;

        Ok(())
    }

    /// Recursively constructs the `ptree` from our consolidated intermediate Trie.
    fn build_ptree(trie_node: &TrieNode, builder: &mut TreeBuilder) {
        for (label, child_node) in &trie_node.children {
            builder.begin_child(label.clone());
            Self::build_ptree(child_node, builder);
            builder.end_child();
        }
    }
}

struct TrieNode {
    children: HashMap<String, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: &[NodeIndex], graph: &DependencyGraph) {
        let mut current = self;
        for &idx in path {
            let label = if let Some(node) = graph.node_weight(idx) {
                format!(
                    "\x1b[1;33m{}@{}\x1b[0m (\x1b[36m{}\x1b[0m)",
                    node.name, node.version, node.ecosystem
                )
            } else {
                "Unknown Node".to_string()
            };
            current = current.children.entry(label).or_insert_with(TrieNode::new);
        }
    }
}
