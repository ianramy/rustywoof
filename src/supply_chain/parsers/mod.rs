// src/supply_chain/parsers/mod.rs

pub mod bun;
pub mod cargo;
pub mod npm;
pub mod package_json;
pub mod pip;
pub mod pnpm;
pub mod poetry;
pub mod traits;
pub mod yarn;

pub use bun::BunParser;
pub use cargo::CargoParser;
pub use npm::NpmParser;
pub use package_json::PackageJsonParser;
pub use pip::PipParser;
pub use pnpm::PnpmParser;
pub use poetry::PoetryParser;
pub use traits::GraphParser;
pub use yarn::YarnParser;

use crate::error::SystemError;
use crate::graph::models::DependencyGraph;
use miette::Result;
use std::collections::HashMap;
use std::path::Path;

pub type DependencyData = (Vec<(String, String, String)>, usize);

/// Scans the directory for all supported lockfiles and merges them into a unified,
/// language-agnostic Directed Graph. This provides full visibility in monorepos.
pub fn build_workspace_graph(directory: &Path) -> Result<DependencyGraph> {
    let parsers: Vec<Box<dyn GraphParser>> = vec![
        Box::new(CargoParser),
        Box::new(NpmParser),
        Box::new(PnpmParser),
        Box::new(YarnParser),
        Box::new(PoetryParser),
        Box::new(PipParser),
        Box::new(BunParser),
        Box::new(PackageJsonParser),
    ];

    let mut unified_graph = DependencyGraph::new();
    let mut lockfiles_processed = 0;

    for parser in parsers {
        if parser.can_parse(directory)
            && let Ok(subgraph) = parser.parse_graph(directory)
        {
            // To merge graphs, we map the subgraph NodeIndex to the unified NodeIndex
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
    }

    if lockfiles_processed == 0 {
        return Err(SystemError::NoLockfilesFound.into());
    }

    Ok(unified_graph)
}

/// Backward compatibility bridge for the legacy OSV scanner.
/// Flattens the new Graph structure into the original tuple format until Phase 6 refactors the OSV module.
pub fn extract_dependencies() -> Result<DependencyData> {
    let current_dir = Path::new(".");
    let graph = build_workspace_graph(current_dir)?;

    let mut all_deps = Vec::new();

    for idx in graph.node_indices() {
        if let Some(node) = graph.node_weight(idx) {
            // Map the strongly-typed ecosystem enum back to the legacy OSV strings
            let legacy_ecosystem_str = match node.ecosystem {
                crate::graph::models::Ecosystem::Cargo => "crates.io",
                crate::graph::models::Ecosystem::Pip => "PyPI",
                _ => "npm",
            };

            all_deps.push((
                node.name.clone(),
                node.version.clone(),
                legacy_ecosystem_str.to_string(),
            ));
        }
    }

    // Deduplicate flattened nodes to prevent duplicate OSV API calls
    all_deps.sort();
    all_deps.dedup();

    // Return the tuple with lockfile count mock (1 prevents NoLockfilesFound failure)
    Ok((all_deps, 1))
}
