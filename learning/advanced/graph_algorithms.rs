/*!
 * Graph Algorithms
 * 
 * Implementation of essential graph algorithms:
 * - Dijkstra's shortest path
 * - Breadth-First Search (BFS)
 * - Depth-First Search (DFS)
 * - Topological Sort
 * - Cycle Detection
 * - Connected Components
 * 
 * # Compile and Run
 * ```bash
 * rustc graph_algorithms.rs -o graph_algorithms
 * ./graph_algorithms
 * ```
 */

use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;
use std::fmt;

// ============================================================================
// Graph Data Structures
// ============================================================================

/// Edge with weight
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub to: usize,
    pub weight: i32,
}

/// Graph representation using adjacency list
#[derive(Debug, Clone)]
pub struct Graph {
    adj_list: Vec<Vec<Edge>>,
    num_vertices: usize,
}

impl Graph {
    /// Create a new graph with n vertices
    pub fn new(n: usize) -> Self {
        Graph {
            adj_list: vec![Vec::new(); n],
            num_vertices: n,
        }
    }

    /// Add a directed edge
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        if from < self.num_vertices && to < self.num_vertices {
            self.adj_list[from].push(Edge { to, weight });
        }
    }

    /// Add an undirected edge
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.add_edge(u, v, weight);
        self.add_edge(v, u, weight);
    }

    /// Get neighbors of a vertex
    pub fn neighbors(&self, vertex: usize) -> &[Edge] {
        &self.adj_list[vertex]
    }

    /// Get number of vertices
    pub fn size(&self) -> usize {
        self.num_vertices
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Graph with {} vertices:", self.num_vertices)?;
        for (i, edges) in self.adj_list.iter().enumerate() {
            write!(f, "  {} -> ", i)?;
            for edge in edges {
                write!(f, "{}(w:{}) ", edge.to, edge.weight)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// ============================================================================
// Dijkstra's Algorithm
// ============================================================================

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra's shortest path algorithm
/// Returns distances and predecessors for path reconstruction
pub fn dijkstra(graph: &Graph, start: usize) -> (Vec<Option<i32>>, Vec<Option<usize>>) {
    let n = graph.size();
    let mut dist = vec![None; n];
    let mut prev = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[start] = Some(0);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if let Some(d) = dist[position] {
            if cost > d {
                continue;
            }
        }

        for edge in graph.neighbors(position) {
            let next_cost = cost + edge.weight;
            
            if dist[edge.to].is_none() || next_cost < dist[edge.to].unwrap() {
                dist[edge.to] = Some(next_cost);
                prev[edge.to] = Some(position);
                heap.push(State { cost: next_cost, position: edge.to });
            }
        }
    }

    (dist, prev)
}

/// Reconstruct path from Dijkstra's predecessors
pub fn reconstruct_path(prev: &[Option<usize>], start: usize, end: usize) -> Option<Vec<usize>> {
    let mut path = Vec::new();
    let mut current = end;

    while current != start {
        path.push(current);
        current = prev[current]?;
    }
    path.push(start);
    path.reverse();

    Some(path)
}

// ============================================================================
// Breadth-First Search (BFS)
// ============================================================================

/// BFS traversal returning visit order
pub fn bfs(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.size()];
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        order.push(vertex);

        for edge in graph.neighbors(vertex) {
            if !visited[edge.to] {
                visited[edge.to] = true;
                queue.push_back(edge.to);
            }
        }
    }

    order
}

/// BFS shortest path (unweighted)
pub fn bfs_shortest_path(graph: &Graph, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut visited = vec![false; graph.size()];
    let mut queue = VecDeque::new();
    let mut prev = vec![None; graph.size()];

    visited[start] = true;
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        if vertex == end {
            return reconstruct_path(&prev, start, end);
        }

        for edge in graph.neighbors(vertex) {
            if !visited[edge.to] {
                visited[edge.to] = true;
                prev[edge.to] = Some(vertex);
                queue.push_back(edge.to);
            }
        }
    }

    None
}

// ============================================================================
// Depth-First Search (DFS)
// ============================================================================

/// DFS traversal (iterative)
pub fn dfs_iterative(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.size()];
    let mut stack = vec![start];
    let mut order = Vec::new();

    while let Some(vertex) = stack.pop() {
        if visited[vertex] {
            continue;
        }

        visited[vertex] = true;
        order.push(vertex);

        for edge in graph.neighbors(vertex).iter().rev() {
            if !visited[edge.to] {
                stack.push(edge.to);
            }
        }
    }

    order
}

/// DFS traversal (recursive)
pub fn dfs_recursive(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.size()];
    let mut order = Vec::new();
    
    fn dfs_helper(
        graph: &Graph,
        vertex: usize,
        visited: &mut Vec<bool>,
        order: &mut Vec<usize>,
    ) {
        visited[vertex] = true;
        order.push(vertex);

        for edge in graph.neighbors(vertex) {
            if !visited[edge.to] {
                dfs_helper(graph, edge.to, visited, order);
            }
        }
    }

    dfs_helper(graph, start, &mut visited, &mut order);
    order
}

