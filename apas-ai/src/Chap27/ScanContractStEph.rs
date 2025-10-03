//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential scan using contraction technique (Chapter 27, Algorithm 27.3).

pub mod ScanContractStEph {

    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    pub trait ScanContractStEphTrait<T: StT> {
        /// Scan a sequence using contraction: contract→solve→expand.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn scan_contract<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> ArraySeqStEphS<T>;
    }

    impl<T: StT + Clone> ScanContractStEphTrait<T> for ArraySeqStEphS<T> {
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

            // Contract: pair up elements and apply f
            // b[i] = f(a[2i], a[2i+1])
            let half = n / 2;
            let b = ArraySeqStEphS::tabulate(&|i| f(a.nth(2 * i), a.nth(2 * i + 1)), half);

            // Solve: recursively scan b to get c
            let c = Self::scan_contract(&b, f, id.clone());

            // Expand: reconstruct result
            // For even indices: result[2i] = c[i]
            // For odd indices: result[2i+1] = f(c[i], a[2i])
            let main_result = ArraySeqStEphS::tabulate(
                &|i| {
                    if i % 2 == 0 {
                        // Even index: use scan result from contracted sequence
                        c.nth(i / 2).clone()
                    } else {
                        // Odd index: combine scan result with original element
                        f(c.nth(i / 2), a.nth(i - 1))
                    }
                },
                if n % 2 == 0 { n } else { n - 1 },
            );

            // Handle last element if odd length
            if n % 2 == 1 {
                // Last element: f(result[n-2], a[n-2])
                let last_elem_of_main = main_result.nth(n - 2).clone();
                let last_part = ArraySeqStEphS::singleton(f(&last_elem_of_main, a.nth(n - 2)));
                ArraySeqStEphS::append(&main_result, &last_part)
            } else {
                main_result
            }
        }
    }
}
