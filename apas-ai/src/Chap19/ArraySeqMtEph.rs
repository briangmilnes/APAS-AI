//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph {

    use std::sync::Mutex;
    use std::thread;

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{
        ArraySeqMtEphS as S,
        ArraySeqMtEphTrait as Chap18Trait,
    };
    use crate::Types::Types::*;

    pub type ArraySeqMtEphS<T> = S<T>;

    pub trait ArraySeqMtEphTrait<T: StTInMtT>: Chap18Trait<T> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>)    -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate<F: PredMt<T>>(f: &F, x: &T)                            -> Self;
    }

    impl<T: StTInMtT + 'static> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {
            let len_a = a.length();
            if index < len_a {
                return Some(a.nth_cloned(index));
            }
            let offset = index - len_a;
            let len_b = b.length();
            if offset < len_b {
                Some(b.nth_cloned(offset))
            } else {
                None
            }
        }

        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
            <ArraySeqMtEphS<T> as Chap18Trait<T>>::tabulate(
                &|i| <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::select(a, b, i).unwrap(),
                a.length() + b.length(),
            )
        }

        fn deflate<F: PredMt<T>>(f: &F, x: &T) -> ArraySeqMtEphS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) {
                <ArraySeqMtEphS<T> as Chap18Trait<T>>::singleton(x.clone())
            } else {
                <ArraySeqMtEphS<T> as Chap18Trait<T>>::empty()
            }
        }
    }
}
