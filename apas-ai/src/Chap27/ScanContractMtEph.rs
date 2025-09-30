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

            // Contract: pair up elements and apply f in parallel
            // b[i] = f(a[2i], a[2i+1])
            let half = n / 2;
            let a_arc = Arc::new(a.clone());
            let f_contract = Arc::clone(&f);

            let b = ArraySeqMtEphS::tabulate(
                &|i| {
                    let left = a_arc.nth_cloned(2 * i);
                    let right = a_arc.nth_cloned(2 * i + 1);
                    f_contract(&left, &right)
                },
                half,
            );

            // Solve: recursively scan b to get c (parallel)
            let f_solve = Arc::clone(&f);
            let c = Self::scan_contract_parallel(&b, f_solve, id);

            // Expand: reconstruct result using parallel tabulation
            // For even indices: result[2i] = c[i]
            // For odd indices: result[2i+1] = f(c[i], a[2i])
            let c_arc = Arc::new(c);
            let a_arc2 = Arc::new(a.clone());
            let f_expand = Arc::clone(&f);

            let main_result = ArraySeqMtEphS::tabulate(
                &|i| {
                    if i % 2 == 0 {
                        // Even index: use scan result from contracted sequence
                        c_arc.nth_cloned(i / 2)
                    } else {
                        // Odd index: combine scan result with original element
                        let scan_val = c_arc.nth_cloned(i / 2);
                        let orig_val = a_arc2.nth_cloned(i - 1);
                        f_expand(&scan_val, &orig_val)
                    }
                },
                if n % 2 == 0 { n } else { n - 1 },
            );

            // Handle last element if odd length
            if n % 2 == 1 {
                // Last element: f(result[n-2], a[n-2])
                let last_elem_of_main = main_result.nth_cloned(n - 2);
                let last_val = f(&last_elem_of_main, &a.nth_cloned(n - 2));
                let last_part = ArraySeqMtEphS::singleton(last_val);
                ArraySeqMtEphS::append(&main_result, &last_part)
            } else {
                main_result
            }
        }
    }
}
