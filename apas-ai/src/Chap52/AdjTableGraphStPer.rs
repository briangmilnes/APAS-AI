//! Adjacency Table Graph Representation - Persistent Single-Threaded Implementation
//!
//! This module implements G = (V × (V set)) table representation where each vertex
//! maps to the set of its out-neighbors. Cost Specification 52.3.

pub mod AdjTableGraphStPer {

use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use crate::Chap43Claude::OrderedTableStPer::OrderedTableStPer::*;
use crate::Types::Types::*;
use std::fmt::{Debug, Display};

/// Persistent single-threaded adjacency-table graph (struct product record type)
#[derive(Clone)]
pub struct AdjTableGraphStPerS<V: StT + Ord + Clone + Debug + Display> {
    /// Adjacency table mapping each vertex to set of out-neighbors
    adj_table: OrderedTableStPer<V, AVLTreeSetStPer<V>>,
}

impl<V: StT + Ord + Clone + Debug + Display> AdjTableGraphStPerS<V> {
    /// Claude Work: O(1) - constant time initialization
    /// Claude Span: O(1) - constant time initialization
    pub fn new() -> Self {
        AdjTableGraphStPerS {
            adj_table: OrderedTableStPer::empty(),
        }
    }

    /// Insert an isolated vertex
    /// Claude Work: O(lg n) - table insert with empty set
    /// Claude Span: O(lg n) - table insert
    pub fn insert_vertex(self, v: V) -> Self {
        AdjTableGraphStPerS {
            adj_table: self.adj_table.insert(v, AVLTreeSetStPer::empty()),
        }
    }

    /// Delete an isolated vertex
    /// Claude Work: O(lg n) - table delete
    /// Claude Span: O(lg n) - table delete
    pub fn delete_vertex(self, v: &V) -> Self {
        AdjTableGraphStPerS {
            adj_table: self.adj_table.delete(v),
        }
    }

    /// Insert a directed edge (u, v)
    /// Claude Work: O(lg n) - table lookup + set insert + table update
    /// Claude Span: O(lg n) - sequential operations
    pub fn insert_edge(self, u: V, v: V) -> Self {
        let neighbors = self.adj_table
            .find(&u)
            .map(|s| s.clone())
            .unwrap_or_else(|| AVLTreeSetStPer::empty())
            .insert(v);
        
        AdjTableGraphStPerS {
            adj_table: self.adj_table.insert(u, neighbors),
        }
    }

    /// Delete a directed edge (u, v)
    /// Claude Work: O(lg n) - table lookup + set delete + table update
    /// Claude Span: O(lg n) - sequential operations
    pub fn delete_edge(self, u: &V, v: &V) -> Self {
        if let Some(neighbors) = self.adj_table.find(u) {
            let new_neighbors = neighbors.clone().delete(v);
            AdjTableGraphStPerS {
                adj_table: self.adj_table.insert(u.clone(), new_neighbors),
            }
        } else {
            self
        }
    }

    /// Check if edge (u, v) exists
    /// Claude Work: O(lg n) - table lookup + set contains
    /// Claude Span: O(lg n) - sequential operations
    pub fn has_edge(&self, u: &V, v: &V) -> bool {
        self.adj_table
            .find(u)
            .map(|neighbors| neighbors.find(v))
            .unwrap_or(false)
    }

    /// Get number of vertices
    /// Claude Work: O(1) - constant time
    /// Claude Span: O(1) - constant time
    pub fn vertex_count(&self) -> usize {
        self.adj_table.size()
    }

    /// Check if graph is empty
    /// Claude Work: O(1) - constant time check
    /// Claude Span: O(1) - constant time check
    pub fn is_empty(&self) -> bool {
        self.adj_table.size() == 0
    }
}

impl<V: StT + Ord + Clone + Debug + Display> Default for AdjTableGraphStPerS<V> {
    fn default() -> Self {
        Self::new()
    }
}

} // end mod AdjTableGraphStPer