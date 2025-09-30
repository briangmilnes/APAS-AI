//! Edge Set Graph Representation - Persistent Single-Threaded Implementation (WORKING REFERENCE)
//!
//! This is the reference implementation showing correct usage of APAS APIs.
//! Other representations follow the same pattern.

pub mod EdgeSetGraphStPer {

use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use crate::Chap43Claude::OrderedTableStPer::OrderedTableStPer::*;
use crate::Types::Types::*;
use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct EdgeSetGraphStPerS<V: StT + Ord> {
    vertices: AVLTreeSetStPer<V>,
    edges: AVLTreeSetStPer<Pair<V, V>>,
}

impl<V: StT + Ord> EdgeSetGraphStPerS<V> {
    pub fn new() -> Self {
        EdgeSetGraphStPerS {
            vertices: AVLTreeSetStPer::empty(),
            edges: AVLTreeSetStPer::empty(),
        }
    }

    pub fn insert_vertex(&self, v: V) -> Self {
        EdgeSetGraphStPerS {
            vertices: self.vertices.insert(v),
            edges: self.edges.clone(),
        }
    }

    pub fn insert_edge(&self, u: V, v: V) -> Self {
        EdgeSetGraphStPerS {
            vertices: self.vertices.clone(),
            edges: self.edges.insert(Pair(u, v)),
        }
    }

    pub fn has_edge(&self, u: &V, v: &V) -> bool {
        self.edges.find(&Pair(u.clone(), v.clone()))
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.size()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.size()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.size() == 0
    }
}

impl<V: StT + Ord> Default for EdgeSetGraphStPerS<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: StT + Ord + Display> Display for EdgeSetGraphStPerS<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EdgeSetGraph(v={}, e={})", self.vertex_count(), self.edge_count())
    }
}

} // end mod