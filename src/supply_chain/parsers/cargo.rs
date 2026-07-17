// src/supply_chain/parsers/cargo.rs

use crate::error::SystemError;
use crate::graph::models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use crate::supply_chain::parsers::traits::GraphParser;
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct CargoParser;

impl GraphParser for CargoParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("Cargo.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("Cargo.lock");
        let content = fs::read_to_string(&lock_path).into_diagnostic()?;

        let parsed: toml::Value =
            toml::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "Cargo.lock".to_string(),
                source: e.into(),
            })?;

        let mut graph = DependencyGraph::new();
        // Maps name to a list of (version, node_index) to handle multiple versions of the same crate
        let mut node_map: HashMap<String, Vec<(String, petgraph::graph::NodeIndex)>> =
            HashMap::new();

        let packages = parsed
            .get("package")
            .and_then(|p| p.as_array())
            .ok_or_else(|| SystemError::LockfileParseError {
                file_name: "Cargo.lock".to_string(),
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing 'package' array",
                )
                .into(),
            })?;

        // Pass 1: Generate Nodes
        for pkg in packages {
            let name = pkg
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or_default()
                .to_string();
            let version = pkg
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            if name.is_empty() || version.is_empty() {
                continue;
            }

            let node = PackageNode {
                name: name.clone(),
                version: version.clone(),
                ecosystem: Ecosystem::Cargo,
                is_vulnerable: false,
            };

            let idx = graph.add_node(node);
            node_map.entry(name).or_default().push((version, idx));
        }

        // Pass 2: Generate Edges
        for pkg in packages {
            let parent_name = pkg.get("name").and_then(|n| n.as_str()).unwrap_or_default();
            let parent_version = pkg
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            let parent_idx = node_map
                .get(parent_name)
                .and_then(|versions| versions.iter().find(|(v, _)| v == parent_version))
                .map(|(_, idx)| *idx);

            if let Some(p_idx) = parent_idx
                && let Some(deps) = pkg.get("dependencies").and_then(|d| d.as_array())
            {
                for dep in deps {
                    if let Some(dep_str) = dep.as_str() {
                        let mut parts = dep_str.split_whitespace();
                        if let Some(dep_name) = parts.next() {
                            let dep_req = parts.next().unwrap_or("*").to_string();

                            // Link to the first available version as a naive resolution MVP
                            if let Some(child_versions) = node_map.get(dep_name)
                                && let Some((_, c_idx)) = child_versions.first()
                            {
                                graph.add_edge(
                                    p_idx,
                                    *c_idx,
                                    DependencyEdge {
                                        requirement: dep_req,
                                        kind: DependencyKind::Runtime,
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(graph)
    }
}
