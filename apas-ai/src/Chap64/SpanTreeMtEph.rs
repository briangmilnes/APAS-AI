//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Parallel)
//!
//! Implements Exercise 64.2: Compute spanning tree using parallel star contraction.

pub mod SpanTreeMtEph {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::star_contract_mt;
    use crate::SetLit;
    use crate::Types::Types::*;

    pub trait SpanTreeMtEphTrait {
        /// Parallel spanning tree via star contraction
        /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
        fn spanning_tree_star_contraction_mt<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
        ) -> Set<Edge<V>>;

        /// Verify spanning tree properties
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        fn verify_spanning_tree<V: StT + MtT + Hash + Ord>(graph: &UnDirGraphMtEph<V>, tree: &Set<Edge<V>>) -> B;
    }

    /// Exercise 64.2: Spanning Tree via Star Contraction (Parallel)
    ///
    /// Computes a spanning tree using parallel star contraction.
    ///
    /// APAS: Work O((n+m) lg n), Span O(lg² n)
    /// claude-4-sonet: Work O((n+m) lg n), Span O(lg² n), Parallelism Θ((n+m)/lg² n)
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for star partition
    ///
    /// Returns:
    /// - Set of edges forming a spanning tree
    pub fn spanning_tree_star_contraction_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> Set<Edge<V>> {
        // Base: no edges means no spanning tree edges
        let base = |_vertices: &Set<V>| SetLit![];

        // Expand: add star partition edges and map quotient tree edges back
        let expand = |_v: &Set<V>,
                      original_edges: &Set<Edge<V>>,
                      _centers: &Set<V>,
                      partition_map: &HashMap<V, V>,
                      quotient_tree: Set<Edge<V>>| {
            let mut spanning_edges = SetLit![];

            // Add edges from vertices to their centers (star edges)
            for (vertex, center) in partition_map.iter() {
                if vertex != center {
                    let edge = if vertex < center {
                        Edge(vertex.clone(), center.clone())
                    } else {
                        Edge(center.clone(), vertex.clone())
                    };
                    let _ = spanning_edges.insert(edge);
                }
            }

            // Map quotient tree edges back to original edges
            // For each edge between centers in quotient tree, find original edge that maps to it
            for quotient_edge in quotient_tree.iter() {
                let Edge(c1, c2) = quotient_edge;

                // Find an original edge that connects the two stars (centers c1 and c2)
                for original_edge in original_edges.iter() {
                    let Edge(u, v) = original_edge;
                    let u_center = partition_map.get(u).unwrap_or(u);
                    let v_center = partition_map.get(v).unwrap_or(v);

                    // Check if this original edge connects the two centers (in either direction)
                    if (u_center == c1 && v_center == c2) || (u_center == c2 && v_center == c1) {
                        let _ = spanning_edges.insert(original_edge.clone());
                        break; // Only need one edge between the two stars
                    }
                }
            }

            spanning_edges
        };

        star_contract_mt(graph, seed, &base, &expand)
    }

    /// Verify that result is a valid spanning tree
    pub fn verify_spanning_tree<V: StT + MtT + Hash + Ord>(graph: &UnDirGraphMtEph<V>, tree_edges: &Set<Edge<V>>) -> B {
        let n = graph.sizeV();
        let expected_edges = if n > 0 { n - 1 } else { 0 };

        if tree_edges.size() != expected_edges {
            return false;
        }

        for edge in tree_edges.iter() {
            // For undirected graphs, check both edge orientations
            let Edge(u, v) = edge;
            if !graph.edges().mem(edge) && !graph.edges().mem(&Edge(v.clone(), u.clone())) {
                return false;
            }
        }

        true
    }
}
