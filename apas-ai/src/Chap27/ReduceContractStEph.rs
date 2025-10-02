//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential reduce using contraction technique (Chapter 27, Algorithm 27.2).

pub mod ReduceContractStEph {

    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Types::Types::*;

    pub trait ReduceContractStEphTrait<T: StT> {
        /// Reduce a sequence using contraction: contract→solve→expand.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn reduce_contract<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;
    }

    impl<T: StT + Clone> ReduceContractStEphTrait<T> for ArraySeqStEphS<T> {
        fn reduce_contract<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {
            let n = a.length();

            // Base case: empty or single element
            if n == 0 {
                return id;
            }
            if n == 1 {
                return a.nth(0).clone();
            }

            // Contract: pair up elements and apply f
            // b[i] = f(a[2i], a[2i+1])
            let half = n / 2;
            let b = ArraySeqStEphS::tabulate(
                &|i| {
                    let left = a.nth(2 * i);
                    let right = a.nth(2 * i + 1);
                    f(left, right)
                },
                half,
            );

            // Handle odd-length sequences: last element unpaired
            let result = Self::reduce_contract(&b, f, id);

            // If odd length, combine result with last element
            if n % 2 == 1 { f(&result, a.nth(n - 1)) } else { result }
        }
    }
}
