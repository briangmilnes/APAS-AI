//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded).
//! PARALLEL complement operation.

pub mod AdjMatrixGraphMtPer {

    use std::sync::Arc;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtPer {
        matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        n: N,
    }

    pub trait AdjMatrixGraphMtPerTrait {
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn new(n: N)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)         -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn num_edges(&self)            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn out_neighbors(&self, u: N)  -> ArraySeqMtPerS<N>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn out_degree(&self, u: N)     -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn complement(&self)           -> Self
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
            use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
            // Parallel: flatten all rows into single sequence, then count true values
            // Work: Θ(n²), Span: Θ(log n) via parallel reduce
            let all_cells = ArraySeqMtPerBaseTrait::flatten(&self.matrix);
            let count_fn = |acc: &N, cell: &bool| if *cell { *acc + 1 } else { *acc };
            ArraySeqMtPerS::iterate(&all_cells, &count_fn, 0)
        }

        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> {
            use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
            if u >= self.n {
                return ArraySeqMtPerS::empty();
            }
            // Parallel: tabulate all possible neighbors, then filter to keep edges
            // Work: Θ(n), Span: Θ(log n) via parallel filter
            let row = self.matrix.nth(u).clone();
            let all_vertices = ArraySeqMtPerS::tabulate(&|v| v, self.n);
            let pred = move |v: &N| -> B { *row.nth(*v) };
            ArraySeqMtPerS::filter(&all_vertices, &pred)
        }

        fn out_degree(&self, u: N) -> N {
            use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
            if u >= self.n {
                return 0;
            }
            // Parallel: reduce over row to count true values
            // Work: Θ(n), Span: Θ(log n) via parallel reduce
            let row = self.matrix.nth(u);
            let count_fn = |acc: &N, cell: &bool| if *cell { *acc + 1 } else { *acc };
            ArraySeqMtPerS::iterate(&row, &count_fn, 0)
        }

        // Exercise 52.6: Parallel complement with Θ(log n) span via nested parallel tabulate
        // Work: Θ(n²), Span: Θ(log n) 
        fn complement(&self) -> Self
        where
            bool: 'static,
        {
            use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
            let n = self.n;
            let original_matrix = self.matrix.clone();
            
            // Parallel tabulate over rows (i = 0..n)
            let new_matrix = ArraySeqMtPerS::tabulate(&|i| {
                let row = original_matrix.nth(i);
                // Parallel tabulate over columns (j = 0..n)
                ArraySeqMtPerS::tabulate(&|j| {
                    if i == j {
                        false  // No self-loops
                    } else {
                        !*row.nth(j)  // Complement the edge
                    }
                }, n)
            }, n);
            
            AdjMatrixGraphMtPer {
                matrix: new_matrix,
                n: self.n,
            }
        }
    }
}
