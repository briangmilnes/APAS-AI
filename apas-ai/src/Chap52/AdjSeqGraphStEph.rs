//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (ephemeral, single-threaded).

pub mod AdjSeqGraphStEph {

    use crate::Types::Types::*;
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;

    #[derive(Clone)]
    pub struct AdjSeqGraphStEph {
        adj: ArraySeqStEphS<ArraySeqStEphS<N>>,
    }

    pub trait AdjSeqGraphStEphTrait {
        fn new(n: N) -> Self;
        fn from_seq(adj: ArraySeqStEphS<ArraySeqStEphS<N>>) -> Self;
        fn num_vertices(&self) -> N;
        fn num_edges(&self) -> N;
        fn has_edge(&self, u: N, v: N) -> B;
        fn out_neighbors(&self, u: N) -> ArraySeqStEphS<N>;
        fn out_degree(&self, u: N) -> N;
        fn set_neighbors(&mut self, v: N, neighbors: ArraySeqStEphS<N>);
        fn set_edge(&mut self, u: N, v: N, exists: B);
    }

    impl AdjSeqGraphStEphTrait for AdjSeqGraphStEph {
        fn new(n: N) -> Self {
            let empty_list = ArraySeqStEphS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphStEph {
                adj: ArraySeqStEphS::from_vec(adj_lists),
            }
        }

        fn from_seq(adj: ArraySeqStEphS<ArraySeqStEphS<N>>) -> Self { AdjSeqGraphStEph { adj } }

        fn num_vertices(&self) -> N { self.adj.length() }

        fn num_edges(&self) -> N {
            let n = self.adj.length();
            let mut count = 0;
            for i in 0..n {
                count += self.adj.nth(i).length();
            }
            count
        }

        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.adj.length() {
                return false;
            }
            let neighbors = self.adj.nth(u);
            for i in 0..neighbors.length() {
                if *neighbors.nth(i) == v {
                    return true;
                }
            }
            false
        }

        fn out_neighbors(&self, u: N) -> ArraySeqStEphS<N> { self.adj.nth(u).clone() }

        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length() }

        fn set_neighbors(&mut self, v: N, neighbors: ArraySeqStEphS<N>) {
            if v < self.adj.length() {
                let _ = self.adj.set(v, neighbors);
            }
        }

        fn set_edge(&mut self, u: N, v: N, exists: B) {
            if u >= self.adj.length() || v >= self.adj.length() {
                return;
            }
            let old_neighbors = self.adj.nth(u);
            if exists {
                // Add edge if not present
                let mut found = false;
                for i in 0..old_neighbors.length() {
                    if *old_neighbors.nth(i) == v {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut new_neighbors_vec: Vec<N> = Vec::with_capacity(old_neighbors.length() + 1);
                    for i in 0..old_neighbors.length() {
                        new_neighbors_vec.push(*old_neighbors.nth(i));
                    }
                    new_neighbors_vec.push(v);
                    let _ = self.adj.set(u, ArraySeqStEphS::from_vec(new_neighbors_vec));
                }
            } else {
                // Remove edge if present
                let mut new_neighbors_vec: Vec<N> = Vec::new();
                for i in 0..old_neighbors.length() {
                    let neighbor = *old_neighbors.nth(i);
                    if neighbor != v {
                        new_neighbors_vec.push(neighbor);
                    }
                }
                let _ = self.adj.set(u, ArraySeqStEphS::from_vec(new_neighbors_vec));
            }
        }
    }
}
