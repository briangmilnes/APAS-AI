//! Chapter 6 Weighted Directed Graph (ephemeral) with floating-point weights - Single-threaded version.

pub mod WeightedDirGraphStEphFloat {
    use crate::Chap6::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Types::Types::*;
    use crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    /// Weighted directed graph with floating-point weights (type alias)
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;

    /// Convenience functions for weighted directed graphs with floating-point weights
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {
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
                edges.insert((labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2.0));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, f64)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert((labeled_edge.1.clone(), labeled_edge.2.0));
                }
            }
            neighbors
        }

        /// Get incoming neighbors with weights
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, f64)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.1 == *v {
                    neighbors.insert((labeled_edge.0.clone(), labeled_edge.2.0));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        pub fn total_weight(&self) -> f64 {
            self.labeled_arcs().iter().map(|edge| edge.2.0).sum()
        }

        /// Get edges with weight greater than threshold
        pub fn edges_above_weight(&self, threshold: f64) -> Set<(V, V, f64)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.2.0 > threshold {
                    edges.insert((labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2.0));
                }
            }
            edges
        }

        /// Get edges with weight less than threshold
        pub fn edges_below_weight(&self, threshold: f64) -> Set<(V, V, f64)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.2.0 < threshold {
                    edges.insert((labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2.0));
                }
            }
            edges
        }

        /// Get the minimum weight edge
        pub fn min_weight_edge(&self) -> Option<(V, V, f64)> {
            self.labeled_arcs().iter()
                .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
                .map(|edge| (edge.0.clone(), edge.1.clone(), edge.2.0))
        }

        /// Get the maximum weight edge
        pub fn max_weight_edge(&self) -> Option<(V, V, f64)> {
            self.labeled_arcs().iter()
                .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
                .map(|edge| (edge.0.clone(), edge.1.clone(), edge.2.0))
        }

        /// Scale all weights by a factor
        pub fn scale_weights(&mut self, factor: f64) {
            let current_edges: Vec<_> = self.labeled_arcs().iter().cloned().collect();
            
            // Clear current edges and re-add with scaled weights
            *self = Self::empty();
            let vertices: Vec<_> = current_edges.iter().map(|e| e.0.clone()).collect();
            for v in vertices {
                self.add_vertex(v);
            }
            
            // Add scaled edges
            for edge in current_edges {
                self.add_labeled_arc(edge.0, edge.1, OrderedFloat(edge.2.0 * factor));
            }
        }
    }

    #[macro_export]
    macro_rules! WeightedDirGraphStEphFloatLit {
        () => {{
            $crate::Chap6::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($from:expr, $to:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( ($from, $to, $weight as f64) ),* ];
            $crate::Chap6::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges)
        }};
    }

    #[allow(dead_code)]
    fn _WeightedDirGraphStEphFloatLit_type_checks() {
        let _ = WeightedDirGraphStEphFloatLit!( V: [1], E: [(1, 2, 10.5)] ); // non-empty infers
        let _: WeightedDirGraphStEphFloat<i32> = WeightedDirGraphStEphFloatLit!(); // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {
        let _g0: WeightedDirGraphStEphFloat<usize> = WeightedDirGraphStEphFloatLit!();
        let _g1 = WeightedDirGraphStEphFloatLit!( V: [0, 1, 2], E: [(0, 1, 5.5), (1, 2, 3.14)] );
        let _g2 = WeightedDirGraphStEphFloatLit!( V: ["a", "b"], E: [("a", "b", 42.0)] );
    }
}