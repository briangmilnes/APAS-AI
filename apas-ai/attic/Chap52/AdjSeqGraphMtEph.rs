// //! Adjacency Sequence Graph - Ephemeral Multi-Threaded
// 
// pub mod AdjSeqGraphMtEph {
// 
// use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// 
// pub struct AdjSeqGraphMtEphS {
//     adj_seq: ArraySeqMtEphS<ArraySeqMtEphS<usize>>,
// }
// 
// impl AdjSeqGraphMtEphS {
//     pub fn new(n: usize) -> Self {
//         let empty_neighbors = ArraySeqMtEphS::new(0, 0usize);
//         AdjSeqGraphMtEphS {
//             adj_seq: ArraySeqMtEphS::new(n, empty_neighbors),
//         }
//     }
// 
//     pub fn from_adjacency_list(adj_list: Vec<Vec<usize>>) -> Self {
//         let n = adj_list.len();
//         let mut adj_seq = ArraySeqMtEphS::new(n, ArraySeqMtEphS::new(0, 0usize));
//         for (i, neighbors) in adj_list.into_iter().enumerate() {
//             let neighbor_seq = ArraySeqMtEphS::from_vec(neighbors);
//             adj_seq.update(Pair(i, neighbor_seq));
//         }
//         AdjSeqGraphMtEphS { adj_seq }
//     }
// 
//     pub fn has_edge(&self, u: usize, v: usize) -> bool {
//         let neighbors = self.adj_seq.nth(u);
//         (0..neighbors.length()).any(|i| neighbors.nth_cloned(i) == v)
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
// impl Default for AdjSeqGraphMtEphS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjSeqGraphMtEphS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjSeqGraphMtEph(v={}, e={})", self.vertex_count(), self.edge_count())
//     }
// }
// 
// } // end mod
