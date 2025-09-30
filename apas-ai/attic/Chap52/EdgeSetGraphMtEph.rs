//! Edge Set Graph Representation - Ephemeral Multi-Threaded Implementation
//!
//! This module implements the ephemeral edge-set representation with parallel
//! map operations and in-place mutations protected by synchronization.

pub mod EdgeSetGraphMtEph {

use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use crate::Types::Types::*;
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

/// Ephemeral multi-threaded edge-set graph (struct product record type)
pub struct EdgeSetGraphMtEphS<V: MtT + Ord + Clone + Debug + Display> {
    /// Set of vertices
    vertices: Arc<Mutex<AVLTreeSetStEph<V>>>,
    /// Set of directed edges (u, v)
    edges: Arc<Mutex<AVLTreeSetStEph<Pair<V, V>>>>,
}

impl<V: MtT + Ord + Clone + Debug + Display> EdgeSetGraphMtEphS<V> {
    /// Claude Work: O(1) - constant time initialization
    /// Claude Span: O(1) - constant time initialization
    pub fn new() -> Self {
        EdgeSetGraphMtEphS {
            vertices: Arc::new(Mutex::new(AVLTreeSetStEph::empty())),
            edges: Arc::new(Mutex::new(AVLTreeSetStEph::empty())),
        }
    }

    /// Insert an isolated vertex (ephemeral mutation, thread-safe)
    /// Claude Work: O(lg n) - tree-based set insert
    /// Claude Span: O(lg n) - tree-based set insert + lock contention
    pub fn insert_vertex(&self, v: V) {
        let mut vertices = self.vertices.lock().unwrap();
        vertices.insert(v);
    }

    /// Delete an isolated vertex (ephemeral mutation, thread-safe)
    /// Claude Work: O(lg n) - tree-based set delete
    /// Claude Span: O(lg n) - tree-based set delete + lock contention
    pub fn delete_vertex(&self, v: &V) {
        let mut vertices = self.vertices.lock().unwrap();
        vertices.delete(v);
    }

    /// Insert a directed edge (ephemeral mutation, thread-safe)
    /// Claude Work: O(lg n) - tree-based set insert
    /// Claude Span: O(lg n) - tree-based set insert + lock contention
    pub fn insert_edge(&self, u: V, v: V) {
        let mut edges = self.edges.lock().unwrap();
        edges.insert(Pair(u, v));
    }

    /// Delete a directed edge (ephemeral mutation, thread-safe)
    /// Claude Work: O(lg n) - tree-based set delete
    /// Claude Span: O(lg n) - tree-based set delete + lock contention
    pub fn delete_edge(&self, u: &V, v: &V) {
        let mut edges = self.edges.lock().unwrap();
        edges.delete(&Pair(u.clone(), v.clone()));
    }

    /// Check if edge (u, v) exists
    /// Claude Work: O(lg n) - tree-based set find
    /// Claude Span: O(lg n) - tree-based set find + lock contention
    pub fn has_edge(&self, u: &V, v: &V) -> bool {
        let edges = self.edges.lock().unwrap();
        edges.find(&Pair(u.clone(), v.clone()))
    }

    /// Get number of vertices
    /// Claude Work: O(1) - constant time
    /// Claude Span: O(1) - constant time + lock contention
    pub fn vertex_count(&self) -> usize {
        let vertices = self.vertices.lock().unwrap();
        vertices.size()
    }

    /// Get number of edges
    /// Claude Work: O(1) - constant time
    /// Claude Span: O(1) - constant time + lock contention
    pub fn edge_count(&self) -> usize {
        let edges = self.edges.lock().unwrap();
        edges.size()
    }

    /// Check if graph is empty
    /// Claude Work: O(1) - constant time check
    /// Claude Span: O(1) - constant time check + lock contention
    pub fn is_empty(&self) -> bool {
        let vertices = self.vertices.lock().unwrap();
        vertices.size() == 0
    }
}

impl<V: MtT + Ord + Clone + Debug + Display> Clone for EdgeSetGraphMtEphS<V> {
    fn clone(&self) -> Self {
        EdgeSetGraphMtEphS {
            vertices: Arc::clone(&self.vertices),
            edges: Arc::clone(&self.edges),
        }
    }
}

impl<V: MtT + Ord + Clone + Debug + Display> Default for EdgeSetGraphMtEphS<V> {
    fn default() -> Self {
        Self::new()
    }
}

} // end mod EdgeSetGraphMtEph