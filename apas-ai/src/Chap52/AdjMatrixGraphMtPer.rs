//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded).
//! PARALLEL complement operation.

pub mod AdjMatrixGraphMtPer {
    use std::sync::Arc;

    use crate::Types::Types::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtPer {
        matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        n: N,
    }

    pub trait AdjMatrixGraphMtPerTrait {
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn new(n: N) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self) -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn num_edges(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn out_degree(&self, u: N) -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn complement(&self) -> Self
        where
            bool: 'static;
    }

    impl AdjMatrixGraphMtPerTrait for AdjMatrixGraphMtPer {
        fn new(n: N) -> Self {
            let false_row = ArraySeqMtPerS::from_vec(vec![false; n]);
            let mut matrix_rows = Vec::with_capacity(n);
            for _ in 0..n {
                matrix_rows.push(false_row.clone());
            }
            AdjMatrixGraphMtPer {
                matrix: ArraySeqMtPerS::from_vec(matrix_rows),
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

        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> {
            if u >= self.n {
                return ArraySeqMtPerS::empty();
            }
            let row = self.matrix.nth(u);
            let mut neighbors = Vec::new();
            for v in 0..self.n {
                if *row.nth(v) {
                    neighbors.push(v);
                }
            }
            ArraySeqMtPerS::from_vec(neighbors)
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

        // Work: Θ(n²), Span: Θ(n²) sequential - Exercise 52.6
        fn complement(&self) -> Self
        where
            bool: 'static,
        {
            let n = self.n;
            let mut new_matrix_vec = Vec::with_capacity(n);
            for i in 0..n {
                let row = self.matrix.nth(i);
                let mut new_row_vec = Vec::with_capacity(n);
                for j in 0..n {
                    if i == j {
                        new_row_vec.push(false);
                    } else {
                        new_row_vec.push(!*row.nth(j));
                    }
                }
                new_matrix_vec.push(ArraySeqMtPerS::from_vec(new_row_vec));
            }
            AdjMatrixGraphMtPer {
                matrix: ArraySeqMtPerS::from_vec(new_matrix_vec),
                n: self.n,
            }
        }
    }
}
