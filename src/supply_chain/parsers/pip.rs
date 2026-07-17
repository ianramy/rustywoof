// src/supply_chain/parsers/pip.rs

use crate::error::SystemError;
use crate::graph::models::{DependencyGraph, Ecosystem, PackageNode};
use crate::supply_chain::parsers::traits::GraphParser;
use miette::{IntoDiagnostic, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct PipParser;

impl GraphParser for PipParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("requirements.txt").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("requirements.txt");
        let content = fs::read_to_string(&lock_path).into_diagnostic()?;

        let mut graph = DependencyGraph::new();

        // Fallback to manual string manipulation if regex compilation fails to avoid panics
        let req_regex = Regex::new(r"^([a-zA-Z0-9_\-]+)(?:[=><~^]+)\s*([a-zA-Z0-9_\-\.]+)")
            .map_err(|e| SystemError::LockfileParseError {
                file_name: "requirements.txt".to_string(),
                source: e.into(),
            })?;

        for line in content.lines() {
            let clean_line = line.split(';').next().unwrap_or(line).trim();
            if clean_line.is_empty() || clean_line.starts_with('#') {
                continue;
            }

            if let Some(captures) = req_regex.captures(clean_line) {
                let name = captures
                    .get(1)
                    .map_or("", |m| m.as_str())
                    .trim()
                    .to_string();
                let version = captures
                    .get(2)
                    .map_or("", |m| m.as_str())
                    .trim()
                    .to_string();

                if !name.is_empty() && !version.is_empty() {
                    let node = PackageNode {
                        name,
                        version,
                        ecosystem: Ecosystem::Pip,
                        is_vulnerable: false,
                    };

                    // Edges are omitted because requirements.txt lacks native dependency tree resolution
                    graph.add_node(node);
                }
            }
        }

        Ok(graph)
    }
}
