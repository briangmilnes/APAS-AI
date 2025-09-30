// //! Adjacency Table Graph - Ephemeral Single-Threaded Implementation
// 
// pub mod AdjTableGraphStEph {
// 
// use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
// use crate::Chap43Claude::OrderedTableStEph::OrderedTableStEph::*;
// use crate::Types::Types::*;
// use std::fmt::{Debug, Display};
// 
// 
// pub struct AdjTableGraphStEphS<V: StT + Ord + Clone + Debug + Display> {
//     adj_table: OrderedTableStEph<V, AVLTreeSetStEph<V>>,
// }
// 
// impl<V: StT + Ord + Clone + Debug + Display> AdjTableGraphStEphS<V> {
//     pub fn new() -> Self {
//         AdjTableGraphStEphS {
//             adj_table: OrderedTableStEph::empty(),
//         }
//     }
// 
//     pub fn insert_vertex(&mut self, v: V) {
//         self.adj_table.insert(v, AVLTreeSetStEph::empty());
//     }
// 
//     pub fn delete_vertex(&mut self, v: &V) {
//         self.adj_table.delete(v);
//     }
// 
//     pub fn insert_edge(&mut self, u: V, v: V) {
//         if let Some(neighbors) = self.adj_table.find(&u) {
//             let mut neighbors = neighbors.clone();
//             neighbors.insert(v);
//             self.adj_table.insert(u, neighbors);
//         } else {
//             let mut neighbors = AVLTreeSetStEph::empty();
//             neighbors.insert(v);
//             self.adj_table.insert(u, neighbors);
//         }
//     }
// 
//     pub fn delete_edge(&mut self, u: &V, v: &V) {
//         if let Some(neighbors) = self.adj_table.find(u) {
//             let mut neighbors = neighbors.clone();
//             neighbors.delete(v);
//             self.adj_table.insert(*u, neighbors);
//         }
//     }
// 
//     pub fn has_edge(&self, u: &V, v: &V) -> bool {
//         self.adj_table
//             .find(u)
//             .map(|n| n.find(v))
//             .unwrap_or(false)
//     }
// 
// //     pub fn out_degree(&self, v: &V) -> usize {
// //         self.adj_table.find(v).map(|n| n.size()).unwrap_or(0)
// //     }
// 
//     pub fn vertex_count(&self) -> usize {
//         self.adj_table.size()
//     }
// 
//     pub fn edge_count(&self) -> usize {
//         self.adj_table.iter().map(|(_, n)| n.size()).sum()
//     }
// 
//     pub fn is_empty(&self) -> bool {
//         self.adj_table.size() == 0
//     }
// 
//     pub fn adj_table(&self) -> &OrderedTableStEph<V, AVLTreeSetStEph<V>> {
//         &self.adj_table
//     }
// }
// 
// impl<V: StT + Ord + Clone + Debug + Display> Default for AdjTableGraphStEphS<V> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// 
// impl<V: StT + Display> Display for AdjTableGraphStEphS<V> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AdjTableGraphStEph(v={}, e={})", self.vertex_count(), self.edge_count())
//     }
// }
// 
// } // end mod
