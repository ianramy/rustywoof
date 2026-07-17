// src/graph/analyzer.rs

use crate::graph::pathfinder::GraphPathfinder;
use crate::graph::visualizer::GraphVisualizer;
use crate::supply_chain::parsers::build_workspace_graph;
use miette::Result;
use std::path::Path;

/// Orchestrates the graph building, pathfinding, and terminal rendering pipeline.
pub fn execute_sniff(target_package: &str) -> Result<()> {
    let current_dir = Path::new(".");

    // Assemble the unified graph across all detected ecosystems in the workspace
    let graph = build_workspace_graph(current_dir)?;

    // Execute cycle-resistant Breadth-First Search
    let pathfinder = GraphPathfinder::new(&graph);
    let paths = pathfinder.find_paths_to(target_package);

    // Pass the discovered raw paths to the visualizer for UI tree construction
    GraphVisualizer::print_sniff_tree(&graph, &paths, target_package)?;

    Ok(())
}
