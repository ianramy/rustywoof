// src/graph/models.rs

use std::fmt;

/// Identifies the target supply chain ecosystem.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ecosystem {
    Cargo,
    Npm,
    Pnpm,
    Yarn,
    Bun,
    Pip,
}

impl fmt::Display for Ecosystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ecosystem::Cargo => "Cargo",
            Ecosystem::Npm => "npm",
            Ecosystem::Pnpm => "pnpm",
            Ecosystem::Yarn => "Yarn",
            Ecosystem::Bun => "bun",
            Ecosystem::Pip => "PyPI",
        };
        write!(f, "{}", s)
    }
}

/// Represents a unique, resolved package version within the dependency graph.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PackageNode {
    /// The canonical name of the package (e.g., "serde").
    pub name: String,
    /// The exact resolved version string (e.g., "1.0.197").
    pub version: String,
    /// The ecosystem this package belongs to.
    pub ecosystem: Ecosystem,
    /// Indicates if this node is a known vulnerable version.
    pub is_vulnerable: bool,
}

/// Specifies how the dependency is integrated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Runtime,
    Development,
    Build,
    Peer,
    Optional,
}

/// Represents the relationship requirement between a parent and child package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyEdge {
    /// The version constraint defined in the parent's manifest.
    pub requirement: String,
    /// The classification of the dependency.
    pub kind: DependencyKind,
}

/// The core Directed Graph representing the project's supply chain tree.
pub type DependencyGraph = petgraph::graph::DiGraph<PackageNode, DependencyEdge>;
