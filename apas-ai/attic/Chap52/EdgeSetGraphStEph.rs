//! Edge Set Graph Representation - Ephemeral Single-Threaded Implementation
//!
//! This module implements the ephemeral edge-set representation where updates
//! are performed in-place for better efficiency in sequential algorithms.

pub mod EdgeSetGraphStEph {

use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use crate::Types::Types::*;

/// Ephemeral single-threaded edge-set graph (struct product record type)
pub struct EdgeSetGraphStEphS<V: StT + Ord> {
    /// Set of vertices
    vertices: AVLTreeSetStEph<V>,
    /// Set of directed edges (u, v)
    edges: AVLTreeSetStEph<Pair<V, V>>,
}

impl<V: StT + Ord> EdgeSetGraphStEphS<V> {
    /// Claude Work: O(1) - constant time initialization
    /// Claude Span: O(1) - constant time initialization
    pub fn new() -> Self {
        EdgeSetGraphStEphS {
            vertices: AVLTreeSetStEph::empty(),
            edges: AVLTreeSetStEph::empty(),
        }
    }

    /// Insert an isolated vertex (ephemeral mutation)
    /// Claude Work: O(lg n) - tree-based set insert
    /// Claude Span: O(lg n) - tree-based set insert
    pub fn insert_vertex(&mut self, v: V) {
        self.vertices.insert(v);
    }

    /// Delete an isolated vertex (ephemeral mutation)
    /// Claude Work: O(lg n) - tree-based set delete
    /// Claude Span: O(lg n) - tree-based set delete
    pub fn delete_vertex(&mut self, v: &V) {
        self.vertices.delete(v);
    }

    /// Insert a directed edge (ephemeral mutation)
    /// Claude Work: O(lg n) - tree-based set insert
    /// Claude Span: O(lg n) - tree-based set insert
    pub fn insert_edge(&mut self, u: V, v: V) {
        self.edges.insert(Pair(u, v));
    }

    /// Delete a directed edge (ephemeral mutation)
    /// Claude Work: O(lg n) - tree-based set delete
    /// Claude Span: O(lg n) - tree-based set delete
    pub fn delete_edge(&mut self, u: &V, v: &V) {
        self.edges.delete(&Pair(u.clone(), v.clone()));
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
    pub fn vertices(&self) -> &AVLTreeSetStEph<V> {
        &self.vertices
    }

    /// Get reference to edge set
    pub fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>> {
        &self.edges
    }
}

impl<V: StT + Ord> Default for EdgeSetGraphStEphS<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: StT + Ord> Clone for EdgeSetGraphStEphS<V> {
    fn clone(&self) -> Self {
        EdgeSetGraphStEphS {
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
        }
    }
}

} // end mod EdgeSetGraphStEph