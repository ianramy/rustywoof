// src/graph/define_dependency_models.rs

use std::fmt;

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PackageNode {
    pub name: String,
    pub version: String,
    pub ecosystem: Ecosystem,
    pub is_vulnerable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Runtime,
    Development,
    Build,
    Peer,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyEdge {
    pub requirement: String,
    pub kind: DependencyKind,
}

pub type DependencyGraph = petgraph::graph::DiGraph<PackageNode, DependencyEdge>;
