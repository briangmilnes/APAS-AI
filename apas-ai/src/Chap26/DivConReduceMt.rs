//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer via reduce pattern - parallel implementation (Chapter 26, Section 5).
//! Note: Unconditionally parallel - no thresholding per APAS rules.

pub mod DivConReduceMt {
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerS, ArraySeqMtPerTrait};
    use crate::Types::Types::*;

    /// Divide-and-conquer using reduce pattern (parallel).
    pub trait DivConReduceMtTrait {
        /// Find maximum element via parallel reduce.
        /// Pattern: reduce max MIN identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_element_parallel(a: &ArraySeqMtPerS<N>) -> Option<N>;

        /// Sum all elements via parallel reduce.
        /// Pattern: reduce (+) 0 identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn sum_parallel(a: &ArraySeqMtPerS<N>) -> N;

        /// Product of all elements via parallel reduce.
        /// Pattern: reduce (*) 1 identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn product_parallel(a: &ArraySeqMtPerS<N>) -> N;

        /// Logical OR of all elements via parallel reduce.
        /// Pattern: reduce (||) false identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn any_parallel(a: &ArraySeqMtPerS<B>) -> B;

        /// Logical AND of all elements via parallel reduce.
        /// Pattern: reduce (&&) true identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn all_parallel(a: &ArraySeqMtPerS<B>) -> B;
    }

    impl DivConReduceMtTrait for ArraySeqMtPerS<N> {
        /// Find maximum element using parallel reduce.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_element_parallel(a: &ArraySeqMtPerS<N>) -> Option<N> {
            if a.length() == 0 {
                return None;
            }
            // Use parallel reduce from ArraySeqMtPer (already uses ParaPair!)
            Some(ArraySeqMtPerS::reduce(a, &|x: &N, y: &N| (*x).max(*y), *a.nth(0)))
        }

        /// Sum all elements using parallel reduce.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn sum_parallel(a: &ArraySeqMtPerS<N>) -> N {
            // Use parallel reduce from ArraySeqMtPer (already uses ParaPair!)
            ArraySeqMtPerS::reduce(a, &|x: &N, y: &N| *x + *y, 0)
        }

        /// Product of all elements using parallel reduce.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn product_parallel(a: &ArraySeqMtPerS<N>) -> N {
            // Use parallel reduce from ArraySeqMtPer (already uses ParaPair!)
            ArraySeqMtPerS::reduce(a, &|x: &N, y: &N| *x * *y, 1)
        }

        /// Logical OR of all elements using parallel reduce.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn any_parallel(a: &ArraySeqMtPerS<B>) -> B {
            // Use parallel reduce from ArraySeqMtPer (already uses ParaPair!)
            ArraySeqMtPerS::reduce(a, &|x: &B, y: &B| *x || *y, false)
        }

        /// Logical AND of all elements using parallel reduce.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn all_parallel(a: &ArraySeqMtPerS<B>) -> B {
            // Use parallel reduce from ArraySeqMtPer (already uses ParaPair!)
            ArraySeqMtPerS::reduce(a, &|x: &B, y: &B| *x && *y, true)
        }
    }
}
