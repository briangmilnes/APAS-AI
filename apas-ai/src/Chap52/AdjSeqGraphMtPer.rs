//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded).
//! PARALLEL operations via ArraySeqMtPer.

pub mod AdjSeqGraphMtPer {
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;
    use std::sync::Arc;

    #[derive(Clone)]
    pub struct AdjSeqGraphMtPer {
        adj: ArraySeqMtPerS<ArraySeqMtPerS<N>>,
    }

    pub trait AdjSeqGraphMtPerTrait {
        fn new(n: N) -> Self;
        fn num_vertices(&self) -> N;
        fn num_edges(&self) -> N;
        fn has_edge(&self, u: N, v: N) -> B;
        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N>;
        fn out_degree(&self, u: N) -> N;
        fn map_vertices<F: Fn(N) -> N + Send + Sync + Clone + 'static>(&self, f: F) -> Self where N: 'static;
    }

    impl AdjSeqGraphMtPerTrait for AdjSeqGraphMtPer {
        fn new(n: N) -> Self {
            let empty_list = ArraySeqMtPerS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphMtPer {
                adj: ArraySeqMtPerS::from_vec(adj_lists),
            }
        }

        fn num_vertices(&self) -> N {
            self.adj.length()
        }

        // Work: Θ(n), Span: Θ(n) - sum all neighbor list lengths
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

        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> {
            self.adj.nth(u).clone()
        }

        fn out_degree(&self, u: N) -> N {
            self.adj.nth(u).length()
        }

        // Work: Θ(n+m), Span: Θ(n) - map over all vertices
        fn map_vertices<F: Fn(N) -> N + Send + Sync + Clone + 'static>(&self, f: F) -> Self where N: 'static {
            let n = self.adj.length();
            let mut new_rows = Vec::with_capacity(n);
            for i in 0..n {
                let row = self.adj.nth(i);
                let m = row.length();
                let mut new_row = Vec::with_capacity(m);
                for j in 0..m {
                    new_row.push(f(*row.nth(j)));
                }
                new_rows.push(ArraySeqMtPerS::from_vec(new_row));
            }
            AdjSeqGraphMtPer {
                adj: ArraySeqMtPerS::from_vec(new_rows),
            }
        }
    }
}
