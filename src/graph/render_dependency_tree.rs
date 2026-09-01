// src/graph/render_dependency_tree.rs

use crate::graph::define_dependency_models::{DependencyGraph, DependencyKind};
use crate::graph::traverse_dependency_paths::GraphPathfinder;
use crate::ui::style_terminal_output::{self as styles, bold, colorize, dim};
use miette::{IntoDiagnostic, Result};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::io::Write;

pub struct GraphVisualizer;

impl GraphVisualizer {
    pub fn render_tree_to_writer<W: Write>(
        graph: &DependencyGraph,
        paths: &[Vec<NodeIndex>],
        target_name: &str,
        mut writer: W,
    ) -> Result<()> {
        if paths.is_empty() {
            writeln!(
                writer,
                "  ℹ Package '{}' not found in the dependency graph.",
                target_name
            )
            .into_diagnostic()?;
            return Ok(());
        }

        let target_node = paths[0].last().copied().unwrap();
        let node_weight = graph.node_weight(target_node).unwrap();

        writeln!(writer, "Dependency trace for {}:", target_name).into_diagnostic()?;

        writeln!(
            writer,
            "{}",
            bold(&format!("{}@{}", node_weight.name, node_weight.version))
        )
        .into_diagnostic()?;

        let mut trie = TrieNode::new();
        for path in paths {
            let mut reversed = path.clone();
            reversed.reverse();
            if reversed.len() > 1 {
                trie.insert(&reversed[1..], graph, path);
            }
        }

        Self::print_inverted_trie(&trie, "", true, &mut writer).into_diagnostic()?;
        writeln!(writer, "\nFound 1 version of {}", target_name).into_diagnostic()?;

        Ok(())
    }

    fn print_inverted_trie<W: Write>(
        trie: &TrieNode,
        prefix: &str,
        is_root: bool,
        writer: &mut W,
    ) -> std::io::Result<()> {
        let count = trie.children.len();
        let mut sorted_children: Vec<_> = trie.children.iter().collect();
        sorted_children.sort_by_key(|k| k.0);

        for (i, (label, child)) in sorted_children.iter().enumerate() {
            let is_last = i == count - 1;
            let has_children = !child.children.is_empty();

            let connector = if is_last {
                if has_children {
                    dim("└─┬")
                } else {
                    dim("└──")
                }
            } else {
                if has_children {
                    dim("├─┬")
                } else {
                    dim("├──")
                }
            };

            if is_root {
                writeln!(writer, "{} {}", connector, label)?;
            } else {
                writeln!(writer, "{}{} {}", prefix, connector, label)?;
            }

            let next_prefix = format!(
                "{}{}",
                prefix,
                if is_last {
                    "  ".to_string()
                } else {
                    format!("{} ", dim("│"))
                }
            );
            Self::print_inverted_trie(child, &next_prefix, false, writer)?;
        }
        Ok(())
    }

    pub fn render_workspace_list_to_writer<W: Write>(
        graph: &DependencyGraph,
        pathfinder: &GraphPathfinder,
        mut writer: W,
    ) -> Result<()> {
        writeln!(
            writer,
            "Legend: production dependency, {}, {}\n",
            colorize(styles::CYAN, "optional only"),
            colorize(styles::YELLOW, "dev only")
        )
        .into_diagnostic()?;

        let mut package_count = 0;

        for root_idx in pathfinder.get_workspace_roots() {
            if let Some(node) = graph.node_weight(root_idx) {
                let cwd = std::env::current_dir().unwrap_or_default();
                let display_path = cwd.display();

                writeln!(
                    writer,
                    "{} {} (PRIVATE)",
                    bold(&format!("{}@{}", node.name, node.version)),
                    display_path
                )
                .into_diagnostic()?;

                let mut runtimes = Vec::new();
                let mut dev = Vec::new();

                for edge in graph.edges_directed(root_idx, Direction::Outgoing) {
                    let target = graph.node_weight(edge.target()).unwrap();
                    package_count += 1;

                    if edge.weight().kind == DependencyKind::Development {
                        let label = format!(
                            "{}@{}",
                            colorize(styles::YELLOW, &target.name),
                            dim(&target.version)
                        );
                        dev.push(label);
                    } else if edge.weight().kind == DependencyKind::Optional {
                        let label = format!(
                            "{}@{}",
                            colorize(styles::CYAN, &target.name),
                            dim(&target.version)
                        );
                        runtimes.push(label);
                    } else {
                        let label = format!("{}@{}", target.name, dim(&target.version));
                        runtimes.push(label);
                    }
                }

                runtimes.sort();
                dev.sort();

                if !runtimes.is_empty() || !dev.is_empty() {
                    writeln!(writer, "{}", dim("│")).into_diagnostic()?;
                }

                if !runtimes.is_empty() {
                    writeln!(
                        writer,
                        "{}   {}",
                        dim("│"),
                        colorize(styles::CYAN, "dependencies:")
                    )
                    .into_diagnostic()?;
                    for (i, dep) in runtimes.iter().enumerate() {
                        let prefix = if i == runtimes.len() - 1 && dev.is_empty() {
                            dim("└──")
                        } else {
                            dim("├──")
                        };
                        writeln!(writer, "{} {}", prefix, dep).into_diagnostic()?;
                    }
                    if !dev.is_empty() {
                        writeln!(writer, "{}", dim("│")).into_diagnostic()?;
                    }
                }

                if !dev.is_empty() {
                    writeln!(
                        writer,
                        "{}   {}",
                        dim("│"),
                        colorize(styles::CYAN, "devDependencies:")
                    )
                    .into_diagnostic()?;
                    for (i, dep) in dev.iter().enumerate() {
                        let prefix = if i == dev.len() - 1 {
                            dim("└──")
                        } else {
                            dim("├──")
                        };
                        writeln!(writer, "{} {}", prefix, dep).into_diagnostic()?;
                    }
                }
            }
        }

        writeln!(writer, "\n{} packages", package_count).into_diagnostic()?;
        Ok(())
    }

    pub fn print_sniff_tree(
        graph: &DependencyGraph,
        paths: &[Vec<NodeIndex>],
        target_name: &str,
    ) -> Result<()> {
        Self::render_tree_to_writer(graph, paths, target_name, std::io::stdout())
    }

    pub fn print_workspace_list(
        graph: &DependencyGraph,
        pathfinder: &GraphPathfinder,
    ) -> Result<()> {
        Self::render_workspace_list_to_writer(graph, pathfinder, std::io::stdout())
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

    fn insert(
        &mut self,
        reversed_path: &[NodeIndex],
        graph: &DependencyGraph,
        original_path: &[NodeIndex],
    ) {
        let mut current = self;
        for (i, &idx) in reversed_path.iter().enumerate() {
            let label = if let Some(node) = graph.node_weight(idx) {
                let mut l = format!("{}@{}", node.name, dim(&node.version));

                if i == reversed_path.len() - 1
                    && original_path.len() > 1
                    && let Some(edge) = graph.find_edge(idx, original_path[1])
                {
                    if graph.edge_weight(edge).unwrap().kind == DependencyKind::Development {
                        l.push_str(&format!(" {}", dim("(devDependencies)")));
                    } else {
                        l.push_str(&format!(" {}", dim("(dependencies)")));
                    }
                }

                if i == reversed_path.len() - 1 {
                    bold(&l)
                } else {
                    l
                }
            } else {
                "Unknown".to_string()
            };
            current = current.children.entry(label).or_insert_with(TrieNode::new);
        }
    }
}
