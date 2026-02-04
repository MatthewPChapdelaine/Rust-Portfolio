use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::{Context, Result, anyhow};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;
use colored::Colorize;

use crate::models::{Manifest, ResolvedPackage};
use crate::registry::Registry;
use crate::lockfile::Lockfile;

pub fn resolve_dependencies(manifest: &Manifest, registry: &Registry) -> Result<Vec<ResolvedPackage>> {
    let mut resolved = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for (name, version) in &manifest.dependencies {
        queue.push_back((name.clone(), version.clone()));
    }

    while let Some((name, version_req)) = queue.pop_front() {
        let key = format!("{}@{}", name, version_req);
        
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);

        let package = registry.get_package(&name, &version_req)
            .context(format!("Failed to resolve: {} {}", name, version_req))?;

        let version = semver::Version::parse(&package.version)?;
        let mut deps = Vec::new();

        for (dep_name, dep_version) in &package.dependencies {
            deps.push(dep_name.clone());
            queue.push_back((dep_name.clone(), dep_version.clone()));
        }

        resolved.push(ResolvedPackage {
            name: name.clone(),
            version,
            dependencies: deps,
        });
    }

    check_for_cycles(&resolved)?;
    Ok(resolved)
}

fn check_for_cycles(packages: &[ResolvedPackage]) -> Result<()> {
    let mut graph = HashMap::new();
    let mut all_nodes = HashSet::new();

    for pkg in packages {
        all_nodes.insert(pkg.name.clone());
        graph.insert(pkg.name.clone(), pkg.dependencies.clone());
    }

    for start in &all_nodes {
        let mut visited = HashSet::new();
        let mut stack = vec![start.clone()];

        while let Some(node) = stack.pop() {
            if !visited.insert(node.clone()) {
                return Err(anyhow!("Circular dependency detected involving: {}", node));
            }

            if let Some(deps) = graph.get(&node) {
                for dep in deps {
                    if all_nodes.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn build_dependency_graph(lockfile: &Lockfile) -> Result<DiGraph<String, ()>> {
    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();

    for pkg in &lockfile.packages {
        let label = format!("{} v{}", pkg.name, pkg.version);
        let node = graph.add_node(label.clone());
        nodes.insert(pkg.name.clone(), node);
    }

    for pkg in &lockfile.packages {
        if let Some(&from_node) = nodes.get(&pkg.name) {
            for dep in &pkg.dependencies {
                if let Some(&to_node) = nodes.get(dep) {
                    graph.add_edge(from_node, to_node, ());
                }
            }
        }
    }

    Ok(graph)
}

pub fn print_dependency_tree(graph: &DiGraph<String, ()>) -> Result<()> {
    let root_nodes: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|&n| graph.neighbors_directed(n, petgraph::Direction::Incoming).count() == 0)
        .collect();

    if root_nodes.is_empty() {
        println!("  {}", "No dependencies".yellow());
        return Ok(());
    }

    for root in root_nodes {
        print_node(graph, root, 0, &mut HashSet::new());
    }

    Ok(())
}

fn print_node(
    graph: &DiGraph<String, ()>,
    node: NodeIndex,
    depth: usize,
    visited: &mut HashSet<NodeIndex>,
) {
    let indent = "  ".repeat(depth);
    let node_label = &graph[node];

    if visited.contains(&node) {
        println!("{}├─ {} {}", indent, node_label.cyan(), "(*)".yellow());
        return;
    }

    visited.insert(node);

    if depth == 0 {
        println!("{}├─ {}", indent, node_label.green().bold());
    } else {
        println!("{}├─ {}", indent, node_label.cyan());
    }

    let children: Vec<NodeIndex> = graph
        .neighbors_directed(node, petgraph::Direction::Outgoing)
        .collect();

    for child in children {
        print_node(graph, child, depth + 1, visited);
    }
}
