//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Algorithm 21.6 (Prime Sieve) using ArraySeqPer and ninject.

use apas_ai::ArraySeqStPer;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStPerS, ArraySeqStPerTrait};

/// Construct primes using a sieve: generate composites cs, create sieve pairs (x,false), ninject, then filter indices.
/// gpt-5-hard: Work: Θ(n lg n), Span: Θ(lg n)
/// APAS: Work: Θ(n lg n), Span: Θ(lg n)
fn prime_sieve(n: N) -> ArraySeqStPerS<N> {
    if n <= 2 {
        return ArraySeqStPerSLit![];
    }
    // cs = 〈 i * j : 2 ≤ i ≤ floor(sqrt(n)) , 2 ≤ j ≤ n/i 〉
    let root: N = (n as f64).sqrt().floor() as N;
    let nested: ArraySeqStPerS<ArraySeqStPerS<N>> =
        <ArraySeqStPerS<ArraySeqStPerS<N>> as ArraySeqStPerTrait<ArraySeqStPerS<N>>>::tabulate(
            |i0| {
                let i = i0 + 2; // i in [2..=root]
                let limit = if i == 0 { 0 } else { n / i };
                let len = if limit >= 2 { limit - 1 } else { 0 }; // j in [2..=limit] => length max(limit-1,0)
                <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|j0| i * (j0 + 2), len)
            },
            if root >= 2 { root - 1 } else { 0 },
        );
    let cs: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&nested);

    // sieve = 〈 (x, false) : x ∈ cs 〉
    let sieve_pairs: ArraySeqStPerS<Pair<N, B>> =
        <ArraySeqStPerS<Pair<N, B>> as ArraySeqStPerTrait<Pair<N, B>>>::tabulate(
            |i| Pair(*cs.nth(i), B::False),
            cs.length(),
        );

    // all = 〈 true : 0 ≤ i < n 〉
    let all: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::tabulate(|_| B::True, n);

    // isPrime = ninject all sieve
    let is_prime: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::ninject(&all, &sieve_pairs);

    // primes = 〈 i : 2 ≤ i < n | isPrime[i] = true 〉
    let candidates: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i, n);
    let filtered_idx: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&candidates, |i| {
        if *i >= 2 && *i < n {
            if *is_prime.nth(*i) == B::True {
                B::True
            } else {
                B::False
            }
        } else {
            B::False
        }
    });
    filtered_idx
}

#[test]
fn test_prime_sieve_small() {
    let p = prime_sieve(10);
    assert_eq!(p, ArraySeqStPerSLit![2, 3, 5, 7]);
}

#[test]
fn test_prime_sieve_n2_empty() {
    let p = prime_sieve(2);
    assert_eq!(p.length(), 0);
}

#[test]
fn test_prime_sieve_debug_shape() {
    let p = prime_sieve(20);
    let dbg_str = format!("{:?}", p);
    assert!(!dbg_str.is_empty());
}
