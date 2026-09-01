// src/audit/remediation/resolve_remediation_context.rs

use crate::graph::define_dependency_models::Ecosystem;
use miette::Result;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct RemediationCommand {
    pub binary: String,
    pub args: Vec<String>,
}

pub fn resolve_context(package: &str, target_version: &str) -> Result<RemediationCommand> {
    if package.starts_with('-') || target_version.starts_with('-') {
        miette::bail!("Invalid package or version: cannot start with a hyphen.");
    }

    let current_dir = Path::new(".");
    let graph = crate::audit::parsers::build_workspace_graph(current_dir)?;

    let mut found_ecosystem = None;
    for idx in graph.node_indices() {
        if let Some(node) = graph.node_weight(idx)
            && node.name == package
        {
            found_ecosystem = Some(node.ecosystem);
            break;
        }
    }

    let ecosystem = found_ecosystem.ok_or_else(|| {
        miette::miette!(
            "Package '{}' not found in local lockfiles. Cannot infer ecosystem.",
            package
        )
    })?;

    let package_target = format!("{}@{}", package, target_version);

    let (binary, action) = match ecosystem {
        Ecosystem::Npm => ("npm", "install"),
        Ecosystem::Cargo => ("cargo", "add"),
        Ecosystem::Yarn => ("yarn", "add"),
        Ecosystem::Pnpm => ("pnpm", "add"),
        Ecosystem::Bun => ("bun", "add"),
        Ecosystem::Pip => ("pip", "install"),
    };

    Ok(RemediationCommand {
        binary: binary.to_string(),
        args: vec![action.to_string(), package_target],
    })
}
