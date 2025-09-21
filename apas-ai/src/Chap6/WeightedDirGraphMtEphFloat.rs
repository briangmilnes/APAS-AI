//! Chapter 6 Weighted Directed Graph (ephemeral) with floating-point weights - Multi-threaded version.

pub mod WeightedDirGraphMtEphFloat {
    use crate::Chap6::LabDirGraphMtEph::LabDirGraphMtEph::*;
    use crate::Types::Types::*;
    use crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    /// Weighted directed graph with floating-point weights (multi-threaded, type alias)
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;

    /// Convenience functions for weighted directed graphs with floating-point weights (multi-threaded)
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {
        /// Create from vertices and weighted edges
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, f64)>) -> Self {
            let labeled_edges = edges.iter().map(|(from, to, weight)| {
                LabEdge(from.clone(), to.clone(), OrderedFloat(*weight))
            }).collect::<Vec<_>>();
            
            let mut edge_set = Set::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }
            
            Self::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// Add a weighted edge to the graph
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: f64) {
            self.add_labeled_arc(from, to, OrderedFloat(weight));
        }

        /// Get the weight of an edge, if it exists
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<f64> {
            self.get_arc_label(from, to).map(|ordered_weight| ordered_weight.0)
        }

        /// Get all weighted edges as (from, to, weight) tuples
        pub fn weighted_edges(&self) -> Set<(V, V, f64)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                edges.insert((labeled_edge.0.clone_mt(), labeled_edge.1.clone_mt(), labeled_edge.2.0));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, f64)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert((labeled_edge.1.clone_mt(), labeled_edge.2.0));
                }
            }
            neighbors
        }

        /// Get incoming neighbors with weights
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, f64)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.1 == *v {
                    neighbors.insert((labeled_edge.0.clone_mt(), labeled_edge.2.0));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        pub fn total_weight(&self) -> f64 {
            self.labeled_arcs().iter().map(|edge| edge.2.0).sum()
        }
    }

    #[macro_export]
    macro_rules! WeightedDirGraphMtEphFloatLit {
        () => {{
            $crate::Chap6::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($from:expr, $to:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( ($from, $to, $weight as f64) ),* ];
            $crate::Chap6::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges)
        }};
    }

    #[allow(dead_code)]
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {
        let _g0: WeightedDirGraphMtEphFloat<usize> = WeightedDirGraphMtEphFloatLit!();
        let _g1 = WeightedDirGraphMtEphFloatLit!( V: [0, 1, 2], E: [(0, 1, 5.5), (1, 2, 3.14)] );
    }
}