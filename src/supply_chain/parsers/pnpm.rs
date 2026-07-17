// src/supply_chain/parsers/pnpm.rs

use crate::error::SystemError;
use crate::graph::models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use crate::supply_chain::parsers::traits::GraphParser;
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct PnpmParser;

impl GraphParser for PnpmParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("pnpm-lock.yaml").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("pnpm-lock.yaml");
        let content = fs::read_to_string(&lock_path).into_diagnostic()?;

        let parsed: serde_norway::Value =
            serde_norway::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "pnpm-lock.yaml".to_string(),
                source: e.into(),
            })?;

        let mut graph = DependencyGraph::new();
        let mut node_map: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

        let packages = parsed
            .get("packages")
            .and_then(|p| p.as_mapping())
            .ok_or_else(|| SystemError::LockfileParseError {
                file_name: "pnpm-lock.yaml".to_string(),
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing 'packages' mapping",
                )
                .into(),
            })?;

        // Pass 1: Generate Nodes
        for (key, _) in packages {
            let path = key.as_str().unwrap_or_default();
            if path.is_empty() || !path.contains('@') {
                continue;
            }

            let parts: Vec<&str> = path.trim_start_matches('/').rsplitn(2, '@').collect();
            if parts.len() == 2 {
                let mut version = parts[0];
                let name = parts[1];

                if let Some(clean_version) = version.split('(').next() {
                    version = clean_version;
                }

                let node = PackageNode {
                    name: name.to_string(),
                    version: version.to_string(),
                    ecosystem: Ecosystem::Pnpm,
                    is_vulnerable: false,
                };

                let idx = graph.add_node(node);
                // pnpm stores the absolute package path as the key, which is reliable for node resolution
                node_map.insert(path.to_string(), idx);
            }
        }

        // Pass 2: Generate Edges
        for (key, details) in packages {
            let path = key.as_str().unwrap_or_default();

            if let Some(&p_idx) = node_map.get(path)
                && let Some(deps) = details.get("dependencies").and_then(|d| d.as_mapping())
            {
                for (dep_name_val, dep_req_val) in deps {
                    let dep_name = dep_name_val.as_str().unwrap_or_default();
                    let dep_req = dep_req_val.as_str().unwrap_or_default();

                    // Reconstruct the child path to look up its NodeIndex
                    let child_path = format!("/{}@{}", dep_name, dep_req);

                    if let Some(&c_idx) = node_map.get(&child_path) {
                        graph.add_edge(
                            p_idx,
                            c_idx,
                            DependencyEdge {
                                requirement: dep_req.to_string(),
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
