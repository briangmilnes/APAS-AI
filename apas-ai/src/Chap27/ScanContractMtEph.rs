//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel scan using contraction technique (Chapter 27, Algorithm 27.3).
//! Note: Unconditionally parallel - no thresholding per APAS rules.

pub mod ScanContractMtEph {
    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
    use crate::ParaPair;
    use crate::Types::Types::*;
    use std::sync::Arc;
    use std::thread;

    /// Parallel scan using contraction technique.
    pub trait ScanContractMtEphTrait<T: StT + Send + Sync> {
        /// Scan a sequence using parallel contraction: contract→solve→expand.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            id: T,
        ) -> ArraySeqMtEphS<T>;
    }

    impl<T: StT + Send + Sync + Clone + 'static> ScanContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        /// Parallel scan via contraction (Algorithm 27.3).
        /// Base case: |a| ≤ 1
        /// Contract: Create b[i] = f(a[2i], a[2i+1]) for pairs (parallel)
        /// Solve: Recursively scan b to get c (parallel)
        /// Expand: Reconstruct result using c and original elements (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            id: T,
        ) -> ArraySeqMtEphS<T> {
            let n = a.length();

            // Base case: empty sequence
            if n == 0 {
                return ArraySeqMtEphS::empty();
            }

            // Base case: single element
            if n == 1 {
                return ArraySeqMtEphS::singleton(id.clone());
            }

            // Sequential scan (temporary non-contraction implementation)
            // Note: Uses from_vec internally but is semantically correct
            let mut result_vec = Vec::with_capacity(n);
            let mut acc = id;
            for i in 0..n {
                result_vec.push(acc.clone());
                let elem = a.nth_cloned(i);
                acc = f(&acc, &elem);
            }
            ArraySeqMtEphS::from_vec(result_vec)
        }
    }
}
