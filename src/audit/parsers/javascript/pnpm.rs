// src/audit/parsers/javascript/pnpm.rs

use crate::audit::parsers::traits::GraphParser;
use crate::graph::define_dependency_models::{
    DependencyEdge, DependencyGraph, DependencyKind, Ecosystem, PackageNode,
};
use miette::{Result, miette};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct PnpmParser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Section {
    None,
    RootDeps,
    Importers,
    Packages,
    Snapshots,
}

fn strip_quotes(s: &str) -> &str {
    s.trim_matches(['\'', '"'])
}

fn strip_peer_suffix(raw: &str) -> &str {
    raw.split('(').next().unwrap_or(raw).trim()
}

fn split_name_version(clean: &str) -> (&str, &str) {
    if let Some(idx) = clean.rfind('@') {
        if idx > 0 {
            (&clean[..idx], &clean[idx + 1..])
        } else if let Some(idx2) = clean.rfind('/') {
            (&clean[..idx2], &clean[idx2 + 1..])
        } else {
            (clean, "")
        }
    } else if let Some(idx) = clean.rfind('/') {
        (&clean[..idx], &clean[idx + 1..])
    } else {
        (clean, "")
    }
}

fn clean_header_key(raw_key: &str) -> String {
    strip_peer_suffix(strip_quotes(raw_key)).to_string()
}

fn clean_dependency_value(raw: &str) -> String {
    let raw_val = strip_peer_suffix(raw);
    let mut clean_val = raw_val.split("||").next().unwrap_or(raw_val).trim();
    clean_val = clean_val.split(' ').next().unwrap_or(clean_val).trim();
    clean_val = clean_val.trim_start_matches(['^', '~', '=', '>', '<', 'v']);
    if clean_val.is_empty() {
        "*".to_string()
    } else {
        clean_val.to_string()
    }
}

fn split_kv(trimmed: &str) -> Option<(&str, &str)> {
    if !trimmed.contains(':') {
        return None;
    }
    let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }
    let key = strip_quotes(parts[0].trim());
    let val = strip_quotes(parts[1].trim());
    Some((key, val))
}

fn get_or_create_node(
    graph: &mut DependencyGraph,
    node_map: &mut HashMap<String, NodeIndex>,
    key: &str,
    name: &str,
    version: &str,
) -> NodeIndex {
    if let Some(&idx) = node_map.get(key) {
        idx
    } else {
        let idx = graph.add_node(PackageNode {
            name: name.to_string(),
            version: version.to_string(),
            ecosystem: Ecosystem::Pnpm,
            is_vulnerable: false,
        });
        node_map.insert(key.to_string(), idx);
        idx
    }
}

impl GraphParser for PnpmParser {
    fn can_parse(&self, directory: &Path) -> bool {
        directory.join("pnpm-lock.yaml").exists()
    }

