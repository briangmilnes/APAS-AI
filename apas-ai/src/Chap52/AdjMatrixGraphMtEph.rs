//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (ephemeral, multi-threaded).

pub mod AdjMatrixGraphMtEph {
    use std::sync::Arc;

    use crate::Types::Types::*;
    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtEph {
        matrix: ArraySeqMtEphS<ArraySeqMtEphS<bool>>,
        n: N,
    }

    pub trait AdjMatrixGraphMtEphTrait {
        fn new(n: N) -> Self;
        fn num_vertices(&self) -> N;
        fn num_edges(&self) -> N;
        fn has_edge(&self, u: N, v: N) -> B;
        fn out_neighbors(&self, u: N) -> ArraySeqMtEphS<N>;
        fn out_degree(&self, u: N) -> N;
        fn set_edge(&self, u: N, v: N, exists: B);
        fn complement(&self) -> Self;
    }

    impl AdjMatrixGraphMtEphTrait for AdjMatrixGraphMtEph {
        fn new(n: N) -> Self {
            let false_row = ArraySeqMtEphS::from_vec(vec![false; n]);
            let mut matrix_rows = Vec::with_capacity(n);
            for _ in 0..n {
                matrix_rows.push(false_row.clone());
            }
            AdjMatrixGraphMtEph {
                matrix: ArraySeqMtEphS::from_vec(matrix_rows),
                n,
            }
        }

        fn num_vertices(&self) -> N { self.n }

        fn num_edges(&self) -> N {
            let mut count = 0;
            for i in 0..self.n {
                let row = self.matrix.nth(i);
                for j in 0..self.n {
                    if *row.nth(j) {
                        count += 1;
                    }
                }
            }
            count
        }

        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        fn out_neighbors(&self, u: N) -> ArraySeqMtEphS<N> {
            if u >= self.n {
                return ArraySeqMtEphS::empty();
            }
            let row = self.matrix.nth(u);
            let mut neighbors = Vec::new();
            for v in 0..self.n {
                if *row.nth(v) {
                    neighbors.push(v);
                }
            }
            ArraySeqMtEphS::from_vec(neighbors)
        }

        fn out_degree(&self, u: N) -> N {
            if u >= self.n {
                return 0;
            }
            let row = self.matrix.nth(u);
            let mut count = 0;
            for v in 0..self.n {
                if *row.nth(v) {
                    count += 1;
                }
            }
            count
        }

        fn set_edge(&self, u: N, v: N, exists: B) {
            if u >= self.n || v >= self.n {
                return;
            }
            let row = self.matrix.nth(u);
            let _ = row.set(v, exists);
        }

        fn complement(&self) -> Self {
            let mut new_matrix_vec = Vec::with_capacity(self.n);
            for i in 0..self.n {
                let row = self.matrix.nth(i);
                let mut new_row_vec = Vec::with_capacity(self.n);
                for j in 0..self.n {
                    if i == j {
                        new_row_vec.push(false);
                    } else {
                        new_row_vec.push(!*row.nth(j));
                    }
                }
                new_matrix_vec.push(ArraySeqMtEphS::from_vec(new_row_vec));
            }
            AdjMatrixGraphMtEph {
                matrix: ArraySeqMtEphS::from_vec(new_matrix_vec),
                n: self.n,
            }
        }
    }
}
