//! Copyright © 2025 APAS-VERUS. All rights reserved.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Float Weights)
//!
//! Data structure for storing the result of all-pairs shortest path algorithms
//! with floating-point edge weights. Stores distance matrix and predecessor matrix for path reconstruction.
//!
//! Uses ephemeral array sequences for efficient in-place updates.
//! Uses `OrderedF64` (OrderedFloat<f64>) for weights to ensure Eq/Hash traits.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n²), Span O(n²) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;
use crate::Types::Types::OrderedF64;
use ordered_float::OrderedFloat;

const UNREACHABLE: OrderedF64 = OrderedFloat(f64::INFINITY);
const NO_PREDECESSOR: usize = usize::MAX;

/// Result structure for all-pairs shortest paths with floating-point weights.
pub struct AllPairsResultStEphFloat {
    /// Distance matrix: distances.nth(u).nth(v) is the distance from u to v.
    pub distances: ArraySeqStEphS<ArraySeqStEphS<OrderedF64>>,
    /// Predecessor matrix: predecessors.nth(u).nth(v) is the predecessor of v on shortest path from u.
    pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
    /// Number of vertices.
    pub n: usize,
}

impl AllPairsResultStEphFloat {
    /// Creates a new all-pairs result structure initialized for n vertices.
    /// All distances are set to UNREACHABLE except diagonal (0.0), all predecessors to NO_PREDECESSOR.
    pub fn new(n: usize) -> Self {
        let mut dist_matrix = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = vec![UNREACHABLE; n];
            row[i] = OrderedFloat(0.0);
            dist_matrix.push(ArraySeqStEphS::from_vec(row));
        }
        let distances = ArraySeqStEphS::from_vec(dist_matrix);

        let pred_matrix = vec![ArraySeqStEphS::new(n, NO_PREDECESSOR); n];
        let predecessors = ArraySeqStEphS::from_vec(pred_matrix);
        AllPairsResultStEphFloat {
            distances,
            predecessors,
            n,
        }
    }

    /// Returns the distance from vertex u to vertex v.
    pub fn get_distance(&self, u: usize, v: usize) -> OrderedF64 {
        if u >= self.n || v >= self.n {
            return UNREACHABLE;
        }
        *self.distances.nth(u).nth(v)
    }

    /// Sets the distance from vertex u to vertex v.
    pub fn set_distance(&mut self, u: usize, v: usize, dist: OrderedF64) {
        if u < self.n && v < self.n {
            let mut row = self.distances.nth(u).clone();
            let _ = row.set(v, dist);
            let _ = self.distances.set(u, row);
        }
    }

    /// Returns the predecessor of vertex v in the shortest path from u.
    pub fn get_predecessor(&self, u: usize, v: usize) -> Option<usize> {
        if u >= self.n || v >= self.n {
            return None;
        }
        let pred = *self.predecessors.nth(u).nth(v);
        if pred == NO_PREDECESSOR { None } else { Some(pred) }
    }

    /// Sets the predecessor of vertex v in the shortest path from u.
    pub fn set_predecessor(&mut self, u: usize, v: usize, pred: usize) {
        if u < self.n && v < self.n {
            let mut row = self.predecessors.nth(u).clone();
            let _ = row.set(v, pred);
            let _ = self.predecessors.set(u, row);
        }
    }

    /// Checks if vertex v is reachable from vertex u.
    pub fn is_reachable(&self, u: usize, v: usize) -> bool {
        self.get_distance(u, v).is_finite()
    }

    /// Extracts the shortest path from u to v by following predecessors.
    /// Returns None if v is unreachable from u, otherwise returns the path as a sequence.
    pub fn extract_path(&self, u: usize, v: usize) -> Option<ArraySeqStPerS<usize>> {
        if u == v {
            return Some(ArraySeqStPerS::from_vec(vec![u]));
        }
        if !self.is_reachable(u, v) {
            return None;
        }

        let mut path = Vec::new();
        let mut current = v;
        path.push(current);

        while current != u {
            let pred = *self.predecessors.nth(u).nth(current);
            if pred == NO_PREDECESSOR {
                return None;
            }
            path.push(pred);
            current = pred;
        }

        path.reverse();
        Some(ArraySeqStPerS::from_vec(path))
    }
}
