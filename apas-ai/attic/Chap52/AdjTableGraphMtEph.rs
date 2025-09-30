// // //! Adjacency Table Graph - Ephemeral Multi-Threaded Implementation
// // 
// // pub mod AdjTableGraphMtEph {
// // 
// // use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
// // use crate::Chap43Claude::OrderedTableStEph::OrderedTableStEph::*;
// // use crate::Types::Types::*;
// // use std::fmt::{Debug, Display};
// // use std::sync::Arc;
// // use std::thread;
// // 
// // 
// // pub struct AdjTableGraphMtEphS<V: MtT + Ord + Clone + Debug + Display> {
// //     adj_table: OrderedTableStEph<V, AVLTreeSetStEph<V>>,
// // }
// // 
// // impl<V: MtT + Ord + Clone + Debug + Display> AdjTableGraphMtEphS<V> {
// //     pub fn new() -> Self {
// //         AdjTableGraphMtEphS {
// //             adj_table: OrderedTableStEph::empty(),
// //         }
// //     }
// // 
// //     pub fn insert_vertex(&mut self, v: V) {
// //         self.adj_table.insert(v, AVLTreeSetStEph::empty());
// //     }
// // 
// //     pub fn insert_edge(&mut self, u: V, v: V) {
// //         if let Some(neighbors) = self.adj_table.find(&u) {
// //             let mut neighbors = neighbors.clone();
// //             neighbors.insert(v);
// //             self.adj_table.insert(u, neighbors);
// //         } else {
// //             let mut neighbors = AVLTreeSetStEph::empty();
// //             neighbors.insert(v);
// //             self.adj_table.insert(u, neighbors);
// //         }
// //     }
// // 
// //     pub fn has_edge(&self, u: &V, v: &V) -> bool {
// //         self.adj_table.find(u).map(|n| n.find(v)).unwrap_or(false)
// //     }
// // 
// //     pub fn vertex_count(&self) -> usize {
// //         self.adj_table.size()
// //     }
// // 
// //     pub fn edge_count(&self) -> usize {
// //         self.adj_table.iter().map(|(_, n)| n.size()).sum()
// //     }
// // 
// // //     pub fn map_vertices_parallel<F, R>(&self, f: F) -> Vec<R>
// // //     where
// // //         F: Fn(&V) -> R + Send + Sync,
// // //         R: Send,
// // //     {
// // //         let vertices: Vec<V> = self.adj_table.iter().map(|(v, _)| v).cloned().collect();
// // //         let chunk_size = (vertices.len() / 4).max(1);
// // //         let f = Arc::new(f);
// // //         
// // //         let handles: Vec<_> = vertices
// // //             .chunks(chunk_size)
// // //             .map(|chunk| {
// // //                 let chunk = chunk.to_vec();
// // //                 let f = Arc::clone(&f);
// // //                 thread::spawn(move || chunk.iter().map(|v| f(v)).collect::<Vec<_>>())
// // //             })
// // //             .collect();
// // //         
// // //         handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
// // //     }
// // 
// //     pub fn is_empty(&self) -> bool {
// //         self.adj_table.size() == 0
// //     }
// // }
// // 
// // impl<V: MtT + Ord + Clone + Debug + Display> Default for AdjTableGraphMtEphS<V> {
// //     fn default() -> Self {
// //         Self::new()
// //     }
// // }
// // 
// // impl<V: MtT + Display> Display for AdjTableGraphMtEphS<V> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         write!(f, "AdjTableGraphMtEph(v={}, e={})", self.vertex_count(), self.edge_count())
// //     }
// // }
// // 
// // } // end mod
