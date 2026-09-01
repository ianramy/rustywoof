// src/audit/parsers/python/poetry.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use miette::{Result, miette};
use std::fs;
use std::path::Path;

pub struct PoetryParser;

impl GraphParser for PoetryParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("poetry.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("poetry.lock");
        let content = fs::read_to_string(&lock_path)
            .map_err(|e| miette!("Failed to read poetry.lock: {}", e))?;

        let mut graph = DependencyGraph::new();
        let mut current_name = String::new();
        let mut current_version = String::new();
        let mut in_package = false;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("[[package]]") {
                if !current_name.is_empty() {
                    graph.add_node(PackageNode {
                        name: current_name.clone(),
                        version: current_version.clone(),
                        ecosystem: Ecosystem::Pip,
                        is_vulnerable: false,
                    });
                }
                current_name.clear();
                current_version.clear();
                in_package = true;
            } else if in_package {
                if line.starts_with("name =") {
                    current_name = line
                        .replace("name =", "")
                        .replace('"', "")
                        .trim()
                        .to_string();
                } else if line.starts_with("version =") {
                    current_version = line
                        .replace("version =", "")
                        .replace('"', "")
                        .trim()
                        .to_string();
                } else if line.starts_with('[') {
                    in_package = false;
                }
            }
        }

        if !current_name.is_empty() {
            graph.add_node(PackageNode {
                name: current_name,
                version: current_version,
                ecosystem: Ecosystem::Pip,
                is_vulnerable: false,
            });
        }

        Ok(graph)
    }
}
