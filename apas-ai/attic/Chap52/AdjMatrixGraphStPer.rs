// //! Adjacency Matrix Graph - Persistent Single-Threaded
// 
// pub mod AdjMatrixGraphStPer {
// 
// use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// pub struct AdjMatrixGraphStPerS {
//     matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>,
//     n: usize,
// }
// 
// impl AdjMatrixGraphStPerS {
//     pub fn new(n: usize) -> Self {
//         let row = ArraySeqStPerS::new(n, false);
//         AdjMatrixGraphStPerS {
//             matrix: ArraySeqStPerS::new(n, row),
//             n,
//         }
//     }
// 
// //     pub fn from_matrix(matrix_data: Vec<Vec<bool>>) -> Self {
// //         let n = matrix_data.len();
// //         let mut matrix = ArraySeqStPerS::new(n, ArraySeqStPerS::new(n, false));
// //         for (i, row) in matrix_data.into_iter().enumerate() {
// //             let row_seq = ArraySeqStPerS::from_vec(row);
// //             matrix = matrix.update(i, row_seq);
// //         }
// //         AdjMatrixGraphStPerS { matrix, n }
// //     }
// 
// //     pub fn insert_edge(self, u: usize, v: usize) -> Self {
// //         let row = self.matrix.nth(u).update(v, true);
// //         AdjMatrixGraphStPerS {
// //             matrix: self.matrix.update(u, row),
// //             n: self.n,
// //         }
// //     }
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
//         (0..self.n)
//             .map(|i| self.out_degree(i))
//             .sum()
//     }
// 
// //     pub fn complement(self) -> Self {
// //         let mut new_matrix = self.matrix.clone();
// //         for i in 0..self.n {
// //             let row = self.matrix.nth(i);
// //             let mut new_row = row.clone();
// //             for j in 0..self.n {
// //                 if i != j {
// //                     new_row = new_row.update(j, !*row.nth(j));
// //                 }
// //             }
// //             new_matrix = new_matrix.update(i, new_row);
// //         }
// //         AdjMatrixGraphStPerS {
// //             matrix: new_matrix,
// //             n: self.n,
// //         }
// //     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.n == 0
//     }
// }
// 
// impl Default for AdjMatrixGraphStPerS {
//     fn default() -> Self {
//         Self::new(0)
//     }
// }
// 
// impl Display for AdjMatrixGraphStPerS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjMatrixGraphStPer(n={}, e={})", self.n, self.edge_count())
//     }
// }
// 
// } // end mod
