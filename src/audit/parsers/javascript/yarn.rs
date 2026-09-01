// src/audit/parsers/javascript/yarn.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use miette::{Result, miette};
use std::fs;
use std::path::Path;

pub struct YarnParser;

impl GraphParser for YarnParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("yarn.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("yarn.lock");
        let content = fs::read_to_string(&lock_path)
            .map_err(|e| miette!("Failed to read yarn.lock: {}", e))?;

        let mut graph = DependencyGraph::new();
        let mut current_name = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && trimmed.ends_with(':')
                && !trimmed.starts_with("version")
            {
                let clean = trimmed.trim_end_matches(':').replace('\"', "");
                if let Some(idx) = clean.find('@')
                    && idx > 0
                {
                    current_name = clean[..idx].to_string();
                }
            } else if (trimmed.starts_with("version ") || trimmed.starts_with("\"version\""))
                && !current_name.is_empty()
            {
                let version = trimmed.replace("version", "").replace(['"', ':', ' '], "");

                graph.add_node(PackageNode {
                    name: current_name.clone(),
                    version,
                    ecosystem: Ecosystem::Yarn,
                    is_vulnerable: false,
                });
                current_name.clear();
            }
        }

        Ok(graph)
    }
}
