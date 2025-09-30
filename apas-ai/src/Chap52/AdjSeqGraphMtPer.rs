// //! Adjacency Sequence Graph - Persistent Multi-Threaded
// 
// pub mod AdjSeqGraphMtPer {
// 
// use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// use std::sync::Arc;
// use std::thread;
// 
// pub struct AdjSeqGraphMtPerS {
//     adj_seq: ArraySeqMtPerS<ArraySeqMtPerS<usize>>,
// }
// 
// impl AdjSeqGraphMtPerS {
//     pub fn new(n: usize) -> Self {
//         let empty_neighbors = ArraySeqMtPerS::new(0, 0usize);
//         AdjSeqGraphMtPerS {
//             adj_seq: ArraySeqMtPerS::new(n, empty_neighbors),
//         }
//     }
// 
//     pub fn from_adjacency_list(adj_list: Vec<Vec<usize>>) -> Self {
//         let n = adj_list.len();
//         let mut adj_seq = ArraySeqMtPerS::new(n, ArraySeqMtPerS::new(0, 0usize));
//         for (i, neighbors) in adj_list.into_iter().enumerate() {
//             let neighbor_seq = ArraySeqMtPerS::from_vec(neighbors);
//             adj_seq = adj_seq.update(i, neighbor_seq);
//         }
//         AdjSeqGraphMtPerS { adj_seq }
//     }
// 
//     pub fn has_edge(&self, u: usize, v: usize) -> bool {
//         let neighbors = self.adj_seq.nth(u);
//         (0..neighbors.length()).any(|i| *neighbors.nth(i) == v)
//     }
// 
//     pub fn vertex_count(&self) -> usize {
//         self.adj_seq.length()
//     }
// 
//     pub fn edge_count(&self) -> usize {
//         (0..self.adj_seq.length())
//             .map(|i| self.adj_seq.nth(i).length())
//             .sum()
//     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.adj_seq.length() == 0
//     }
// }
// 
// impl Default for AdjSeqGraphMtPerS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjSeqGraphMtPerS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjSeqGraphMtPer(v={}, e={})", self.vertex_count(), self.edge_count())
//     }
// }
// 
// } // end mod
