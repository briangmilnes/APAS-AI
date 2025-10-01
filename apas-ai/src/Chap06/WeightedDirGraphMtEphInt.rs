//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with integer weights - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for weighted neighbor operations.
//! Weighted arc filtering (out_neighbors_weighted, in_neighbors_weighted) is parallel.

pub mod WeightedDirGraphMtEphInt {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    /// Weighted directed graph with integer weights (multi-threaded, type alias)
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;

    /// Convenience functions for weighted directed graphs with integer weights (multi-threaded)
    impl<V: StT + MtT + Hash + 'static> WeightedDirGraphMtEphInt<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {
            let labeled_edges = edges
                .iter()
                .map(|(from, to, weight)| LabEdge(from.clone(), to.clone(), *weight))
                .collect::<Vec<_>>();

            let mut edge_set = Set::empty();
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
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                edges.insert((labeled_edge.0.clone_mt(), labeled_edge.1.clone_mt(), labeled_edge.2));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {
            // PARALLEL: filter weighted arcs using divide-and-conquer
            let arcs: Vec<LabEdge<V, i32>> = self.labeled_arcs().iter().cloned().collect();
            let n = arcs.len();

            if n <= 8 {
                let mut neighbors = Set::empty();
                for labeled_edge in arcs {
                    if labeled_edge.0 == *v {
                        neighbors.insert((labeled_edge.1.clone_mt(), labeled_edge.2));
                    }
                }
                return neighbors;
            }

            // Parallel divide-and-conquer
            fn parallel_out<V: StT + MtT + Hash + 'static>(arcs: Vec<LabEdge<V, i32>>, v: V) -> Set<(V, i32)> {
                let n = arcs.len();
                if n == 0 {
                    return Set::empty();
                }
                if n == 1 {
                    return if arcs[0].0 == v {
                        let mut s = Set::empty();
                        s.insert((arcs[0].1.clone_mt(), arcs[0].2));
                        s
                    } else {
                        Set::empty()
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_out(left_arcs, v_left), move || parallel_out(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_out(arcs, v.clone_mt())
        }

        /// Get incoming neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {
            // PARALLEL: filter weighted arcs using divide-and-conquer
            let arcs: Vec<LabEdge<V, i32>> = self.labeled_arcs().iter().cloned().collect();
            let n = arcs.len();

            if n <= 8 {
                let mut neighbors = Set::empty();
                for labeled_edge in arcs {
                    if labeled_edge.1 == *v {
                        neighbors.insert((labeled_edge.0.clone_mt(), labeled_edge.2));
                    }
                }
                return neighbors;
            }

            // Parallel divide-and-conquer
            fn parallel_in<V: StT + MtT + Hash + 'static>(arcs: Vec<LabEdge<V, i32>>, v: V) -> Set<(V, i32)> {
                let n = arcs.len();
                if n == 0 {
                    return Set::empty();
                }
                if n == 1 {
                    return if arcs[0].1 == v {
                        let mut s = Set::empty();
                        s.insert((arcs[0].0.clone_mt(), arcs[0].2));
                        s
                    } else {
                        Set::empty()
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_in(left_arcs, v_left), move || parallel_in(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_in(arcs, v.clone_mt())
        }

        /// Get the total weight of all edges
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential sum
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() }
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
