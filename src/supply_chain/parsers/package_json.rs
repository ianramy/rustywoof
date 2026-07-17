// src/supply_chain/parsers/package_json.rs

use crate::error::SystemError;
use crate::graph::models::{DependencyGraph, Ecosystem, PackageNode};
use crate::supply_chain::parsers::traits::GraphParser;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;

pub struct PackageJsonParser;

impl GraphParser for PackageJsonParser {
    fn can_parse(&self, directory: &Path) -> bool {
        let pkg_exists = directory.join("package.json").exists();
        let no_lockfiles = !directory.join("package-lock.json").exists()
            && !directory.join("yarn.lock").exists()
            && !directory.join("pnpm-lock.yaml").exists()
            && !directory.join("bun.lock").exists()
            && !directory.join("bun.lockb").exists();

        // Only act as a fallback when NO lockfiles are present.
        pkg_exists && no_lockfiles
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let path = directory.join("package.json");
        let content = fs::read_to_string(&path).into_diagnostic()?;

        let parsed: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| SystemError::LockfileParseError {
                file_name: "package.json".to_string(),
                source: e.into(),
            })?;

        let mut graph = DependencyGraph::new();

        let deps = parsed.get("dependencies").and_then(|d| d.as_object());
        let dev_deps = parsed.get("devDependencies").and_then(|d| d.as_object());

        let mut all_deps = deps.cloned().unwrap_or_default();
        if let Some(dev) = dev_deps {
            for (k, v) in dev {
                all_deps.insert(k.clone(), v.clone());
            }
        }

        for (name, req_val) in all_deps {
            let requirement = req_val.as_str().unwrap_or("unknown").to_string();

            // Clean common semver prefixes to get a pseudo-version since we lack exact resolution
            let version = requirement
                .trim_start_matches(['^', '~', '=', '>'])
                .to_string();

            let node = PackageNode {
                name: name.clone(),
                version,
                ecosystem: Ecosystem::Npm, // Defaulting to npm ecosystem for generic package.json
                is_vulnerable: false,
            };

            // Edges are omitted because a raw package.json lacks the deep dependency tree
            graph.add_node(node);
        }

        Ok(graph)
    }
}
