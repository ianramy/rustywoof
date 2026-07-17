// src/supply_chain/parsers/bun.rs

use crate::error::SystemError;
use crate::graph::models::DependencyGraph;
use crate::supply_chain::parsers::traits::GraphParser;
use miette::Result;
use std::path::Path;

pub struct BunParser;

impl GraphParser for BunParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("bun.lock").exists()
    }

    fn parse_graph(&self, _directory: &Path) -> Result<DependencyGraph> {
        Err(SystemError::LockfileParseError {
            file_name: "bun.lock".to_string(),
            source: std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Bun text lockfile support requires shared Yarn engine",
            )
            .into(),
        }
        .into())
    }
}
