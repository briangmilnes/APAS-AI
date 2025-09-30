//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer via reduce pattern - sequential implementation (Chapter 26, Section 5).

pub mod DivConReduceSt {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};
    use crate::Types::Types::*;

    /// Divide-and-conquer using reduce pattern (sequential).
    pub trait DivConReduceStTrait {
        /// Find maximum element via reduce.
        /// Pattern: reduce max MIN identity
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn max_element(a: &ArraySeqStPerS<N>) -> Option<N>;

        /// Sum all elements via reduce.
        /// Pattern: reduce (+) 0 identity
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn sum(a: &ArraySeqStPerS<N>) -> N;

        /// Product of all elements via reduce.
        /// Pattern: reduce (*) 1 identity
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn product(a: &ArraySeqStPerS<N>) -> N;

        /// Logical OR of all elements via reduce.
        /// Pattern: reduce (||) false identity
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn any(a: &ArraySeqStPerS<B>) -> B;

        /// Logical AND of all elements via reduce.
        /// Pattern: reduce (&&) true identity
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn all(a: &ArraySeqStPerS<B>) -> B;
    }

    impl DivConReduceStTrait for ArraySeqStPerS<N> {
        /// Find maximum element using sequential reduce.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn max_element(a: &ArraySeqStPerS<N>) -> Option<N> {
            if a.length() == 0 {
                return None;
            }
            Some(ArraySeqStPerS::reduce(a, &|x, y| (*x).max(*y), *a.nth(0)))
        }

        /// Sum all elements using sequential reduce.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn sum(a: &ArraySeqStPerS<N>) -> N {
            ArraySeqStPerS::reduce(a, &|x, y| x + y, 0)
        }

        /// Product of all elements using sequential reduce.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn product(a: &ArraySeqStPerS<N>) -> N {
            ArraySeqStPerS::reduce(a, &|x, y| x * y, 1)
        }

        /// Logical OR of all elements using sequential reduce.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn any(a: &ArraySeqStPerS<B>) -> B {
            ArraySeqStPerS::reduce(a, &|x, y| *x || *y, false)
        }

        /// Logical AND of all elements using sequential reduce.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn all(a: &ArraySeqStPerS<B>) -> B {
            ArraySeqStPerS::reduce(a, &|x, y| *x && *y, true)
        }
    }
}
