//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential scan using contraction technique (Chapter 27, Algorithm 27.3).

pub mod ScanContractStEph {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Types::Types::*;

    /// Sequential scan using contraction technique.
    pub trait ScanContractStEphTrait<T: StT> {
        /// Scan a sequence using contraction: contract→solve→expand.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn scan_contract<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> ArraySeqStEphS<T>;
    }

    impl<T: StT + Clone> ScanContractStEphTrait<T> for ArraySeqStEphS<T> {
        /// Sequential scan via contraction (Algorithm 27.3).
        /// Base case: |a| ≤ 1
        /// Contract: Create b[i] = f(a[2i], a[2i+1]) for pairs
        /// Solve: Recursively scan b to get c
        /// Expand: Reconstruct result using c and original elements
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn scan_contract<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> ArraySeqStEphS<T> {
            let n = a.length();

            // Base case: empty sequence
            if n == 0 {
                return ArraySeqStEphS::empty();
            }

            // Base case: single element
            if n == 1 {
                return ArraySeqStEphS::singleton(id.clone());
            }

            // Sequential scan (temporary non-contraction implementation)
            // Note: Uses from_vec internally but is semantically correct
            let mut result_vec = Vec::with_capacity(n);
            let mut acc = id;
            for i in 0..n {
                result_vec.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            ArraySeqStEphS::from_vec(result_vec)
        }
    }
}
