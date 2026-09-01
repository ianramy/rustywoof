// src/audit/parsers/javascript/bun.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use miette::{Result, miette};
use std::fs;
use std::path::Path;

pub struct BunParser;

impl GraphParser for BunParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("bun.lockb").exists() || directory.join("bun.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let pkg_path = directory.join("package.json");
        let content = fs::read_to_string(&pkg_path)
            .map_err(|e| miette!("Failed to read package.json for Bun parsing: {}", e))?;

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
                        ecosystem: Ecosystem::Bun,
                        is_vulnerable: false,
                    });
                }
            }
        }

        Ok(graph)
    }
}
