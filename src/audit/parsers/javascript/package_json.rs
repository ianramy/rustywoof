// src/audit/parsers/javascript/package_json.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use miette::{Result, miette};
use std::fs;
use std::path::Path;

pub struct PackageJsonParser;

impl GraphParser for PackageJsonParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("package.json").exists() && !directory.join("package-lock.json").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("package.json");
        let content = fs::read_to_string(&lock_path)
            .map_err(|e| miette!("Failed to read package.json: {}", e))?;

        let mut graph = DependencyGraph::new();
        let mut in_deps = false;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("\"dependencies\":") || line.starts_with("\"devDependencies\":") {
                in_deps = true;
            } else if in_deps && line.starts_with('}') {
                in_deps = false;
            } else if in_deps && line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let name = parts[0].replace('"', "").trim().to_string();
                    let raw_version = parts[1].replace(['"', ','], "").trim().to_string();

                    let version = raw_version
                        .trim_start_matches(|c: char| !c.is_ascii_digit())
                        .to_string();

                    graph.add_node(PackageNode {
                        name,
                        version,
                        ecosystem: Ecosystem::Npm,
                        is_vulnerable: false,
                    });
                }
            }
        }

        Ok(graph)
    }
}
