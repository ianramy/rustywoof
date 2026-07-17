// src/graph/pathfinder.rs

use crate::graph::models::DependencyGraph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use std::collections::{HashSet, VecDeque};

/// Handles traversal algorithms across the dependency graph.
pub struct GraphPathfinder<'a> {
    graph: &'a DependencyGraph,
}

impl<'a> GraphPathfinder<'a> {
    /// Instantiates a new pathfinder with a reference to the parsed graph.
    pub fn new(graph: &'a DependencyGraph) -> Self {
        Self { graph }
    }

    /// Executes a Breadth-First Search (BFS) to find all acyclic paths from any root node
    /// to the specified target package name.
    ///
    /// Returns a vector of paths, where each path is a sequence of `NodeIndex` from root to target.
    pub fn find_paths_to(&self, target_name: &str) -> Vec<Vec<NodeIndex>> {
        let mut roots = Vec::new();

        // Identify all root nodes (in-degree == 0). These are typically the project's direct dependencies.
        for i in self.graph.node_indices() {
            if self.graph.edges_directed(i, Direction::Incoming).count() == 0 {
                roots.push(i);
            }
        }

        let mut successful_paths = Vec::new();

        for root in roots {
            // Queue stores the current path being evaluated
            let mut queue: VecDeque<Vec<NodeIndex>> = VecDeque::new();
            queue.push_back(vec![root]);

            let mut global_visited = HashSet::new();

            while let Some(path) = queue.pop_front() {
                // Safely get the last node in the current path.
                let current_idx = match path.last() {
                    Some(idx) => *idx,
                    None => continue,
                };

                // Check if we reached the target.
                if let Some(node) = self.graph.node_weight(current_idx)
                    && node.name == target_name
                {
                    successful_paths.push(path.clone());
                    continue; // We found the target on this path; no need to traverse deeper on this specific chain.
                }

                // Prevent redundant traversal of highly connected subgraphs
                // unless we are discovering a new, unique path to a node.
                if !global_visited.insert(current_idx) {
                    // Note: In strict shortest-path, we might skip. But to show all paths,
                    // we allow traversal if the path itself does not contain a cycle.
                }

                // Traverse children
                for neighbor in self
                    .graph
                    .neighbors_directed(current_idx, Direction::Outgoing)
                {
                    // Prevent infinite loops in cyclic dependencies (e.g., bad npm packages)
                    if !path.contains(&neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        queue.push_back(new_path);
                    }
                }
            }
        }

        successful_paths
    }
}
