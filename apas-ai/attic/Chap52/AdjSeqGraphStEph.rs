// //! Adjacency Sequence Graph - Ephemeral Single-Threaded
// 
// pub mod AdjSeqGraphStEph {
// 
// use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// 
// pub struct AdjSeqGraphStEphS {
//     adj_seq: ArraySeqStEphS<ArraySeqStEphS<usize>>,
// }
// 
// impl AdjSeqGraphStEphS {
//     pub fn new(n: usize) -> Self {
//         let empty_neighbors = ArraySeqStEphS::new(0, 0usize);
//         AdjSeqGraphStEphS {
//             adj_seq: ArraySeqStEphS::new(n, empty_neighbors),
//         }
//     }
// 
//     pub fn from_adjacency_list(adj_list: Vec<Vec<usize>>) -> Self {
//         let n = adj_list.len();
//         let mut adj_seq = ArraySeqStEphS::new(n, ArraySeqStEphS::new(0, 0usize));
//         for (i, neighbors) in adj_list.into_iter().enumerate() {
//             let neighbor_seq = ArraySeqStEphS::from_vec(neighbors);
//             adj_seq.update(Pair(i, neighbor_seq));
//         }
//         AdjSeqGraphStEphS { adj_seq }
//     }
// 
//     pub fn insert_edge(&mut self, u: usize, v: usize) {
//         let neighbors = self.adj_seq.nth(u).append_elem(v);
//         self.adj_seq.update(Pair(u, neighbors));
//     }
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
//     pub fn is_empty(&self) -> bool {
//         self.adj_seq.length() == 0
//     }
// }
// 
// impl Default for AdjSeqGraphStEphS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjSeqGraphStEphS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjSeqGraphStEph(v={}, e={})", self.vertex_count(), self.edge_count())
//     }
// }
// 
// } // end mod
