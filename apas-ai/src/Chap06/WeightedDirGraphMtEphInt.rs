//! Chapter 6 Weighted Directed Graph (ephemeral) with integer weights - Multi-threaded version.

pub mod WeightedDirGraphMtEphInt {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
    use crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::*;
    use crate::Types::Types::*;

    /// Weighted directed graph with integer weights (multi-threaded, type alias)
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;

    /// Convenience functions for weighted directed graphs with integer weights (multi-threaded)
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {
        /// Create from vertices and weighted edges
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {
            let labeled_edges = edges.iter().map(|(from, to, weight)| {
                LabEdge(from.clone(), to.clone(), *weight)
            }).collect::<Vec<_>>();
            
            let mut edge_set = Set::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }
            
            Self::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// Add a weighted edge to the graph
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) {
            self.add_labeled_arc(from, to, weight);
        }

        /// Get the weight of an edge, if it exists
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> {
            self.get_arc_label(from, to).copied()
        }

        /// Get all weighted edges as (from, to, weight) tuples
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                edges.insert((labeled_edge.0.clone_mt(), labeled_edge.1.clone_mt(), labeled_edge.2));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert((labeled_edge.1.clone_mt(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get incoming neighbors with weights
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.1 == *v {
                    neighbors.insert((labeled_edge.0.clone_mt(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        pub fn total_weight(&self) -> i32 {
            self.labeled_arcs().iter().map(|edge| edge.2).sum()
        }
    }

    #[macro_export]
    macro_rules! WeightedDirGraphMtEphIntLit {
        () => {{
            $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($from:expr, $to:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( ($from, $to, $weight) ),* ];
            $crate::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges)
        }};
    }

    #[allow(dead_code)]
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {
        let _g0: WeightedDirGraphMtEphInt<usize> = WeightedDirGraphMtEphIntLit!();
        let _g1 = WeightedDirGraphMtEphIntLit!( V: [0, 1, 2], E: [(0, 1, 5), (1, 2, 3)] );
    }
}
