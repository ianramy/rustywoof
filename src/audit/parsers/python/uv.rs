// src/audit/parsers/python/uv.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use miette::{Result, miette};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct UvParser;

fn extract_dep_names(line: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("name = \"") {
        rest = &rest[start + "name = \"".len()..];
        if let Some(end) = rest.find('"') {
            names.push(rest[..end].to_string());
            rest = &rest[end + 1..];
        } else {
            break;
        }
    }
    names
}

#[derive(Clone, Copy, PartialEq)]
enum EdgeKind {
    Runtime,
    Dev,
    Optional,
}

impl From<EdgeKind> for DependencyKind {
    fn from(k: EdgeKind) -> Self {
        match k {
            EdgeKind::Runtime => DependencyKind::Runtime,
            EdgeKind::Dev => DependencyKind::Development,
            EdgeKind::Optional => DependencyKind::Optional,
        }
    }
}

impl GraphParser for UvParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("uv.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("uv.lock");
        let content =
            fs::read_to_string(&lock_path).map_err(|e| miette!("Failed to read uv.lock: {}", e))?;

        let mut graph = DependencyGraph::new();
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
        let mut pending_edges: Vec<(String, String, EdgeKind)> = Vec::new();

        let mut current_name = String::new();
        let mut current_version = String::new();
        let mut in_package_block = false;

        let mut current_section = EdgeKind::Runtime;

        let mut in_deps_array = false;
        let mut array_kind = EdgeKind::Runtime;

        let mut in_old_style_deps = false;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("[[package]]") {
                current_name.clear();
                current_version.clear();
                in_package_block = true;
                current_section = EdgeKind::Runtime;
                in_deps_array = false;
                in_old_style_deps = false;
                continue;
            }

            if in_deps_array {
                for name in extract_dep_names(line) {
                    pending_edges.push((current_name.clone(), name, array_kind));
                }
                if line.contains(']') {
                    in_deps_array = false;
                }
                continue;
            }

            if in_old_style_deps {
                if let Some(rest) = line.strip_prefix("name = ") {
                    let dep_name = rest.trim().trim_matches('"').to_string();
                    pending_edges.push((current_name.clone(), dep_name, EdgeKind::Runtime));
                }
                if line.starts_with('[') {
                    in_old_style_deps = line.starts_with("[[package.dependencies]]");
                    if !in_old_style_deps && line.starts_with("[package.dev-dependencies") {
                        current_section = EdgeKind::Dev;
                    } else if !in_old_style_deps
                        && line.starts_with("[package.optional-dependencies")
                    {
                        current_section = EdgeKind::Optional;
                    }
                }
                continue;
            }

            if !in_package_block {
                continue;
            }

            if line.starts_with("[package.dev-dependencies") {
                current_section = EdgeKind::Dev;
                continue;
            }
            if line.starts_with("[package.optional-dependencies") {
                current_section = EdgeKind::Optional;
                continue;
            }
            if line.starts_with("[[package.dependencies]]") {
                in_old_style_deps = true;
                continue;
            }
            if line.starts_with("[package.dependencies]") {
                current_section = EdgeKind::Runtime;
                continue;
            }
            if line.starts_with('[') {
                continue;
            }

            if let Some(rest) = line.strip_prefix("name = ") {
                current_name = rest.trim().trim_matches('"').to_string();
                continue;
            }
            if let Some(rest) = line.strip_prefix("version = ") {
                current_version = rest.trim().trim_matches('"').to_string();
                let idx = graph.add_node(PackageNode {
                    name: current_name.clone(),
                    version: current_version.clone(),
                    ecosystem: Ecosystem::Pip,
                    is_vulnerable: false,
                });
                node_map.insert(current_name.clone(), idx);
                continue;
            }

            if line.contains(" = [") {
                let kind = if line.starts_with("dependencies = [") {
                    EdgeKind::Runtime
                } else {
                    current_section
                };

                for name in extract_dep_names(line) {
                    pending_edges.push((current_name.clone(), name, kind));
                }
                if !line.contains(']') {
                    in_deps_array = true;
                    array_kind = kind;
                }
            }
        }

        for (src_name, dep_name, kind) in pending_edges {
            let (Some(&source_idx), Some(&target_idx)) =
                (node_map.get(&src_name), node_map.get(&dep_name))
            else {
                continue;
            };
            graph.add_edge(
                source_idx,
                target_idx,
                DependencyEdge {
                    requirement: dep_name,
                    kind: kind.into(),
                },
            );
        }

        Ok(graph)
    }
}
