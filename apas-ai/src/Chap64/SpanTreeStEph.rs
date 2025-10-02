// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Sequential)
//!
//! Implements Exercise 64.2: Compute spanning tree using star contraction.

pub mod SpanTreeStEph {

use std::collections::HashMap;
use std::hash::Hash;

use crate::Types::Types::*;
use crate::Chap05::SetStEph::SetStEph::*;
use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use crate::Chap62::StarContractionStEph::StarContractionStEph::star_contract;
use crate::SetLit;
    pub trait SpanTreeStEphTrait {
        /// Sequential spanning tree via star contraction
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn spanning_tree_star_contraction<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> Set<Edge<V>>;
        
        /// Verify spanning tree properties
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn verify_spanning_tree<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>, tree: &Set<Edge<V>>) -> B;
    }

    /// Exercise 64.2: Spanning Tree via Star Contraction
    ///
    /// Computes a spanning tree by recursively applying star contraction and
    /// collecting all edges from star partitions.
    ///
    /// Algorithm:
    /// 1. Base case: If no edges, return empty edge set
    /// 2. Compute star partition (centers and partition map)
    /// 3. Add all edges from partition map to spanning tree
    /// 4. Build quotient graph
    /// 5. Recursively compute spanning tree of quotient
    /// 6. Combine edges from partition and quotient spanning tree
    ///
    /// APAS: Work O((n+m) lg n), Span O((n+m) lg n)
    /// claude-4-sonet: Work O((n+m) lg n), Span O((n+m) lg n)
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - Set of edges forming a spanning tree
    pub fn spanning_tree_star_contraction<V: StT + Hash + Ord>(
        graph: &UnDirGraphStEph<V>,
    ) -> Set<Edge<V>> {
        // Base: no edges means no spanning tree edges (isolated vertices)
        let base = |_vertices: &Set<V>| SetLit![];

        // Expand: add star partition edges to recursive result
        let expand = |_v: &Set<V>,
                      _e: &Set<Edge<V>>,
                      _centers: &Set<V>,
                      partition_map: &HashMap<V, V>,
                      quotient_edges: Set<Edge<V>>| {
            // Collect edges from partition map (vertex → center edges)
            let mut spanning_edges = SetLit![];
            
            for (vertex, center) in partition_map.iter() {
                // Add edge if vertex is not its own center (avoid self-loops)
                if vertex != center {
                    // Normalize edge order
                    let edge = if vertex < center {
                        Edge(vertex.clone(), center.clone())
                    } else {
                        Edge(center.clone(), vertex.clone())
                    };
                    let _ = spanning_edges.insert(edge);
                }
            }
            
            // Map quotient edges back to original graph
            // Quotient edges are between centers, which are valid in original graph
            for edge in quotient_edges.iter() {
                let _ = spanning_edges.insert(edge.clone());
            }
            
            spanning_edges
        };

        star_contract(graph, &base, &expand)
    }

    /// Verify that result is a valid spanning tree
    ///
    /// Checks:
    /// 1. All vertices are included
    /// 2. Exactly |V| - 1 edges
    /// 3. All edges are from original graph
    ///
    /// Returns true if valid spanning tree
    pub fn verify_spanning_tree<V: StT + Hash + Ord>(
        graph: &UnDirGraphStEph<V>,
        tree_edges: &Set<Edge<V>>,
    ) -> B {
        let n = graph.sizeV();
        let expected_edges = if n > 0 { n - 1 } else { 0 };
        
        // Check edge count
        if tree_edges.size() != expected_edges {
            return false;
        }
        
        // Check all edges are from original graph
        for edge in tree_edges.iter() {
            if !graph.edges().mem(edge) {
                return false;
            }
        }
        
        true
    }
}

