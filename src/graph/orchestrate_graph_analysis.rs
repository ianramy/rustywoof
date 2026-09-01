// src/graph/orchestrate_graph_analysis.rs

use crate::audit::parsers::build_workspace_graph;
use crate::graph::render_dependency_tree::GraphVisualizer;
use crate::graph::traverse_dependency_paths::GraphPathfinder;
use miette::Result;
use std::path::Path;

pub fn execute_sniff(target_package: Option<&str>, working_dir: &Path) -> Result<()> {
    let graph = build_workspace_graph(working_dir)?;

    let pathfinder = GraphPathfinder::new(&graph);

    match target_package {
        Some(target) => {
            let paths = pathfinder.find_paths_to(target);
            GraphVisualizer::print_sniff_tree(&graph, &paths, target)?;
        }
        None => {
            GraphVisualizer::print_workspace_list(&graph, &pathfinder)?;
        }
    }

    Ok(())
}
