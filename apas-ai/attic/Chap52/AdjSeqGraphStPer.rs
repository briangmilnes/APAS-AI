// //! Adjacency Sequence Graph - Persistent Single-Threaded (Enumerable Graphs)
// 
// pub mod AdjSeqGraphStPer {
// 
// use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// pub struct AdjSeqGraphStPerS {
//     adj_seq: ArraySeqStPerS<ArraySeqStPerS<usize>>,
// }
// 
// impl AdjSeqGraphStPerS {
//     pub fn new(n: usize) -> Self {
//         let empty_neighbors = ArraySeqStPerS::new(0, 0usize);
//         AdjSeqGraphStPerS {
//             adj_seq: ArraySeqStPerS::new(n, empty_neighbors),
//         }
//     }
// 
// //     pub fn from_adjacency_list(adj_list: Vec<Vec<usize>>) -> Self {
// //         let n = adj_list.len();
// //         let mut adj_seq = ArraySeqStPerS::new(n, ArraySeqStPerS::new(0, 0usize));
// //         for (i, neighbors) in adj_list.into_iter().enumerate() {
// //             let neighbor_seq = ArraySeqStPerS::from_vec(neighbors);
// //             adj_seq = adj_seq.update(i, neighbor_seq);
// //         }
// //         AdjSeqGraphStPerS { adj_seq }
// //     }
// 
// //     pub fn insert_edge(self, u: usize, v: usize) -> Self {
// //         let neighbors = self.adj_seq.nth(u).append_elem(v);
// //         AdjSeqGraphStPerS {
// //             adj_seq: self.adj_seq.update(u, neighbors),
// //         }
// //     }
// 
//     pub fn has_edge(&self, u: usize, v: usize) -> bool {
//         let neighbors = self.adj_seq.nth(u);
//         (0..neighbors.length()).any(|i| *neighbors.nth(i) == v)
//     }
// 
// //     pub fn out_degree(&self, v: usize) -> usize {
// //         self.adj_seq.nth(v).length()
// //     }
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
// //     pub fn out_neighbors(&self, v: usize) -> ArraySeqStPerS<usize> {
// //         self.adj_seq.nth(v).clone()
// //     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.adj_seq.length() == 0
//     }
// }
// 
// impl Default for AdjSeqGraphStPerS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjSeqGraphStPerS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjSeqGraphStPer(v={}, e={})", self.vertex_count(), self.edge_count())
//     }
// }
// 
// } // end mod
