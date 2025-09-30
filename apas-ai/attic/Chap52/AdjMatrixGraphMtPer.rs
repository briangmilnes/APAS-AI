// //! Adjacency Matrix Graph - Persistent Multi-Threaded
// 
// pub mod AdjMatrixGraphMtPer {
// 
// use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// pub struct AdjMatrixGraphMtPerS {
//     matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
//     n: usize,
// }
// 
// impl AdjMatrixGraphMtPerS {
//     pub fn new(n: usize) -> Self {
//         let row = ArraySeqMtPerS::new(n, false);
//         AdjMatrixGraphMtPerS {
//             matrix: ArraySeqMtPerS::new(n, row),
//             n,
//         }
//     }
// 
//     pub fn has_edge(&self, u: usize, v: usize) -> bool {
//         *self.matrix.nth(u).nth(v)
//     }
// 
// //     pub fn out_degree(&self, v: usize) -> usize {
// //         let row = self.matrix.nth(v);
// //         (0..self.n).filter(|&i| *row.nth(i)).count()
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
// impl Default for AdjMatrixGraphMtPerS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjMatrixGraphMtPerS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjMatrixGraphMtPer(n={}, e={})", self.n, self.edge_count())
//     }
// }
// 
// } // end mod
