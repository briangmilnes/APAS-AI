//! Chapter 6 Weighted Undirected Graph (ephemeral) with floating-point weights - Single-threaded version.

pub mod WeightedUnDirGraphStEphFloat {
    use crate::Chap6::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;
    use crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    /// Weighted undirected graph with floating-point weights (type alias)
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;

    /// Convenience functions for weighted undirected graphs with floating-point weights
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {
        /// Create from vertices and weighted edges
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, f64)>) -> Self {
            let labeled_edges = edges.iter().map(|(v1, v2, weight)| {
                LabEdge(v1.clone(), v2.clone(), OrderedFloat(*weight))
            }).collect::<Vec<_>>();
            
            let mut edge_set = Set::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }
            
            Self::from_vertices_and_labeled_edges(vertices, edge_set)
        }

        /// Add a weighted edge to the graph (undirected)
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: f64) {
            self.add_labeled_edge(v1, v2, OrderedFloat(weight));
        }

        /// Get the weight of an edge, if it exists
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<f64> {
            self.get_edge_label(v1, v2).map(|ordered_weight| ordered_weight.0)
        }

        /// Get all weighted edges as (v1, v2, weight) tuples
        pub fn weighted_edges(&self) -> Set<(V, V, f64)> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                edges.insert((labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2.0));
            }
            edges
        }

        /// Get neighbors with weights
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, f64)> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_edges().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert((labeled_edge.1.clone(), labeled_edge.2.0));
                } else if labeled_edge.1 == *v {
                    neighbors.insert((labeled_edge.0.clone(), labeled_edge.2.0));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        pub fn total_weight(&self) -> f64 {
            self.labeled_edges().iter().map(|edge| edge.2.0).sum()
        }

        /// Get the degree of a vertex (number of incident edges)
        pub fn vertex_degree(&self, v: &V) -> usize {
            self.neighbors(v).size()
        }

        /// Check if the graph is connected (all vertices reachable from any vertex)
        pub fn is_connected(&self) -> bool {
            if self.vertices().is_empty() {
                return true; // Empty graph is considered connected
            }
            
            // Simple connectivity check using DFS from first vertex
            let mut visited = Set::empty();
            let mut stack = Vec::new();
            
            if let Some(start) = self.vertices().iter().next() {
                stack.push(start.clone());
                
                while let Some(current) = stack.pop() {
                    if !visited.mem(&current) {
                        visited.insert(current.clone());
                        for neighbor in self.neighbors(&current).iter() {
                            if !visited.mem(neighbor) {
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }
            }
            
            visited.size() == self.vertices().size()
        }

        /// Get the minimum weight edge
        pub fn min_weight_edge(&self) -> Option<(V, V, f64)> {
            self.labeled_edges().iter()
                .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
                .map(|edge| (edge.0.clone(), edge.1.clone(), edge.2.0))
        }

        /// Get the maximum weight edge
        pub fn max_weight_edge(&self) -> Option<(V, V, f64)> {
            self.labeled_edges().iter()
                .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
                .map(|edge| (edge.0.clone(), edge.1.clone(), edge.2.0))
        }
    }

    #[macro_export]
    macro_rules! WeightedUnDirGraphStEphFloatLit {
        () => {{
            $crate::Chap6::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($v1:expr, $v2:expr, $weight:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( ($v1, $v2, $weight as f64) ),* ];
            $crate::Chap6::WeightedUnDirGraphStEphFloat::WeightedUnDirGraphStEphFloat::WeightedUnDirGraphStEphFloat::from_weighted_edges(vertices, edges)
        }};
    }

    #[allow(dead_code)]
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {
        let _g0: WeightedUnDirGraphStEphFloat<usize> = WeightedUnDirGraphStEphFloatLit!();
        let _g1 = WeightedUnDirGraphStEphFloatLit!( V: [0, 1, 2], E: [(0, 1, 5.5), (1, 2, 3.14)] );
        let _g2 = WeightedUnDirGraphStEphFloatLit!( V: ["a", "b"], E: [("a", "b", 42.0)] );
    }
}