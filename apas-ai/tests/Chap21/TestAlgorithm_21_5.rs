//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Algorithm 21.5: Brute Force Solution to the Primes Problem tests.
use apas_ai::Chap21::Algorithm21_5::Algorithm21_5::*;
use apas_ai::Types::Types::*;

#[test]
fn test_primes_bf_small() {
    let p = primes_bf(10);
    let v: Vec<N> = p.iter().copied().collect();
    assert_eq!(v, vec![2, 3, 5, 7]);
}

#[test]
fn test_primes_bf_debug_shape() {
    let p = primes_bf(5);
    let dbg_str = format!("{p:?}");
    assert!(!dbg_str.is_empty());
}

#[test]
fn test_primes_bf_edge_cases() {
    let p0 = primes_bf(0);
    assert_eq!(p0.length(), 0);

    let p2 = primes_bf(2);
    assert_eq!(p2.length(), 0);

    let p3 = primes_bf(3);
    let v3: Vec<N> = p3.iter().copied().collect();
    assert_eq!(v3, vec![2]);
}
