// src/supply_chain/parsers/traits.rs

use crate::graph::models::DependencyGraph;
use miette::Result;
use std::path::Path;

/// Defines the contract for parsing a lockfile into a Directed Graph.
pub trait GraphParser {
    /// Checks if the lockfile specific to this ecosystem exists in the given directory.
    fn can_parse(&self, directory: &Path) -> bool;

    /// Parses the lockfile and returns a constructed `DependencyGraph`.
    ///
    /// # Errors
    /// Returns a `miette::Result` containing a `SystemError::LockfileParseError`
    /// if the file is malformed or cannot be read.
    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph>;
}
