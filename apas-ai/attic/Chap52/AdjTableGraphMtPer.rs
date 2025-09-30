// //! Adjacency Table Graph - Persistent Multi-Threaded Implementation
// 
// pub mod AdjTableGraphMtPer {
// 
// use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
// use crate::Chap43Claude::OrderedTableStPer::OrderedTableStPer::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// use std::sync::Arc;
// use std::thread;
// 
// pub struct AdjTableGraphMtPerS<V: MtT + Ord + Clone + Debug + Display> {
//     adj_table: OrderedTableStPer<V, AVLTreeSetStPer<V>>,
// }
// 
// impl<V: MtT + Ord + Clone + Debug + Display> AdjTableGraphMtPerS<V> {
//     pub fn new() -> Self {
//         AdjTableGraphMtPerS {
//             adj_table: OrderedTableStPer::empty(),
//         }
//     }
// 
//     pub fn insert_vertex(self, v: V) -> Self {
//         AdjTableGraphMtPerS {
//             adj_table: self.adj_table.insert(v, AVLTreeSetStPer::empty()),
//         }
//     }
// 
//     pub fn insert_edge(self, u: V, v: V) -> Self {
//         let neighbors = self.adj_table
//             .find(&u)
//             .map(|s| s.clone())
//             .unwrap_or_else(|| AVLTreeSetStPer::empty())
//             .insert(v);
//         AdjTableGraphMtPerS {
//             adj_table: self.adj_table.insert(u, neighbors),
//         }
//     }
// 
//     pub fn has_edge(&self, u: &V, v: &V) -> bool {
//         self.adj_table.find(u).map(|n| n.find(v)).unwrap_or(false)
//     }
// 
//     pub fn vertex_count(&self) -> usize {
//         self.adj_table.size()
//     }
// 
//     pub fn edge_count(&self) -> usize {
//         self.adj_table.iter().map(|(_, n)| n.size()).sum()
//     }
// 
// //     pub fn map_vertices_parallel<F, R>(&self, f: F) -> Vec<R>
// //     where
// //         F: Fn(&V) -> R + Send + Sync,
// //         R: Send,
// //     {
// //         let vertices: Vec<V> = self.adj_table.iter().map(|(v, _)| v).cloned().collect();
// //         let chunk_size = (vertices.len() / 4).max(1);
// //         let f = Arc::new(f);
// //         
// //         let handles: Vec<_> = vertices
// //             .chunks(chunk_size)
// //             .map(|chunk| {
// //                 let chunk = chunk.to_vec();
// //                 let f = Arc::clone(&f);
// //                 thread::spawn(move || chunk.iter().map(|v| f(v)).collect::<Vec<_>>())
// //             })
// //             .collect();
// //         
// //         handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
// //     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.adj_table.size() == 0
//     }
// }
// 
// impl<V: MtT + Ord + Clone + Debug + Display> Default for AdjTableGraphMtPerS<V> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// 
// impl<V: MtT + Display> Display for AdjTableGraphMtPerS<V> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjTableGraphMtPer(v={}, e={})", self.vertex_count(), self.edge_count())
//     }
// }
// 
// } // end mod
