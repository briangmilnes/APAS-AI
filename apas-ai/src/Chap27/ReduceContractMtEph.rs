//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel reduce using contraction technique (Chapter 27, Algorithm 27.2).

pub mod ReduceContractMtEph {
    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
    use crate::ParaPair;
    use crate::Types::Types::*;
    use std::sync::Arc;
    use std::thread;

    /// Parallel reduce using contraction technique.
    pub trait ReduceContractMtEphTrait<T: StT + Send + Sync> {
        /// Reduce a sequence using parallel contraction: contract→solve→expand.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            id: T,
        ) -> T;
    }

    impl<T: StT + Send + Sync + Clone + 'static> ReduceContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            id: T,
        ) -> T {
            let n = a.length();

            // Base case: empty or single element
            if n == 0 {
                return id;
            }
            if n == 1 {
                return a.nth_cloned(0);
            }

            // Contract: pair up elements and apply f in parallel
            // b[i] = f(a[2i], a[2i+1])
            let half = n / 2;
            let a_arc = Arc::new(a.clone());
            let f_clone = Arc::clone(&f);

            let b = ArraySeqMtEphS::tabulate(
                &|i| {
                    let left = a_arc.nth_cloned(2 * i);
                    let right = a_arc.nth_cloned(2 * i + 1);
                    f_clone(&left, &right)
                },
                half,
            );

            // Handle odd-length sequences: last element unpaired
            let f_solve = Arc::clone(&f);
            let f_combine = Arc::clone(&f);

            if n % 2 == 1 {
                let last = a.nth_cloned(n - 1);
                let last_for_combine = last.clone();
                let id_clone = id.clone();

                // Solve and combine in parallel using ParaPair!
                let Pair(result, _) = ParaPair!(
                    move || Self::reduce_contract_parallel(&b, f_solve, id_clone),
                    move || last.clone()
                );

                f_combine(&result, &last_for_combine)
            } else {
                // Solve: recursively reduce b
                Self::reduce_contract_parallel(&b, f_solve, id)
            }
        }
    }
}
