//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.8: Brute Force Primality Test tests.
use apas_ai::Chap21::Exercise21_8::Exercise21_8::*;
use apas_ai::Types::Types::*;

#[test]
fn test_is_prime_small_values() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(!is_prime(9));
}

#[test]
fn test_is_divisible() {
    assert!(is_divisible(6, 2));
    assert!(is_divisible(6, 3));
    assert!(!is_divisible(7, 2));
    assert!(!is_divisible(7, 3));
}
