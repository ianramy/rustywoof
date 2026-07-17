// src/supply_chain/parsers/yarn.rs

use crate::graph::models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use crate::supply_chain::parsers::traits::GraphParser;
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct YarnParser;

impl GraphParser for YarnParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("yarn.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("yarn.lock");
        let content = fs::read_to_string(&lock_path).into_diagnostic()?;

        let mut graph = DependencyGraph::new();
        // Map abstract requirement (e.g., "lodash@^4.0.0") to concrete NodeIndex
        let mut node_map: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

        // Intermediate structure since Yarn is parsed imperatively
        struct YarnBlock {
            identifiers: Vec<String>,
            version: String,
            dependencies: HashMap<String, String>,
        }

        let mut blocks: Vec<YarnBlock> = Vec::new();
        let mut current_identifiers = Vec::new();
        let mut current_version = String::new();
        let mut current_deps = HashMap::new();
        let mut in_dependencies = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if !line.starts_with(' ') {
                if !current_identifiers.is_empty() && !current_version.is_empty() {
                    blocks.push(YarnBlock {
                        identifiers: std::mem::take(&mut current_identifiers),
                        version: std::mem::take(&mut current_version),
                        dependencies: std::mem::take(&mut current_deps),
                    });
                }
                in_dependencies = false;
                let clean_line = trimmed.trim_end_matches(':');
                current_identifiers = clean_line
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .collect();
            } else if trimmed.starts_with("version ") {
                current_version = trimmed
                    .replace("version ", "")
                    .trim_matches('"')
                    .to_string();
            } else if trimmed == "dependencies:" {
                in_dependencies = true;
            } else if in_dependencies {
                let mut parts = trimmed.splitn(2, ' ');
                if let (Some(dep_name), Some(dep_req)) = (parts.next(), parts.next()) {
                    current_deps.insert(
                        dep_name.trim_matches('"').to_string(),
                        dep_req.trim_matches('"').to_string(),
                    );
                }
            }
        }

        // Push the final block
        if !current_identifiers.is_empty() && !current_version.is_empty() {
            blocks.push(YarnBlock {
                identifiers: current_identifiers,
                version: current_version,
                dependencies: current_deps,
            });
        }

        // Pass 1: Nodes
        for block in &blocks {
            if let Some(first_id) = block.identifiers.first() {
                let name = first_id
                    .rsplitn(2, '@')
                    .last()
                    .unwrap_or(first_id)
                    .to_string();

                let node = PackageNode {
                    name,
                    version: block.version.clone(),
                    ecosystem: Ecosystem::Yarn,
                    is_vulnerable: false,
                };

                let idx = graph.add_node(node);
                for id in &block.identifiers {
                    node_map.insert(id.clone(), idx);
                }
            }
        }

        // Pass 2: Edges
        for block in &blocks {
            if let Some(first_id) = block.identifiers.first()
                && let Some(&p_idx) = node_map.get(first_id)
            {
                for (dep_name, dep_req) in &block.dependencies {
                    let lookup_id = format!("{}@{}", dep_name, dep_req);
                    if let Some(&c_idx) = node_map.get(&lookup_id) {
                        graph.add_edge(
                            p_idx,
                            c_idx,
                            DependencyEdge {
                                requirement: dep_req.clone(),
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
