// src/audit/parsers/python/pip.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{DependencyGraph, Ecosystem, PackageNode};
use miette::{Result, miette};
use std::fs;
use std::path::Path;

pub struct PipParser;

impl GraphParser for PipParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("requirements.txt").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let req_path = directory.join("requirements.txt");
        let content = fs::read_to_string(&req_path)
            .map_err(|e| miette!("Failed to read requirements.txt: {}", e))?;

        let mut graph = DependencyGraph::new();

        for line in content.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            let line = line.split(';').next().unwrap_or(line).trim();
            if line.is_empty() || line.starts_with('-') {
                continue;
            }

            let no_extras = match line.find('[') {
                Some(start) => match line.find(']') {
                    Some(end) if end > start => format!("{}{}", &line[..start], &line[end + 1..]),
                    _ => line.to_string(),
                },
                None => line.to_string(),
            };

            let parts: Vec<&str> = no_extras.split(['=', '>', '<', '~']).collect();
            if !parts.is_empty() {
                let name = parts[0].trim().to_string();
                if name.is_empty() {
                    continue;
                }
                let version = if parts.len() > 1 {
                    parts.last().unwrap().trim().to_string()
                } else {
                    "latest".to_string()
                };

                graph.add_node(PackageNode {
                    name,
                    version,
                    ecosystem: Ecosystem::Pip,
                    is_vulnerable: false,
                });
            }
        }

        Ok(graph)
    }
}
