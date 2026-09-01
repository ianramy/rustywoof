// src/audit/parsers/traits.rs

use crate::graph::define_dependency_models::DependencyGraph;
use miette::Result;
use std::path::Path;

pub trait GraphParser {
    fn can_parse(&self, directory: &Path) -> bool;
    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph>;
}
