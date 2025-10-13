//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 21: Algorithm21_6 - Prime Sieve using ArraySeqPer

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap21::Algorithm21_6::Algorithm21_6::*;
use apas_ai::Types::Types::*;

// Helper function to check if number is prime
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    for i in (3..=((n as f64).sqrt() as usize)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Helper function for expected primes (simple sieve)
fn expected_primes_up_to(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    (2..=n).filter(|&i| is_prime[i]).collect()
}

// Helper function to convert sequence to vector for testing
fn seq_to_vec<T: Clone>(seq: &ArraySeqStPerS<T>) -> Vec<T>
where
    T: std::fmt::Debug,
    T: Eq,
    T: StT,
    T: std::fmt::Display,
{
    let mut vec = Vec::new();
    for i in 0..seq.length() {
        vec.push(seq.nth(i).clone());
    }
    vec
}

#[test]
fn test_prime_sieve_small_n() {
    // n <= 2 should return empty sequence
    let result = prime_sieve(0);
    assert_eq!(result.length(), 0);

    let result = prime_sieve(1);
    assert_eq!(result.length(), 0);

    let result = prime_sieve(2);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_prime_sieve_n_3() {
    // n = 3: candidates = [2], no composites for range, should return [2]
    let result = prime_sieve(3);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(3);

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }
}

#[test]
fn test_prime_sieve_n_10() {
    // n = 10: candidates = [2,3,4,5,6,7,8,9], composites include 4,6,8,9
    // Expected primes: [2,3,5,7]
    let result = prime_sieve(10);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(10);

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }
}

#[test]
fn test_prime_sieve_n_20() {
    // n = 20: Expected primes: [2,3,5,7,11,13,17,19]
    let result = prime_sieve(20);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(20);

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }
}

#[test]
fn test_prime_sieve_n_30() {
    // n = 30: Expected primes: [2,3,5,7,11,13,17,19,23,29]
    let result = prime_sieve(30);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(30);

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }
}

#[test]
fn test_prime_sieve_n_50() {
    // n = 50: Test with larger input
    let result = prime_sieve(50);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(50);

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }
}

#[test]
fn test_prime_sieve_n_100() {
    // n = 100: Test with even larger input
    let result = prime_sieve(100);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(100);

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }

    // Check that the first few primes are correct
    assert!(primes.contains(&2));
    assert!(primes.contains(&3));
    assert!(primes.contains(&5));
    assert!(primes.contains(&7));
    assert!(primes.contains(&11));
    assert!(primes.contains(&97)); // Largest prime < 100

    // Check that some composites are NOT included
    assert!(!primes.contains(&4));
    assert!(!primes.contains(&6));
    assert!(!primes.contains(&8));
    assert!(!primes.contains(&9));
    assert!(!primes.contains(&10));
    assert!(!primes.contains(&99));
}

#[test]
fn test_prime_sieve_edge_cases() {
    // Test edge cases around small primes
    let result = prime_sieve(4);
    let primes = seq_to_vec(&result);
    let expected = vec![2, 3];

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime));
    }

    let result = prime_sieve(5);
    let primes = seq_to_vec(&result);
    let expected = vec![2, 3, 5]; // 5 is prime!

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime));
    }

    let result = prime_sieve(6);
    let primes = seq_to_vec(&result);
    let expected = vec![2, 3, 5];

    assert_eq!(primes.len(), expected.len());
    for prime in expected {
        assert!(primes.contains(&prime));
    }
}

#[test]
fn test_prime_sieve_specific_ranges() {
    // Test specific interesting ranges

    // Range that includes first 10 primes
    let result = prime_sieve(30);
    let primes = seq_to_vec(&result);

    // First 10 primes are: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
    let first_10_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for prime in first_10_primes {
        assert!(primes.contains(&prime), "Missing prime: {}", prime);
    }
}

#[test]
fn test_prime_sieve_no_duplicates() {
    // Ensure no duplicate primes in result
    let result = prime_sieve(50);
    let primes = seq_to_vec(&result);

    let mut sorted_primes = primes.clone();
    sorted_primes.sort();
    sorted_primes.dedup();

    assert_eq!(primes.len(), sorted_primes.len(), "Found duplicate primes");
}

#[test]
fn test_prime_sieve_ordering() {
    // The sieve should return primes in some consistent order
    let result = prime_sieve(30);
    let primes = seq_to_vec(&result);

    // Check that all returned numbers are actually prime
    for &prime in &primes {
        assert!(is_prime(prime), "{} is not prime", prime);
    }

    // Check that all primes in range are included
    for candidate in 2..30 {
        if is_prime(candidate) {
            assert!(primes.contains(&candidate), "Missing prime: {}", candidate);
        }
    }
}

#[test]
fn test_prime_sieve_larger_input() {
    // Test with a reasonably large input to verify algorithm correctness
    let result = prime_sieve(200);
    let primes = seq_to_vec(&result);
    let expected = expected_primes_up_to(200);

    assert_eq!(primes.len(), expected.len());

    // Check a sampling of known primes
    let known_primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107,
        109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199,
    ];

    for &prime in &known_primes {
        assert!(primes.contains(&prime), "Missing known prime: {}", prime);
    }

    // Check that some known composites are not included
    let known_composites = vec![
        4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32, 33, 34, 35, 36, 38, 39, 40, 42, 44,
        45, 46, 48, 49, 50,
    ];

    for &composite in &known_composites {
        if composite < 200 {
            assert!(!primes.contains(&composite), "Found composite number: {}", composite);
        }
    }
}
