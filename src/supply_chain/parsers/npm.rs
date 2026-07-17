// src/supply_chain/parsers/npm.rs

use crate::error::SystemError;
use crate::graph::models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use crate::supply_chain::parsers::traits::GraphParser;
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct NpmParser;

impl GraphParser for NpmParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("package-lock.json").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("package-lock.json");
        let content = fs::read_to_string(&lock_path).into_diagnostic()?;

        let parsed: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "package-lock.json".to_string(),
                source: e.into(),
            })?;

        let mut graph = DependencyGraph::new();
        let mut node_map: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

        let packages = parsed
            .get("packages")
            .and_then(|p| p.as_object())
            .ok_or_else(|| SystemError::LockfileParseError {
                file_name: "package-lock.json".to_string(),
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing 'packages' object",
                )
                .into(),
            })?;

        // Pass 1: Generate Nodes
        for (path, details) in packages {
            if path.is_empty() {
                continue; // Skip the root workspace node for now
            }

            let name = path
                .split("node_modules/")
                .last()
                .unwrap_or(path)
                .to_string();
            let version = details
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            let node = PackageNode {
                name: name.clone(),
                version,
                ecosystem: Ecosystem::Npm,
                is_vulnerable: false,
            };

            let idx = graph.add_node(node);
            node_map.insert(name, idx);
        }

        // Pass 2: Generate Edges
        for (path, details) in packages {
            let parent_name = path.split("node_modules/").last().unwrap_or(path);

            if let Some(&p_idx) = node_map.get(parent_name)
                && let Some(deps) = details.get("dependencies").and_then(|d| d.as_object())
            {
                for (dep_name, req_val) in deps {
                    let requirement = req_val.as_str().unwrap_or("*").to_string();
                    if let Some(&c_idx) = node_map.get(dep_name) {
                        graph.add_edge(
                            p_idx,
                            c_idx,
                            DependencyEdge {
                                requirement,
                                kind: DependencyKind::Runtime,
                            },
                        );
                    }
                }
            }
        }

        Ok(graph)
    }
}
