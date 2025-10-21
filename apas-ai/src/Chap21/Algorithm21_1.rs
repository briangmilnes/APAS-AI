//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.1: 2D Points using ArraySeqPer via tabulate + flatten.

pub mod Algorithm21_1 {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Algorithm21_1Trait {
        /// Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten
        /// APAS: Work Θ(n²), Span Θ(lg n)
        fn points2d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, N>>;
    }

    /// Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten.
    /// Functional form: points2D n = flatten (tabulate (\x. tabulate (\y. (x, y+1)) (n-1)) n)
    ///
    /// Generates all 2D points (x, y) where 0 ≤ x < n and 1 ≤ y < n.
    /// gpt-5-hard: Work: Θ(n²), Span: Θ(lg n)
    /// APAS: Work: Θ(n²), Span: Θ(lg n)
    pub fn points2d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, N>> {
        if n == 0 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let inner: ArraySeqStPerS<ArraySeqStPerS<Pair<N, N>>> =
            <ArraySeqStPerS<ArraySeqStPerS<Pair<N, N>>> as ArraySeqStPerRedefinableTrait<ArraySeqStPerS<Pair<N, N>>>>::tabulate(
                &|x| {
                    <ArraySeqStPerS<Pair<N, N>> as ArraySeqStPerRedefinableTrait<Pair<N, N>>>::tabulate(&|y| Pair(x, y + 1), n - 1)
                },
                n,
            );
        <ArraySeqStPerS<Pair<N, N>> as ArraySeqStPerBaseTrait<Pair<N, N>>>::flatten(&inner)
    }
}