// ============================================================================
// Topological Sort
// ============================================================================

/// Topological sort using DFS (Kahn's algorithm alternative)
pub fn topological_sort(graph: &Graph) -> Option<Vec<usize>> {
    let n = graph.size();
    let mut visited = vec![false; n];
    let mut stack = Vec::new();
    let mut rec_stack = vec![false; n];

    fn visit(
        graph: &Graph,
        vertex: usize,
        visited: &mut Vec<bool>,
        rec_stack: &mut Vec<bool>,
        stack: &mut Vec<usize>,
    ) -> bool {
        if rec_stack[vertex] {
            return false; // Cycle detected
        }
        if visited[vertex] {
            return true;
        }

        visited[vertex] = true;
        rec_stack[vertex] = true;

        for edge in graph.neighbors(vertex) {
            if !visit(graph, edge.to, visited, rec_stack, stack) {
                return false;
            }
        }

        rec_stack[vertex] = false;
        stack.push(vertex);
        true
    }

    for i in 0..n {
        if !visited[i] {
            if !visit(graph, i, &mut visited, &mut rec_stack, &mut stack) {
                return None; // Graph has cycle
            }
        }
    }

    stack.reverse();
    Some(stack)
}

/// Topological sort using Kahn's algorithm (in-degree based)
pub fn topological_sort_kahn(graph: &Graph) -> Option<Vec<usize>> {
    let n = graph.size();
    let mut in_degree = vec![0; n];
    
    // Calculate in-degrees
    for i in 0..n {
        for edge in graph.neighbors(i) {
            in_degree[edge.to] += 1;
        }
    }

    // Queue vertices with in-degree 0
    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &deg)| deg == 0)
        .map(|(i, _)| i)
        .collect();

    let mut result = Vec::new();

    while let Some(vertex) = queue.pop_front() {
        result.push(vertex);

        for edge in graph.neighbors(vertex) {
            in_degree[edge.to] -= 1;
            if in_degree[edge.to] == 0 {
                queue.push_back(edge.to);
            }
        }
    }

    if result.len() == n {
        Some(result)
    } else {
        None // Graph has cycle
    }
}

// ============================================================================
// Cycle Detection
// ============================================================================

