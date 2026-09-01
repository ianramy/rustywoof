// src/audit/parsers/mod.rs

pub mod javascript;
pub mod python;
pub mod rust;
pub mod traits;

pub use traits::GraphParser;

use crate::error::segregate_domain_diagnostics::AuditDiagnostic;
use crate::graph::define_dependency_models::DependencyGraph;
use miette::Result;
use std::collections::HashMap;
use std::path::Path;

pub type DependencyData = (Vec<(String, String, String)>, usize);

pub fn build_workspace_graph(directory: &Path) -> Result<DependencyGraph> {
    let js_parsers: Vec<Box<dyn GraphParser>> = vec![
        Box::new(javascript::bun::BunParser),
        Box::new(javascript::pnpm::PnpmParser),
        Box::new(javascript::yarn::YarnParser),
        Box::new(javascript::npm::NpmParser),
        Box::new(javascript::package_json::PackageJsonParser),
    ];

    let python_parsers: Vec<Box<dyn GraphParser>> = vec![
        Box::new(python::uv::UvParser),
        Box::new(python::poetry::PoetryParser),
        Box::new(python::pip::PipParser),
    ];

    let rust_parsers: Vec<Box<dyn GraphParser>> = vec![Box::new(rust::cargo::CargoParser)];

    let parser_groups = vec![js_parsers, python_parsers, rust_parsers];

    let mut unified_graph = DependencyGraph::new();
    let mut lockfiles_processed = 0;

    for group in parser_groups {
        for parser in group {
            if parser.can_parse(directory) {
                if let Ok(subgraph) = parser.parse_graph(directory) {
                    let mut index_map = HashMap::new();

                    for idx in subgraph.node_indices() {
                        if let Some(node) = subgraph.node_weight(idx) {
                            let new_idx = unified_graph.add_node(node.clone());
                            index_map.insert(idx, new_idx);
                        }
                    }

                    for edge in subgraph.edge_indices() {
                        if let (Some((src, dst)), Some(weight)) =
                            (subgraph.edge_endpoints(edge), subgraph.edge_weight(edge))
                            && let (Some(&new_src), Some(&new_dst)) =
                                (index_map.get(&src), index_map.get(&dst))
                        {
                            unified_graph.add_edge(new_src, new_dst, weight.clone());
                        }
                    }
                    lockfiles_processed += 1;
                }

                break;
            }
        }
    }

    if lockfiles_processed == 0 {
        return Err(AuditDiagnostic::NoLockfilesFound.into());
    }

    Ok(unified_graph)
}

pub fn extract_dependencies() -> Result<DependencyData> {
    let current_dir = Path::new(".");
    let graph = build_workspace_graph(current_dir)?;

    let mut all_deps = Vec::new();

    for idx in graph.node_indices() {
        if let Some(node) = graph.node_weight(idx) {
            let legacy_ecosystem_str = match node.ecosystem {
                crate::graph::define_dependency_models::Ecosystem::Cargo => "crates.io",
                crate::graph::define_dependency_models::Ecosystem::Pip => "PyPI",
                _ => "npm",
            };

            all_deps.push((
                node.name.clone(),
                node.version.clone(),
                legacy_ecosystem_str.to_string(),
            ));
        }
    }

    all_deps.sort();
    all_deps.dedup();

    Ok((all_deps, 1))
}
