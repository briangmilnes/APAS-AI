// //! Adjacency Matrix Graph - Ephemeral Multi-Threaded
// 
// pub mod AdjMatrixGraphMtEph {
// 
// use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// 
// pub struct AdjMatrixGraphMtEphS {
//     matrix: ArraySeqMtEphS<ArraySeqMtEphS<bool>>,
//     n: usize,
// }
// 
// impl AdjMatrixGraphMtEphS {
//     pub fn new(n: usize) -> Self {
//         let row = ArraySeqMtEphS::new(n, false);
//         AdjMatrixGraphMtEphS {
//             matrix: ArraySeqMtEphS::new(n, row),
//             n,
//         }
//     }
// 
//     pub fn has_edge(&self, u: usize, v: usize) -> bool {
//         self.matrix.nth(u).nth_cloned(v)
//     }
// 
// //     pub fn out_degree(&self, v: usize) -> usize {
// //         let row = self.matrix.nth(v);
// //         (0..self.n).filter(|&i| row.nth_cloned(i)).count()
// //     }
// 
//     pub fn vertex_count(&self) -> usize {
//         self.n
//     }
// 
//     pub fn edge_count(&self) -> usize {
//         (0..self.n).map(|i| self.out_degree(i)).sum()
//     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.n == 0
//     }
// }
// 
// impl Default for AdjMatrixGraphMtEphS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjMatrixGraphMtEphS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjMatrixGraphMtEph(n={}, e={})", self.n, self.edge_count())
//     }
// }
// 
// } // end mod
