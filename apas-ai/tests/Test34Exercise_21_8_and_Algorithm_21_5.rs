//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Exercise 21.8 (isPrime) and Algorithm 21.5 (primesBF) tests and analysis.

use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Types::Types::*;

fn is_divisible(n: N, i: N) -> B { if n % i == 0 { B::True } else { B::False } }

/// Algorithm 21.4 (Brute Force Primality Test)
/// isPrime n = |{ x in 1..=floor(sqrt(n)) : n mod i == 0 }| == 1
/// gpt-5-hard: Work: Θ(√n), Span: Θ(lg n)
fn is_prime(n: N) -> B {
    if n < 2 {
        return B::False;
    }
    let k: N = (n as f64).sqrt().floor() as N;
    let all: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::tabulate(|i| is_divisible(n, i + 1), k);
    let zeros: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::filter(&all, |x| *x);
    if zeros.length() == 1 { B::True } else { B::False }
}

/// Algorithm 21.5 (Brute Force Solution to the Primes Problem)
/// primesBF n = { i in 2..n : isPrime(i) }
/// APAS: Work: Θ(n^{3/2}), Span: Θ(lg n)
/// gpt-5-hard: Work: Θ(n^{3/2}), Span: Θ(lg n)
fn primes_bf(n: N) -> ArraySeqStPerS<N> {
    if n <= 2 {
        return ArraySeqStPerS::from_vec(Vec::new());
    }
    let all: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i + 2, n - 2);
    let filtered: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&all, |x| is_prime(*x));
    filtered
}

#[test]
fn test_is_prime_small_values() {
    assert_eq!(is_prime(0), B::False);
    assert_eq!(is_prime(1), B::False);
    assert_eq!(is_prime(2), B::True);
    assert_eq!(is_prime(3), B::True);
    assert_eq!(is_prime(4), B::False);
    assert_eq!(is_prime(9), B::False);
}

#[test]
fn test_primes_bf_small() {
    let p = primes_bf(10);
    let v: Vec<N> = p.iter().copied().collect();
    assert_eq!(v, vec![2, 3, 5, 7]);
}

#[test]
fn test_primes_bf_debug_shape() {
    let p = primes_bf(5);
    let dbg_str = format!("{:?}", p);
    assert!(!dbg_str.is_empty());
}
