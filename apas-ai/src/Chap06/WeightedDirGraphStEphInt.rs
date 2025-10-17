//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with integer weights - Single-threaded version.

pub mod WeightedDirGraphStEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Types::Types::*;

    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;

    /// Convenience functions for weighted directed graphs with integer weights
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        pub fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> Self {
            let labeled_edges = edges
                .iter()
                .map(|Triple(from, to, weight)| LabEdge(from.clone(), to.clone(), *weight))
                .collect::<Vec<_>>();

            let mut edge_set = SetStEph::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }

            Self::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// Add a weighted edge to the graph
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(from, to, weight); }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(from, to).copied() }

        /// Get all weighted edges as (from, to, weight) tuples
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        pub fn weighted_edges(&self) -> SetStEph<Triple<V, V, i32>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        pub fn out_neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, i32>> {
            let mut neighbors = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert(Pair(labeled_edge.1.clone(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get incoming neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        pub fn in_neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, i32>> {
            let mut neighbors = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.1 == *v {
                    neighbors.insert(Pair(labeled_edge.0.clone(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential sum
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() }

        /// Get edges with weight greater than threshold
        pub fn edges_above_weight(&self, threshold: i32) -> SetStEph<Triple<V, V, i32>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.2 > threshold {
                    edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
                }
            }
            edges
        }

        /// Get edges with weight less than threshold
        pub fn edges_below_weight(&self, threshold: i32) -> SetStEph<Triple<V, V, i32>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.2 < threshold {
                    edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
                }
            }
            edges
        }
    }

    /// Macro accepts raw tuple syntax: `E: [(from, to, weight), ...]`
    /// Internally wraps each edge as `Triple(from, to, weight)` for StT compliance.
    #[macro_export]
    macro_rules! WeightedDirGraphStEphIntLit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($from:expr, $to:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( Triple($from, $to, $weight) ),* ];
            $crate::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges)
        }};
    }
}
