//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.8: Brute Force Primality Test tests.

pub mod Test34Exercise_21_8 {
    use apas_ai::Chap21::Exercise21_8::Exercise21_8::*;
    use apas_ai::Types::Types::*;

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
fn test_is_divisible_helper() {
    assert_eq!(is_divisible(6, 2), B::True);
    assert_eq!(is_divisible(6, 3), B::True);
    assert_eq!(is_divisible(7, 2), B::False);
    assert_eq!(is_divisible(7, 3), B::False);
}

}
