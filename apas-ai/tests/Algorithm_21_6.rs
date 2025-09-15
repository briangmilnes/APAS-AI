//! Algorithm 21.6 (Prime Sieve) using ArraySeqPer and ninject.

use apas_ai::Types::Types::*;
use apas_ai::{ArrayPerS, ArraySeqPerTrait};
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use apas_ai::ArraySeqPer;

/// Construct primes using a sieve: generate composites cs, create sieve pairs (x,false), ninject, then filter indices.
/// gpt-5-hard: Work: Θ(n lg n), Span: Θ(lg n)
/// APAS: Work: Θ(n lg n), Span: Θ(lg n)
fn prime_sieve(n: N) -> ArrayPerS<N> {
    if n <= 2 { return ArraySeqPer![]; }
    // cs = 〈 i * j : 2 ≤ i ≤ floor(sqrt(n)) , 2 ≤ j ≤ n/i 〉
    let root: N = (n as f64).sqrt().floor() as N;
    let nested: ArrayPerS<ArrayPerS<N>> =
        <ArrayPerS<ArrayPerS<N>> as ArraySeqPerChap19Trait<T>>::tabulate(
            |i0| {
                let i = i0 + 2; // i in [2..=root]
                let limit = if i == 0 { 0 } else { n / i };
                let len = if limit >= 2 { limit - 1 } else { 0 }; // j in [2..=limit] => length max(limit-1,0)
                <ArrayPerS<N> as ArraySeqPerChap19Trait<T>>::tabulate(|j0| i * (j0 + 2), len)
            },
            if root >= 2 { root - 1 } else { 0 },
        );
    let cs: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap18Trait<T>>::flatten(&nested);

    // sieve = 〈 (x, false) : x ∈ cs 〉
    let sieve_pairs: ArrayPerS<(N, B)> = <ArrayPerS<(N, B)> as ArraySeqPerChap19Trait<T>>::map(&cs, |x| (*x, B::False));

    // all = 〈 true : 0 ≤ i < n 〉
    let all: ArrayPerS<B> = <ArrayPerS<B> as ArraySeqPerChap19Trait<T>>::tabulate(|_| B::True, n);

    // isPrime = ninject all sieve
    let is_prime: ArrayPerS<B> = <ArrayPerS<B> as ArraySeqPerChap19Trait<T>>::ninject_parallel2(&all, &sieve_pairs);

    // primes = 〈 i : 2 ≤ i < n | isPrime[i] = true 〉
    let candidates: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap19Trait<T>>::tabulate(|i| i, n);
    let filtered_idx: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap18Trait<T>>::filter(&candidates, |i| {
        if *i >= 2 && *i < n { if *is_prime.nth(*i) == B::True { B::True } else { B::False } } else { B::False }
    });
    filtered_idx
}

#[test]
fn test_prime_sieve_small() {
    let p = prime_sieve(10);
    let v: Vec<N> = p.iter().copied().collect();
    assert_eq!(v, vec![2, 3, 5, 7]);
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
