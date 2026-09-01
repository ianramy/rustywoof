// src/graph/traverse_dependency_paths.rs

use crate::graph::define_dependency_models::DependencyGraph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use std::collections::{HashSet, VecDeque};

pub struct GraphPathfinder<'a> {
    graph: &'a DependencyGraph,
}

impl<'a> GraphPathfinder<'a> {
    pub fn new(graph: &'a DependencyGraph) -> Self {
        Self { graph }
    }

    pub fn get_workspace_roots(&self) -> Vec<NodeIndex> {
        let mut roots = Vec::new();
        for i in self.graph.node_indices() {
            if self.graph.edges_directed(i, Direction::Incoming).count() == 0 {
                roots.push(i);
            }
        }
        roots
    }

    pub fn get_direct_dependencies(&self, root: NodeIndex) -> Vec<NodeIndex> {
        self.graph
            .neighbors_directed(root, Direction::Outgoing)
            .collect()
    }

    pub fn find_paths_to(&self, target_name: &str) -> Vec<Vec<NodeIndex>> {
        let roots = self.get_workspace_roots();
        let mut successful_paths = Vec::new();

        for root in roots {
            let mut queue: VecDeque<Vec<NodeIndex>> = VecDeque::new();
            queue.push_back(vec![root]);

            let mut global_visited = HashSet::new();

            while let Some(path) = queue.pop_front() {
                let Some(current_idx) = path.last().copied() else {
                    continue;
                };

                if let Some(node) = self.graph.node_weight(current_idx)
                    && node.name == target_name
                {
                    successful_paths.push(path.clone());
                    continue;
                }

                let _ = global_visited.insert(current_idx);

                for neighbor in self
                    .graph
                    .neighbors_directed(current_idx, Direction::Outgoing)
                {
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
