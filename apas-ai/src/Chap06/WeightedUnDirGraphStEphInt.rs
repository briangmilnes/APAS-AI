//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Undirected Graph (ephemeral) with integer weights - Single-threaded version.

pub mod WeightedUnDirGraphStEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;

    /// Convenience functions for weighted undirected graphs with integer weights
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {
            let labeled_edges = edges
                .iter()
                .map(|(v1, v2, weight)| LabEdge(v1.clone(), v2.clone(), *weight))
                .collect::<Vec<_>>();

            let mut edge_set = Set::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }

            Self::from_vertices_and_labeled_edges(vertices, edge_set)
        }

        /// Add a weighted edge to the graph (undirected)
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(v1, v2, weight); }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential search
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, v2).copied() }

        /// Get all weighted edges as (v1, v2, weight) tuples
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential map
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                edges.insert((labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
            }
            edges
        }

        /// Get neighbors with weights
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential filter
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert((labeled_edge.1.clone(), labeled_edge.2));
                } else if labeled_edge.1 == *v {
                    neighbors.insert((labeled_edge.0.clone(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential sum
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum() }

        /// Get the degree of a vertex (number of incident edges)
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }

        /// Check if the graph is connected (all vertices reachable from any vertex)
        pub fn is_connected(&self) -> bool {
            if self.vertices().size() == 0 {
                return true; // Empty graph is considered connected
            }

            // Simple connectivity check using DFS from first vertex
            let mut visited = Set::empty();
            let mut stack = Vec::new();

            if let Some(start) = self.vertices().iter().next() {
                stack.push(start.clone());

                while let Some(current) = stack.pop() {
                    if visited.mem(&current) == false {
                        visited.insert(current.clone());
                        for neighbor in self.neighbors(&current).iter() {
                            if visited.mem(neighbor) == false {
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }
            }

            visited.size() == self.vertices().size()
        }
    }

    #[macro_export]
    macro_rules! WeightedUnDirGraphStEphIntLit {
        () => {{
            $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($v1:expr, $v2:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( ($v1, $v2, $weight) ),* ];
            $crate::Chap06::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::from_weighted_edges(vertices, edges)
        }};
    }

    #[allow(dead_code)]
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {
        let _g0: WeightedUnDirGraphStEphInt<usize> = WeightedUnDirGraphStEphIntLit!();
        let _g1 = WeightedUnDirGraphStEphIntLit!( V: [0, 1, 2], E: [(0, 1, 5), (1, 2, 3)] );
        let _g2 = WeightedUnDirGraphStEphIntLit!( V: ["a", "b"], E: [("a", "b", 42)] );
    }
}
