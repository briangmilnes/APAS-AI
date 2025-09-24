//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Undirected Graph (ephemeral) with floating-point weights - Multi-threaded version.

pub mod WeightedUnDirGraphMtEphFloat {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
    use crate::Types::Types::*;

    /// Weighted undirected graph with floating-point weights (multi-threaded, type alias)
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;

    /// Convenience functions for weighted undirected graphs with floating-point weights (multi-threaded)
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {
        /// Create from vertices and weighted edges
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Self {
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
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {
            self.add_labeled_edge(v1, v2, weight);
        }

        /// Get the weight of an edge, if it exists
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {
            self.get_edge_label(v1, v2).copied()
        }

        /// Get all weighted edges as (v1, v2, weight) tuples
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                edges.insert((labeled_edge.0.clone_mt(), labeled_edge.1.clone_mt(), labeled_edge.2));
            }
            edges
        }

        /// Get neighbors with weights
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert((labeled_edge.1.clone_mt(), labeled_edge.2));
                } else if labeled_edge.1 == *v {
                    neighbors.insert((labeled_edge.0.clone_mt(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        pub fn total_weight(&self) -> OrderedFloat<f64> {
            self.labeled_edges()
                .iter()
                .map(|edge| edge.2)
                .fold(OrderedFloat(0.0), |acc, w| acc + w)
        }

        /// Get the degree of a vertex (number of incident edges)
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }
    }

    #[macro_export]
    macro_rules! WeightedUnDirGraphMtEphFloatLit {
        () => {{
            $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($v1:expr, $v2:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( ($v1, $v2, OrderedFloat($weight as f64)) ),* ];
            $crate::Chap06::WeightedUnDirGraphMtEphFloat::WeightedUnDirGraphMtEphFloat::WeightedUnDirGraphMtEphFloat::from_weighted_edges(vertices, edges)
        }};
    }

    #[allow(dead_code)]
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {
        let _g0: WeightedUnDirGraphMtEphFloat<usize> = WeightedUnDirGraphMtEphFloatLit!();
        let _g1 = WeightedUnDirGraphMtEphFloatLit!( V: [0, 1, 2], E: [(0, 1, 5.5), (1, 2, 3.14)] );
    }
}
