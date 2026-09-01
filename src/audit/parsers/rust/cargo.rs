// src/audit/parsers/rust/cargo.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use miette::{Result, miette};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Value;

pub struct CargoParser;

fn strip_quotes(s: &str) -> &str {
    s.trim_matches('"')
}

fn parse_dep_spec(entry: &str) -> (String, Option<String>) {
    let entry = strip_quotes(entry.trim().trim_end_matches(','));
    let without_source = entry.split('(').next().unwrap_or(entry).trim();
    let mut parts = without_source.split_whitespace();
    let name = parts.next().unwrap_or("").to_string();
    let version = parts.next().map(|v| v.to_string());
    (name, version)
}

impl GraphParser for CargoParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("Cargo.lock").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("Cargo.lock");
        let content = fs::read_to_string(&lock_path)
            .map_err(|e| miette!("Failed to read Cargo.lock: {}", e))?;

        // 1. Extract dependency kinds from Cargo.toml
        let mut dep_kinds: HashMap<String, DependencyKind> = HashMap::new();
        let toml_path = directory.join("Cargo.toml");

        if let Ok(toml_content) = fs::read_to_string(&toml_path)
            && let Ok(value) = toml_content.parse::<Value>()
            && let Some(table) = value.as_table()
        {
            let mut extract_section = |section: &str, default_kind: DependencyKind| {
                if let Some(deps) = table.get(section).and_then(|v| v.as_table()) {
                    for (name, val) in deps {
                        let mut kind = default_kind;
                        // Check if the dependency is marked as optional
                        if let Some(details) = val.as_table()
                            && details.get("optional").and_then(|v| v.as_bool()) == Some(true)
                        {
                            kind = DependencyKind::Optional;
                        }
                        dep_kinds.insert(name.clone(), kind);
                    }
                }
            };

            extract_section("dependencies", DependencyKind::Runtime);
            extract_section("dev-dependencies", DependencyKind::Development);
            extract_section("build-dependencies", DependencyKind::Build);
        }

        let mut graph = DependencyGraph::new();
        let mut node_map: HashMap<String, Vec<(String, NodeIndex)>> = HashMap::new();
        let mut pending_edges: Vec<(String, String, String)> = Vec::new();

        let mut current_name = String::new();
        let mut current_version = String::new();
        let mut in_package = false;
        let mut in_deps = false;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("[[package]]") {
                current_name.clear();
                current_version.clear();
                in_package = true;
                in_deps = false;
                continue;
            }

            if in_deps {
                if line.starts_with(']') {
                    in_deps = false;
                } else if !line.is_empty() {
                    pending_edges.push((
                        current_name.clone(),
                        current_version.clone(),
                        line.to_string(),
                    ));
                }
                continue;
            }

            if in_package {
                if let Some(rest) = line.strip_prefix("name =") {
                    current_name = strip_quotes(rest.trim()).to_string();
                } else if let Some(rest) = line.strip_prefix("version =") {
                    current_version = strip_quotes(rest.trim()).to_string();
                    let idx = graph.add_node(PackageNode {
                        name: current_name.clone(),
                        version: current_version.clone(),
                        ecosystem: Ecosystem::Cargo,
                        is_vulnerable: false,
                    });
                    node_map
                        .entry(current_name.clone())
                        .or_default()
                        .push((current_version.clone(), idx));
                } else if line.starts_with("dependencies = [") {
                    in_deps = true;
                } else if line.starts_with('[') {
                    in_package = false;
                }
            }
        }

        for (src_name, src_version, dep_entry) in pending_edges {
            let Some(&(_, source_idx)) = node_map
                .get(&src_name)
                .and_then(|versions| versions.iter().find(|(v, _)| *v == src_version))
            else {
                continue;
            };

            let (dep_name, dep_version) = parse_dep_spec(&dep_entry);
            let Some(versions) = node_map.get(&dep_name) else {
                continue;
            };

            let target_idx = match dep_version {
                Some(v) => versions
                    .iter()
                    .find(|(ver, _)| *ver == v)
                    .map(|(_, idx)| *idx),
                None if versions.len() == 1 => Some(versions[0].1),
                None => None,
            };

            if let Some(target_idx) = target_idx {
                let edge_kind = dep_kinds
                    .get(&dep_name)
                    .copied()
                    .unwrap_or(DependencyKind::Runtime);

                graph.add_edge(
                    source_idx,
                    target_idx,
                    DependencyEdge {
                        requirement: dep_entry,
                        kind: edge_kind,
                    },
                );
            }
        }

        Ok(graph)
    }
}
