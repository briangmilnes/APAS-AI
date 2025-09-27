//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.8: Brute Force Primality Test tests.

pub mod Test34Exercise_21_8 {
    use apas_ai::Chap21::Exercise21_8::Exercise21_8::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_is_prime_small_values() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(9), false);
    }

    #[test]
    fn test_is_divisible_helper() {
        assert_eq!(is_divisible(6, 2), true);
        assert_eq!(is_divisible(6, 3), true);
        assert_eq!(is_divisible(7, 2), false);
        assert_eq!(is_divisible(7, 3), false);
    }
}
