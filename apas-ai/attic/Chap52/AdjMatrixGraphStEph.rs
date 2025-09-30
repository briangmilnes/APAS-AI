// //! Adjacency Matrix Graph - Ephemeral Single-Threaded
// 
// pub mod AdjMatrixGraphStEph {
// 
// use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// 
// pub struct AdjMatrixGraphStEphS {
//     matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>,
//     n: usize,
// }
// 
// impl AdjMatrixGraphStEphS {
//     pub fn new(n: usize) -> Self {
//         let row = ArraySeqStEphS::new(n, false);
//         AdjMatrixGraphStEphS {
//             matrix: ArraySeqStEphS::new(n, row),
//             n,
//         }
//     }
// 
//     pub fn insert_edge(&mut self, u: usize, v: usize) {
//         let row = self.matrix.nth(u).update(Pair(v, true));
//         self.matrix.update(Pair(u, row));
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
//     pub fn complement(&mut self) {
//         for i in 0..self.n {
//             let row = self.matrix.nth(i);
//             let mut new_row = row.clone();
//             for j in 0..self.n {
//                 if i != j {
//                     new_row.update(Pair(j, !*row.nth(j)));
//                 }
//             }
//             self.matrix.update(Pair(i, new_row));
//         }
//     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.n == 0
//     }
// }
// 
// impl Default for AdjMatrixGraphStEphS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjMatrixGraphStEphS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjMatrixGraphStEph(n={}, e={})", self.n, self.edge_count())
//     }
// }
// 
// } // end mod