/// Detect cycle in directed graph
pub fn has_cycle_directed(graph: &Graph) -> bool {
    let n = graph.size();
    let mut visited = vec![false; n];
    let mut rec_stack = vec![false; n];

    fn dfs_cycle(
        graph: &Graph,
        vertex: usize,
        visited: &mut Vec<bool>,
        rec_stack: &mut Vec<bool>,
    ) -> bool {
        visited[vertex] = true;
        rec_stack[vertex] = true;

        for edge in graph.neighbors(vertex) {
            if !visited[edge.to] {
                if dfs_cycle(graph, edge.to, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack[edge.to] {
                return true;
            }
        }

        rec_stack[vertex] = false;
        false
    }

    for i in 0..n {
        if !visited[i] && dfs_cycle(graph, i, &mut visited, &mut rec_stack) {
            return true;
        }
    }

    false
}

/// Detect cycle in undirected graph
pub fn has_cycle_undirected(graph: &Graph) -> bool {
    let n = graph.size();
    let mut visited = vec![false; n];

    fn dfs_cycle(
        graph: &Graph,
        vertex: usize,
        parent: Option<usize>,
        visited: &mut Vec<bool>,
    ) -> bool {
        visited[vertex] = true;

        for edge in graph.neighbors(vertex) {
            if !visited[edge.to] {
                if dfs_cycle(graph, edge.to, Some(vertex), visited) {
                    return true;
                }
            } else if Some(edge.to) != parent {
                return true;
            }
        }

        false
    }

    for i in 0..n {
        if !visited[i] && dfs_cycle(graph, i, None, &mut visited) {
            return true;
        }
    }

    false
}

// ============================================================================
// Connected Components
// ============================================================================

/// Find all connected components in undirected graph
pub fn connected_components(graph: &Graph) -> Vec<Vec<usize>> {
    let n = graph.size();
    let mut visited = vec![false; n];
    let mut components = Vec::new();

    for i in 0..n {
        if !visited[i] {
            let mut component = Vec::new();
            let mut stack = vec![i];

            while let Some(vertex) = stack.pop() {
                if visited[vertex] {
                    continue;
                }

                visited[vertex] = true;
                component.push(vertex);

                for edge in graph.neighbors(vertex) {
                    if !visited[edge.to] {
                        stack.push(edge.to);
                    }
                }
            }

            components.push(component);
        }
    }

    components
}

// ============================================================================
// Demonstrations
// ============================================================================

fn demo_dijkstra() {
    println!("\n{:=^60}", " DIJKSTRA'S ALGORITHM ");
    
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1, 4);
    graph.add_edge(0, 2, 2);
    graph.add_edge(1, 2, 1);
    graph.add_edge(1, 3, 5);
    graph.add_edge(2, 3, 8);
    graph.add_edge(2, 4, 10);
    graph.add_edge(3, 4, 2);
    graph.add_edge(3, 5, 6);
    graph.add_edge(4, 5, 3);

    println!("{}", graph);

    let (dist, prev) = dijkstra(&graph, 0);
    
    println!("Shortest distances from vertex 0:");
    for (i, d) in dist.iter().enumerate() {
        match d {
            Some(distance) => {
                print!("  Vertex {}: distance = {}", i, distance);
                if let Some(path) = reconstruct_path(&prev, 0, i) {
                    print!(", path = {:?}", path);
                }
                println!();
            }
            None => println!("  Vertex {}: unreachable", i),
        }
    }
}

fn demo_bfs_dfs() {
    println!("\n{:=^60}", " BFS & DFS ");
    
    let mut graph = Graph::new(7);
    graph.add_edge(0, 1, 1);
    graph.add_edge(0, 2, 1);
    graph.add_edge(1, 3, 1);
    graph.add_edge(1, 4, 1);
    graph.add_edge(2, 5, 1);
    graph.add_edge(2, 6, 1);

    println!("BFS from vertex 0: {:?}", bfs(&graph, 0));
    println!("DFS (iterative) from vertex 0: {:?}", dfs_iterative(&graph, 0));
    println!("DFS (recursive) from vertex 0: {:?}", dfs_recursive(&graph, 0));

    if let Some(path) = bfs_shortest_path(&graph, 0, 6) {
        println!("Shortest path from 0 to 6: {:?}", path);
    }
}

fn demo_topological_sort() {
    println!("\n{:=^60}", " TOPOLOGICAL SORT ");
    
    let mut graph = Graph::new(6);
    // DAG: task dependencies
    graph.add_edge(5, 2, 1); // Task 5 before 2
    graph.add_edge(5, 0, 1); // Task 5 before 0
    graph.add_edge(4, 0, 1); // Task 4 before 0
    graph.add_edge(4, 1, 1); // Task 4 before 1
    graph.add_edge(2, 3, 1); // Task 2 before 3
    graph.add_edge(3, 1, 1); // Task 3 before 1

    println!("{}", graph);

    match topological_sort(&graph) {
        Some(order) => println!("Topological order (DFS): {:?}", order),
        None => println!("Graph has a cycle!"),
    }

    match topological_sort_kahn(&graph) {
        Some(order) => println!("Topological order (Kahn): {:?}", order),
        None => println!("Graph has a cycle!"),
    }
}

fn demo_cycle_detection() {
    println!("\n{:=^60}", " CYCLE DETECTION ");
    
    // Directed graph with cycle
    let mut graph1 = Graph::new(4);
    graph1.add_edge(0, 1, 1);
    graph1.add_edge(1, 2, 1);
    graph1.add_edge(2, 3, 1);
    graph1.add_edge(3, 1, 1); // Creates cycle

    println!("Directed graph:");
    println!("{}", graph1);
    println!("Has cycle: {}", has_cycle_directed(&graph1));

    // Directed acyclic graph
    let mut graph2 = Graph::new(4);
    graph2.add_edge(0, 1, 1);
    graph2.add_edge(0, 2, 1);
    graph2.add_edge(1, 3, 1);
    graph2.add_edge(2, 3, 1);

    println!("\nDirected acyclic graph:");
    println!("{}", graph2);
    println!("Has cycle: {}", has_cycle_directed(&graph2));
}

fn demo_connected_components() {
    println!("\n{:=^60}", " CONNECTED COMPONENTS ");
    
    let mut graph = Graph::new(9);
    // Component 1
    graph.add_undirected_edge(0, 1, 1);
    graph.add_undirected_edge(1, 2, 1);
    // Component 2
    graph.add_undirected_edge(3, 4, 1);
    graph.add_undirected_edge(4, 5, 1);
    graph.add_undirected_edge(5, 3, 1);
    // Component 3
    graph.add_undirected_edge(6, 7, 1);
    // Isolated vertex 8

    println!("{}", graph);

    let components = connected_components(&graph);
    println!("Found {} connected components:", components.len());
    for (i, comp) in components.iter().enumerate() {
        println!("  Component {}: {:?}", i + 1, comp);
    }
}

fn main() {
    println!("🔷 Graph Algorithms in Rust 🔷\n");
    
    demo_dijkstra();
    demo_bfs_dfs();
    demo_topological_sort();
    demo_cycle_detection();
    demo_connected_components();
    
    println!("\n{:=^60}", " COMPLETE ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, 3);
        assert_eq!(g.size(), 3);
        assert_eq!(g.neighbors(0).len(), 1);
    }

    #[test]
    fn test_bfs() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, 1);
        let order = bfs(&g, 0);
        assert_eq!(order.len(), 4);
        assert_eq!(order[0], 0);
    }

    #[test]
    fn test_cycle_detection() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 0, 1);
        assert!(has_cycle_directed(&g));
    }

    #[test]
    fn test_topological_sort() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        let result = topological_sort(&g);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![0, 1, 2]);
    }
}
