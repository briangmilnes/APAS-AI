//! Algorithm 21.6 (Prime Sieve) using ArraySeqPer and ninject.

use apas_ai::ArraySeqStPer;
    use apas_ai::ArrayStPerSLit;
use apas_ai::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
use apas_ai::ArraySeqStPerChap19::ArraySeqStPerChap19::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStPerTrait, ArrayStPerS};

/// Construct primes using a sieve: generate composites cs, create sieve pairs (x,false), ninject, then filter indices.
/// gpt-5-hard: Work: Θ(n lg n), Span: Θ(lg n)
/// APAS: Work: Θ(n lg n), Span: Θ(lg n)
fn prime_sieve(n: N) -> ArrayStPerS<N> {
    if n <= 2 {
        return ArrayStPerSLit![];
    }
    // cs = 〈 i * j : 2 ≤ i ≤ floor(sqrt(n)) , 2 ≤ j ≤ n/i 〉
    let root: N = (n as f64).sqrt().floor() as N;
    let nested: ArrayStPerS<ArrayStPerS<N>> =
        <ArrayStPerS<ArrayStPerS<N>> as ArraySeqStPerChap19Trait<ArrayStPerS<N>>>::tabulate(
            |i0| {
                let i = i0 + 2; // i in [2..=root]
                let limit = if i == 0 { 0 } else { n / i };
                let len = if limit >= 2 { limit - 1 } else { 0 }; // j in [2..=limit] => length max(limit-1,0)
                <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|j0| i * (j0 + 2), len)
            },
            if root >= 2 { root - 1 } else { 0 },
        );
    let cs: ArrayStPerS<N> = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::flatten(&nested);

    // sieve = 〈 (x, false) : x ∈ cs 〉
    let sieve_pairs: ArrayStPerS<Pair<N, B>> =
        <ArrayStPerS<Pair<N, B>> as ArraySeqStPerChap19Trait<Pair<N, B>>>::tabulate(
            |i| Pair(*cs.nth(i), B::False),
            cs.length(),
        );

    // all = 〈 true : 0 ≤ i < n 〉
    let all: ArrayStPerS<B> = <ArrayStPerS<B> as ArraySeqStPerChap19Trait<B>>::tabulate(|_| B::True, n);

    // isPrime = ninject all sieve
    let is_prime: ArrayStPerS<B> = <ArrayStPerS<B> as ArraySeqStPerChap19Trait<B>>::ninject(&all, &sieve_pairs);

    // primes = 〈 i : 2 ≤ i < n | isPrime[i] = true 〉
    let candidates: ArrayStPerS<N> = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i, n);
    let filtered_idx: ArrayStPerS<N> = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::filter(&candidates, |i| {
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
    assert_eq!(p, ArrayStPerSLit![2, 3, 5, 7]);
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
