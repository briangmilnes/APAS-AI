//! Edge Set Graph Representation - Persistent Multi-Threaded Implementation
//!
//! This module implements the edge-set representation with parallel map operations.
//! Uses persistent data structures with thread-safe parallel traversal.

pub mod EdgeSetGraphMtPer {

use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use crate::Types::Types::*;
use std::fmt::{Debug, Display};

/// Persistent multi-threaded edge-set graph (struct product record type)
#[derive(Clone)]
pub struct EdgeSetGraphMtPerS<V: MtT + Ord + Clone + Debug + Display> {
    /// Set of vertices
    vertices: AVLTreeSetStPer<V>,
    /// Set of directed edges (u, v)
    edges: AVLTreeSetStPer<Pair<V, V>>,
}

impl<V: MtT + Ord + Clone + Debug + Display> EdgeSetGraphMtPerS<V> {
    /// Claude Work: O(1) - constant time initialization
    /// Claude Span: O(1) - constant time initialization
    pub fn new() -> Self {
        EdgeSetGraphMtPerS {
            vertices: AVLTreeSetStPer::empty(),
            edges: AVLTreeSetStPer::empty(),
        }
    }

    /// Create graph from vertex and edge sets
    /// Claude Work: O(1) - just wrapping existing sets
    /// Claude Span: O(1) - just wrapping existing sets
    pub fn from_sets(vertices: AVLTreeSetStPer<V>, edges: AVLTreeSetStPer<Pair<V, V>>) -> Self {
        EdgeSetGraphMtPerS { vertices, edges }
    }

    /// Insert an isolated vertex
    /// Claude Work: O(lg n) - tree-based set insert
    /// Claude Span: O(lg n) - tree-based set insert
    pub fn insert_vertex(self, v: V) -> Self {
        EdgeSetGraphMtPerS {
            vertices: self.vertices.insert(v),
            edges: self.edges,
        }
    }

    /// Delete an isolated vertex
    /// Claude Work: O(lg n) - tree-based set delete
    /// Claude Span: O(lg n) - tree-based set delete
    pub fn delete_vertex(self, v: &V) -> Self {
        EdgeSetGraphMtPerS {
            vertices: self.vertices.delete(v),
            edges: self.edges,
        }
    }

    /// Insert a directed edge
    /// Claude Work: O(lg n) - tree-based set insert
    /// Claude Span: O(lg n) - tree-based set insert
    pub fn insert_edge(self, u: V, v: V) -> Self {
        EdgeSetGraphMtPerS {
            vertices: self.vertices,
            edges: self.edges.insert(Pair(u, v)),
        }
    }

    /// Delete a directed edge
    /// Claude Work: O(lg n) - tree-based set delete
    /// Claude Span: O(lg n) - tree-based set delete
    pub fn delete_edge(self, u: &V, v: &V) -> Self {
        EdgeSetGraphMtPerS {
            vertices: self.vertices,
            edges: self.edges.delete(&Pair(u.clone(), v.clone())),
        }
    }

    /// Check if edge (u, v) exists
    /// Claude Work: O(lg n) - tree-based set find
    /// Claude Span: O(lg n) - tree-based set find
    pub fn has_edge(&self, u: &V, v: &V) -> bool {
        self.edges.find(&Pair(u.clone(), v.clone()))
    }

    /// Get number of vertices
    /// Claude Work: O(1) - constant time
    /// Claude Span: O(1) - constant time
    pub fn vertex_count(&self) -> usize {
        self.vertices.size()
    }

    /// Get number of edges
    /// Claude Work: O(1) - constant time
    /// Claude Span: O(1) - constant time
    pub fn edge_count(&self) -> usize {
        self.edges.size()
    }

    /// Check if graph is empty
    /// Claude Work: O(1) - constant time check
    /// Claude Span: O(1) - constant time check
    pub fn is_empty(&self) -> bool {
        self.vertices.size() == 0
    }

    /// Get reference to vertex set
    pub fn vertices(&self) -> &AVLTreeSetStPer<V> {
        &self.vertices
    }

    /// Get reference to edge set
    pub fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>> {
        &self.edges
    }
}

impl<V: MtT + Ord + Clone + Debug + Display> Default for EdgeSetGraphMtPerS<V> {
    fn default() -> Self {
        Self::new()
    }
}

} // end mod EdgeSetGraphMtPer