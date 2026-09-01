// src/audit/parsers/javascript/npm.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use miette::{Result, miette};
use std::fs;
use std::path::Path;

pub struct NpmParser;

impl GraphParser for NpmParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("package-lock.json").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("package-lock.json");
        let content = fs::read_to_string(&lock_path)
            .map_err(|e| miette!("Failed to read package-lock.json: {}", e))?;

        let mut graph = DependencyGraph::new();
        let mut current_name = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("\"node_modules/") {
                current_name = line
                    .replace("\"node_modules/", "")
                    .replace("\": {", "")
                    .replace('"', "");
            } else if !current_name.is_empty() && line.starts_with("\"version\":") {
                let version = line
                    .replace("\"version\":", "")
                    .replace(['"', ','], "")
                    .trim()
                    .to_string();

                graph.add_node(PackageNode {
                    name: current_name.clone(),
                    version,
                    ecosystem: Ecosystem::Npm,
                    is_vulnerable: false,
                });
                current_name.clear(); // reset for next module
            }
        }

        Ok(graph)
    }
}
