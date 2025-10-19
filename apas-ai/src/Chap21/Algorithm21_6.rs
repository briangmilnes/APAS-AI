//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.6: Prime Sieve using ArraySeqPer and ninject.

pub mod Algorithm21_6 {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    pub trait Algorithm21_6Trait {
        /// Algorithm 21.6 (Prime Sieve) using ArraySeqPer - simplified version
        /// APAS: Work Θ(n lg n), Span Θ(lg n)
        fn prime_sieve(n: N) -> ArraySeqStPerS<N>;
    }

    /// Algorithm 21.6 (Prime Sieve) using ArraySeqPer - simplified version.
    /// Construct primes using a sieve: generate composites, then filter candidates.
    ///
    /// Uses the sieve of Eratosthenes approach with functional programming constructs.
    /// gpt-5-hard: Work: Θ(n lg n), Span: Θ(lg n)
    /// APAS: Work: Θ(n lg n), Span: Θ(lg n)
    pub fn prime_sieve(n: N) -> ArraySeqStPerS<N> {
        if n <= 2 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        // cs = 〈 i * j : 2 ≤ i ≤ floor(sqrt(n)) , 2 ≤ j ≤ n/i 〉
        let root: N = (n as f64).sqrt().floor() as N;
        let nested: ArraySeqStPerS<ArraySeqStPerS<N>> =
            <ArraySeqStPerS<ArraySeqStPerS<N>> as ArraySeqStPerRedefinableTrait<ArraySeqStPerS<N>>>::tabulate(
                &|i0| {
                    let i = i0 + 2; // i in [2..=root]
                    let limit = if i == 0 { 0 } else { n / i };
                    let len = if limit >= 2 { limit - 1 } else { 0 }; // j in [2..=limit] => length max(limit-1,0)
                    <ArraySeqStPerS<N> as ArraySeqStPerRedefinableTrait<N>>::tabulate(&|j0| i * (j0 + 2), len)
                },
                if root >= 2 { root - 1 } else { 0 },
            );
        let composites: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerBaseTrait<N>>::flatten(&nested);

        // Create candidates: 2, 3, ..., n
        let candidates: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerRedefinableTrait<N>>::tabulate(&|i| i + 2, n - 1);

        // Filter out composites to get primes
        let filtered: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerRedefinableTrait<N>>::filter(&candidates, &|x| {
            // Check if x is NOT in composites
            let mut is_composite = false;
            for i in 0..composites.length() {
                if *composites.nth(i) == *x {
                    is_composite = true;
                    break;
                }
            }
            !is_composite
        });
        filtered
    }
}
