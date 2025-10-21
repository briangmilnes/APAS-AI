//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: TSP 2-Approximation via MST (Parallel)
//!
//! Implements Section 4: Approximating Metric TSP via MST (parallel version)
//! Note: Euler tour remains sequential (DFS-based), but included for API completeness

pub mod TSPApproxMtEph {

    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;
    use std::vec::Vec;

    use ordered_float::OrderedFloat;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    pub type T<V> = LabUnDirGraphMtEph<V, OrderedFloat<f64>>;

    pub trait TSPApproxMtEphTrait {
        /// Parallel Euler tour of a tree
        /// APAS: Work O(|V|), Span O(|V|)
        fn euler_tour_mt<V: StT + MtT + Hash + Ord + 'static>(
            tree: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
            start: V,
        ) -> Vec<V>;

        /// Parallel shortcut Euler tour
        /// APAS: Work O(|V|), Span O(lg |V|)
        fn shortcut_tour_mt<V: StT + MtT + Hash + Ord>(euler_tour: &[V]) -> Vec<V>;

        /// Parallel tour weight computation
        /// APAS: Work O(|V|), Span O(lg |V|)
        fn tour_weight_mt<V: StT + MtT + Hash + Ord>(
            tour: &[V],
            distances: &HashMap<(V, V), OrderedFloat<f64>>,
        ) -> OrderedFloat<f64>;

        /// Parallel 2-approximation algorithm for metric TSP
        /// APAS: Work O(|V|² log |V|), Span O(|V| log |V|)
        fn approx_metric_tsp_mt<V: StT + MtT + Hash + Ord + 'static>(
            distances: &HashMap<(V, V), OrderedFloat<f64>>,
            vertices: &SetStEph<V>,
        ) -> Vec<V>;
    }

    /// Euler Tour of a Tree (Parallel version, but DFS remains sequential)
    ///
    /// APAS: Work O(n), Span O(n)
    /// claude-4-sonet: Work O(n), Span O(n) [DFS is inherently sequential]
    ///
    /// Arguments:
    /// - graph: Undirected graph (should be a tree)
    /// - start: Starting vertex
    /// - tree_edges: Set of edges forming the tree
    ///
    /// Returns:
    /// - Vector of vertices in Euler tour order
    pub fn euler_tour_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
        start: &V,
        tree_edges: &SetStEph<LabEdge<V, OrderedFloat<f64>>>,
    ) -> Vec<V> {
        let mut tour = Vec::new();
        let mut visited_edges: HashSet<(V, V)> = HashSet::new();

        euler_tour_dfs(graph, start, None, tree_edges, &mut tour, &mut visited_edges);

        tour
    }

    fn euler_tour_dfs<V: StT + MtT + Hash + Ord>(
        graph: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
        current: &V,
        parent: Option<&V>,
        tree_edges: &SetStEph<LabEdge<V, OrderedFloat<f64>>>,
        tour: &mut Vec<V>,
        visited_edges: &mut HashSet<(V, V)>,
    ) {
        tour.push(current.clone());

        let neighbors = get_neighbors(graph, current);
        for neighbor in neighbors.iter() {
            if let Some(p) = parent {
                if neighbor == p {
                    continue;
                }
            }

            let edge_key = if current < neighbor {
                (current.clone(), neighbor.clone())
            } else {
                (neighbor.clone(), current.clone())
            };

            if visited_edges.contains(&edge_key) {
                continue;
            }

            let mut edge_found = false;
            for edge in tree_edges.iter() {
                let LabEdge(u, v, _) = edge;
                if (u == current && v == neighbor) || (u == neighbor && v == current) {
                    edge_found = true;
                    break;
                }
            }

            if edge_found {
                visited_edges.insert(edge_key);
                euler_tour_dfs(graph, neighbor, Some(current), tree_edges, tour, visited_edges);
                tour.push(current.clone());
            }
        }
    }

    /// Shortcut Tour (Parallel version)
    ///
    /// APAS: Work O(n), Span O(n)
    /// claude-4-sonet: Work O(n), Span O(n)
    ///
    /// Note: Could be parallelized with filter operation, but overhead likely not worth it
    pub fn shortcut_tour_mt<V: StT + MtT + Hash + Ord>(euler_tour: &[V]) -> Vec<V> {
        if euler_tour.is_empty() {
            return Vec::new();
        }

        let mut shortcut = Vec::new();
        let mut visited: HashSet<V> = HashSet::new();

        for vertex in euler_tour.iter() {
            if !visited.contains(vertex) {
                shortcut.push(vertex.clone());
                let _ = visited.insert(vertex.clone());
            }
        }

        if let Some(start) = shortcut.first() {
            shortcut.push(start.clone());
        }

        shortcut
    }

    /// Compute tour weight
    ///
    /// APAS: Work O(n), Span O(n)
    /// claude-4-sonet: Work O(n), Span O(n)
    pub fn tour_weight_mt<V: StT + MtT + Hash + Ord>(
        graph: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
        tour: &[V],
    ) -> OrderedFloat<f64> {
        let mut total = OrderedFloat(0.0);

        for i in 0..tour.len() - 1 {
            let u = &tour[i];
            let v = &tour[i + 1];

            if let Some(weight) = get_edge_weight(graph, u, v) {
                total += weight;
            }
        }

        total
    }

    fn get_neighbors<V: StT + MtT + Hash + Ord>(
        graph: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
        v: &V,
    ) -> SetStEph<V> {
        let mut neighbors = SetLit![];
        for edge in graph.labeled_edges().iter() {
            let LabEdge(a, b, _) = edge;
            if a == v {
                let _ = neighbors.insert(b.clone());
            } else if b == v {
                let _ = neighbors.insert(a.clone());
            }
        }
        neighbors
    }

    fn get_edge_weight<V: StT + MtT + Hash + Ord>(
        graph: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
        u: &V,
        v: &V,
    ) -> Option<OrderedFloat<f64>> {
        for edge in graph.labeled_edges().iter() {
            let LabEdge(a, b, w) = edge;
            if (a == u && b == v) || (a == v && b == u) {
                return Some(*w);
            }
        }
        None
    }

    /// Approximate Metric TSP (Parallel version)
    ///
    /// APAS: Work O(n+m), Span O(n+m)
    /// claude-4-sonet: Work O(n+m), Span O(n+m) [Limited parallelism due to DFS]
    ///
    /// Arguments:
    /// - graph: Complete weighted undirected graph (metric)
    /// - spanning_tree: Spanning tree edges (ideally MST)
    /// - start: Starting vertex
    ///
    /// Returns:
    /// - (tour, weight): Hamiltonian cycle and its total weight
    pub fn approx_metric_tsp_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &LabUnDirGraphMtEph<V, OrderedFloat<f64>>,
        spanning_tree: &SetStEph<LabEdge<V, OrderedFloat<f64>>>,
        start: &V,
    ) -> (Vec<V>, OrderedFloat<f64>) {
        let euler = euler_tour_mt(graph, start, spanning_tree);
        let tour = shortcut_tour_mt(&euler);
        let weight = tour_weight_mt(graph, &tour);

        (tour, weight)
    }
}