    fn parse_graph(&self, directory: &Path) -> Result<DependencyGraph> {
        let lock_path = directory.join("pnpm-lock.yaml");
        let content = fs::read_to_string(&lock_path)
            .map_err(|e| miette!("Failed to read pnpm-lock.yaml: {}", e))?;

        let mut graph = DependencyGraph::new();
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

        let project_name = directory
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut section = Section::None;
        let mut current_kind = DependencyKind::Runtime;

        let mut root_node_idx: Option<NodeIndex> = None;

        let mut current_package_key = String::new();
        let mut in_pkg_deps = false;

        let mut importer_roots: HashMap<String, NodeIndex> = HashMap::new();
        let mut current_importer_idx: Option<NodeIndex> = None;
        let mut in_importer_deps = false;
        let mut pending_importer_dep: Option<String> = None;

        let mut current_snapshot_idx: Option<NodeIndex> = None;
        let mut in_snapshot_deps = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let indent = line.chars().take_while(|c| *c == ' ').count();

            if indent == 0 {
                section = if line.starts_with("importers:") {
                    Section::Importers
                } else if line.starts_with("packages:") {
                    Section::Packages
                } else if line.starts_with("snapshots:") {
                    Section::Snapshots
                } else if line.starts_with("devDependencies:") {
                    current_kind = DependencyKind::Development;
                    Section::RootDeps
                } else if line.starts_with("optionalDependencies:") {
                    current_kind = DependencyKind::Optional;
                    Section::RootDeps
                } else if line.starts_with("dependencies:") {
                    current_kind = DependencyKind::Runtime;
                    Section::RootDeps
                } else {
                    Section::None
                };

                in_pkg_deps = false;
                in_importer_deps = false;
                in_snapshot_deps = false;
                pending_importer_dep = None;
                continue;
            }

            match section {
                Section::RootDeps => {
                    if indent != 2 {
                        continue;
                    }
                    let Some((dep_name, dep_val)) = split_kv(trimmed) else {
                        continue;
                    };
                    if dep_val.is_empty() || dep_val == "{" {
                        continue;
                    }

                    let raw_val = strip_peer_suffix(dep_val);
                    let clean_val = clean_dependency_value(raw_val);
                    let dep_key = format!("{}@{}", dep_name, clean_val);
                    let target_idx = get_or_create_node(
                        &mut graph,
                        &mut node_map,
                        &dep_key,
                        dep_name,
                        &clean_val,
                    );

                    let source_idx = *root_node_idx.get_or_insert_with(|| {
                        let idx = graph.add_node(PackageNode {
                            name: project_name.clone(),
                            version: "0.1.0".to_string(),
                            ecosystem: Ecosystem::Pnpm,
                            is_vulnerable: false,
                        });
                        node_map.insert(format!("{}@0.1.0", project_name), idx);
                        idx
                    });

                    graph.add_edge(
                        source_idx,
                        target_idx,
                        DependencyEdge {
                            requirement: raw_val.to_string(),
                            kind: current_kind,
                        },
                    );
                }

                Section::Importers => {
                    if indent == 2 && trimmed.ends_with(':') {
                        let path = strip_quotes(trimmed.trim_end_matches(':'));
                        let name = if path == "." {
                            project_name.clone()
                        } else {
                            path.to_string()
                        };
                        let key = format!("importer:{}", path);
                        let idx = *importer_roots.entry(key).or_insert_with(|| {
                            graph.add_node(PackageNode {
                                name,
                                version: "0.1.0".to_string(),
                                ecosystem: Ecosystem::Pnpm,
                                is_vulnerable: false,
                            })
                        });
                        current_importer_idx = Some(idx);
                        if path == "." {
                            root_node_idx = Some(idx);
                        }
                        in_importer_deps = false;
                        pending_importer_dep = None;
                        continue;
                    }

                    if indent == 4 {
                        in_importer_deps = line.trim_start().starts_with("dependencies:")
                            || line.trim_start().starts_with("devDependencies:")
                            || line.trim_start().starts_with("optionalDependencies:");
                        current_kind = if trimmed.starts_with("devDependencies:") {
                            DependencyKind::Development
                        } else if trimmed.starts_with("optionalDependencies:") {
                            DependencyKind::Optional
                        } else {
                            DependencyKind::Runtime
                        };
                        pending_importer_dep = None;
                        continue;
                    }

                    if !in_importer_deps {
                        continue;
                    }

                    let Some(source_idx) = current_importer_idx else {
                        continue;
                    };

                    if indent == 6 {
                        match split_kv(trimmed) {
                            Some((dep_name, "")) => {
                                pending_importer_dep = Some(dep_name.to_string());
                            }
                            Some((dep_name, dep_val)) if !dep_val.is_empty() => {
                                let clean_val = clean_dependency_value(dep_val);
                                let dep_key = format!("{}@{}", dep_name, clean_val);
                                let target_idx = get_or_create_node(
                                    &mut graph,
                                    &mut node_map,
                                    &dep_key,
                                    dep_name,
                                    &clean_val,
                                );
                                graph.add_edge(
                                    source_idx,
                                    target_idx,
                                    DependencyEdge {
                                        requirement: clean_val,
                                        kind: current_kind,
                                    },
                                );
                                pending_importer_dep = None;
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if indent == 8 {
                        let Some(dep_name) = pending_importer_dep.clone() else {
                            continue;
                        };
                        if let Some(("version", version_val)) = split_kv(trimmed) {
                            let clean_val = clean_dependency_value(version_val);
                            let dep_key = format!("{}@{}", dep_name, clean_val);
                            let target_idx = get_or_create_node(
                                &mut graph,
                                &mut node_map,
                                &dep_key,
                                &dep_name,
                                &clean_val,
                            );
                            graph.add_edge(
                                source_idx,
                                target_idx,
                                DependencyEdge {
                                    requirement: clean_val,
                                    kind: current_kind,
                                },
                            );
                            pending_importer_dep = None;
                        }
                    }
                }

                Section::Packages => {
                    if indent == 2 && trimmed.ends_with(':') {
                        in_pkg_deps = false;

                        let raw_key = trimmed.trim_end_matches(':').trim_start_matches('/');
                        let clean_key = clean_header_key(raw_key);
                        let (name, version) = split_name_version(&clean_key);

                        if !version.is_empty() {
                            get_or_create_node(
                                &mut graph,
                                &mut node_map,
                                &clean_key,
                                name,
                                version,
                            );
                        }
                        current_package_key = clean_key;
                        continue;
                    }

                    if indent == 4 {
                        in_pkg_deps = trimmed.starts_with("dependencies:")
                            || trimmed.starts_with("optionalDependencies:");
                        continue;
                    }

                    if indent == 6 && in_pkg_deps {
                        let Some((dep_name, dep_val)) = split_kv(trimmed) else {
                            continue;
                        };
                        if dep_val.is_empty() || dep_val == "{" {
                            continue;
                        }

                        let raw_val = strip_peer_suffix(dep_val);
                        let clean_val = clean_dependency_value(raw_val);
                        let dep_key = format!("{}@{}", dep_name, clean_val);
                        let target_idx = get_or_create_node(
                            &mut graph,
                            &mut node_map,
                            &dep_key,
                            dep_name,
                            &clean_val,
                        );

                        if let Some(&source_idx) = node_map.get(&current_package_key) {
                            graph.add_edge(
                                source_idx,
                                target_idx,
                                DependencyEdge {
                                    requirement: raw_val.to_string(),
                                    kind: DependencyKind::Runtime,
                                },
                            );
                        }
                    }
                }

                Section::Snapshots => {
                    if indent == 2 && trimmed.ends_with(':') {
                        in_snapshot_deps = false;

                        let raw_key = trimmed.trim_end_matches(':');
                        let clean_key = clean_header_key(raw_key);
                        let (name, version) = split_name_version(&clean_key);

                        let idx = get_or_create_node(
                            &mut graph,
                            &mut node_map,
                            &clean_key,
                            name,
                            version,
                        );
                        current_snapshot_idx = Some(idx);
                        continue;
                    }

                    if indent == 4 {
                        in_snapshot_deps = trimmed.starts_with("dependencies:")
                            || trimmed.starts_with("optionalDependencies:");
                        continue;
                    }

                    if indent == 6 && in_snapshot_deps {
                        let Some(source_idx) = current_snapshot_idx else {
                            continue;
                        };
                        let Some((dep_name, dep_val)) = split_kv(trimmed) else {
                            continue;
                        };
                        if dep_val.is_empty() {
                            continue;
                        }

                        let clean_val = clean_dependency_value(dep_val);
                        let dep_key = format!("{}@{}", dep_name, clean_val);
                        let target_idx = get_or_create_node(
                            &mut graph,
                            &mut node_map,
                            &dep_key,
                            dep_name,
                            &clean_val,
                        );

                        graph.add_edge(
                            source_idx,
                            target_idx,
                            DependencyEdge {
                                requirement: clean_val,
                                kind: DependencyKind::Runtime,
                            },
                        );
                    }
                }

                Section::None => {}
            }
        }

        Ok(graph)
    }
}
