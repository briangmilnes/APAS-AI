//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Undirected Graph (ephemeral) with floating-point weights - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for weighted neighbor operations.
//! Weighted edge filtering (neighbors_weighted) is parallel.

pub mod WeightedUnDirGraphMtEphFloat {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;

    /// Convenience functions for weighted undirected graphs with floating-point weights (multi-threaded)
    impl<V: HashOrd + MtT + 'static> WeightedUnDirGraphMtEphFloat<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
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
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {
            self.add_labeled_edge(v1, v2, weight);
        }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential search
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {
            self.get_edge_label(v1, v2).copied()
        }

        /// Get all weighted edges as (v1, v2, weight) tuples
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential map
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                edges.insert((labeled_edge.0.clone_mt(), labeled_edge.1.clone_mt(), labeled_edge.2));
            }
            edges
        }

        /// Get neighbors with weights
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|) - parallel divide-and-conquer filter
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {
            // PARALLEL: filter weighted edges using divide-and-conquer
            let edges: Vec<LabEdge<V, OrderedF64>> = self.labeled_edges().iter().cloned().collect();
            let n = edges.len();

            if n <= 8 {
                let mut neighbors = Set::empty();
                for labeled_edge in edges {
                    if labeled_edge.0 == *v {
                        neighbors.insert((labeled_edge.1.clone_mt(), labeled_edge.2));
                    } else if labeled_edge.1 == *v {
                        neighbors.insert((labeled_edge.0.clone_mt(), labeled_edge.2));
                    }
                }
                return neighbors;
            }

            // Parallel divide-and-conquer
            fn parallel_neighbors<V: HashOrd + MtT + 'static>(
                edges: Vec<LabEdge<V, OrderedF64>>,
                v: V,
            ) -> Set<(V, OrderedFloat<f64>)> {
                let n = edges.len();
                if n == 0 {
                    return Set::empty();
                }
                if n == 1 {
                    if edges[0].0 == v {
                        let mut s = Set::empty();
                        s.insert((edges[0].1.clone_mt(), edges[0].2));
                        return s;
                    } else if edges[0].1 == v {
                        let mut s = Set::empty();
                        s.insert((edges[0].0.clone_mt(), edges[0].2));
                        return s;
                    }
                    return Set::empty();
                }

                let mid = n / 2;
                let mut right_edges = edges;
                let left_edges = right_edges.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_neighbors(left_edges, v_left), move || {
                        parallel_neighbors(right_edges, v_right)
                    });

                left_result.union(&right_result)
            }

            parallel_neighbors(edges, v.clone_mt())
        }

        /// Get the total weight of all edges
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential sum
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
