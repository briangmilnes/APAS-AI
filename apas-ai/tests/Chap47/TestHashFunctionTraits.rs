//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Function Traits Tests

use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Types::Types::*;

// Note: No HashFunTraitsLit macro exists, skipping macro test

#[test]
fn test_string_position_hash_function() {
    let hash_fn = StringPositionHashFunction;
    let table_size = 5;

    // Test Example 47.1 cases
    assert_eq!(hash_fn.hash(&"aa".to_string(), table_size), 0); // (0+0) % 5 = 0
    assert_eq!(hash_fn.hash(&"bb".to_string(), table_size), 2); // (1+1) % 5 = 2
    assert_eq!(hash_fn.hash(&"cc".to_string(), table_size), 4); // (2+2) % 5 = 4

    // Test description
    assert!(hash_fn.description().contains("Example 47.1"));
}

#[test]
fn test_polynomial_hash_function() {
    let hash_fn = PolynomialHashFunction::new(31);
    let table_size = 100;

    let test_string = "test".to_string();
    let hash_value = hash_fn.hash(&test_string, table_size);

    assert!(hash_value < table_size);
    assert!(hash_fn.description().contains("base=31"));
}

#[test]
fn test_universal_integer_hash_function() {
    let hash_fn = UniversalIntegerHashFunction::new(123, 456);
    let table_size = 1000;

    let test_key = 42;
    let hash_value = hash_fn.hash(&test_key, table_size);

    assert!(hash_value < table_size);
    assert!(hash_fn.description().contains("Universal"));
}

#[test]
fn test_default_key_equality() {
    let eq_fn = DefaultKeyEquality;

    assert!(eq_fn.eq(&"hello", &"hello"));
    assert!(!eq_fn.eq(&"hello", &"world"));
    assert!(eq_fn.eq(&42, &42));
    assert!(!eq_fn.eq(&42, &24));
}

#[test]
fn test_case_insensitive_string_equality() {
    let eq_fn = CaseInsensitiveStringEquality;

    assert!(eq_fn.eq(&"Hello".to_string(), &"hello".to_string()));
    assert!(eq_fn.eq(&"WORLD".to_string(), &"world".to_string()));
    assert!(!eq_fn.eq(&"hello".to_string(), &"world".to_string()));
}

#[test]
fn test_universal_hash_family() {
    let family = UniversalIntegerHashFamily::new();
    let hash_fn1 = family.generate(12345);
    let hash_fn2 = family.generate(67890);

    let test_key = 100;
    let table_size = 50;

    let hash1 = hash_fn1.hash(&test_key, table_size);
    let hash2 = hash_fn2.hash(&test_key, table_size);

    assert!(hash1 < table_size);
    assert!(hash2 < table_size);
    // Different seeds should likely produce different hash functions
    assert!(family.family_description().contains("Universal"));
}

#[test]
fn test_probe_sequence_generator() {
    let hash1 = StringPositionHashFunction;
    let hash2 = PolynomialHashFunction::new(31);
    let probe_gen = ProbeSequenceGenerator::new(hash1, hash2);

    let key = "test".to_string();
    let table_size = 10;

    let probe0 = probe_gen.probe_hash(&key, 0, table_size);
    let probe1 = probe_gen.probe_hash(&key, 1, table_size);
    let probe2 = probe_gen.probe_hash(&key, 2, table_size);

    assert!(probe0 < table_size);
    assert!(probe1 < table_size);
    assert!(probe2 < table_size);

    // Different probe indices should generally give different results
    assert!(probe0 != probe1 || probe1 != probe2);
}

#[test]
fn test_load_factor_manager() {
    let manager = LoadFactorManager::new(0.75, 0.25);

    // Test load factor calculation
    assert_eq!(manager.load_factor(0, 10), 0.0);
    assert_eq!(manager.load_factor(5, 10), 0.5);
    assert_eq!(manager.load_factor(8, 10), 0.8);

    // Test growth decisions
    assert!(!manager.should_grow(5, 10)); // 0.5 < 0.75
    assert!(manager.should_grow(8, 10)); // 0.8 > 0.75

    // Test shrink decisions
    assert!(!manager.should_shrink(5, 10)); // 0.5 > 0.25
    assert!(manager.should_shrink(2, 10)); // 0.2 < 0.25
    assert!(!manager.should_shrink(2, 8)); // Don't shrink below minimum

    // Test size calculations
    assert_eq!(manager.grow_size(10), 20);
    assert_eq!(manager.shrink_size(20), 10);
    assert_eq!(manager.shrink_size(16), 8); // Minimum size
}

#[test]
fn test_hash_table_stats() {
    let stats = HashTableStats::new(100, 200).with_collision_stats(25, 5, 2.5);

    assert_eq!(stats.num_elements, 100);
    assert_eq!(stats.table_size, 200);
    assert_eq!(stats.load_factor, 0.5);
    assert_eq!(stats.num_collisions, 25);
    assert_eq!(stats.max_chain_length, 5);
    assert_eq!(stats.avg_chain_length, 2.5);

    let display_str = format!("{}", stats);
    assert!(display_str.contains("elements: 100"));
    assert!(display_str.contains("size: 200"));
    assert!(display_str.contains("load_factor: 0.500"));
}

#[test]
fn test_hash_table_utils() {
    // Test prime checking
    assert!(!HashTableUtils::is_prime(1));
    assert!(HashTableUtils::is_prime(2));
    assert!(HashTableUtils::is_prime(3));
    assert!(!HashTableUtils::is_prime(4));
    assert!(HashTableUtils::is_prime(5));
    assert!(!HashTableUtils::is_prime(9));
    assert!(HashTableUtils::is_prime(11));

    // Test next prime
    assert_eq!(HashTableUtils::next_prime(10), 11);
    assert_eq!(HashTableUtils::next_prime(11), 11);
    assert_eq!(HashTableUtils::next_prime(12), 13);

    // Test good table sizes
    assert_eq!(HashTableUtils::good_table_size(10, false), 16); // Next power of 2
    assert_eq!(HashTableUtils::good_table_size(10, true), 11); // Next prime
}

#[test]
fn test_hash_function_tester() {
    let hash_fn = StringPositionHashFunction;
    let keys = vec![
        "aa".to_string(),
        "bb".to_string(),
        "cc".to_string(),
        "dd".to_string(),
        "ee".to_string(),
    ];
    let table_size = 5;

    let stats = HashFunctionTester::test_distribution(&hash_fn, &keys, table_size);

    assert_eq!(stats.num_elements, 5);
    assert_eq!(stats.table_size, 5);
    assert_eq!(stats.load_factor, 1.0);

    // Test performance measurement
    let duration = HashFunctionTester::benchmark_hash_function(&hash_fn, &keys, table_size);
    assert!(duration.as_nanos() > 0); // Should take some time
}

#[test]
fn test_simple_integer_hash_macro() {
    let hash_fn = SimpleIntegerHash;
    let table_size = 100;

    let hash_value = hash_fn.hash(&42, table_size);
    assert!(hash_value < table_size);
    assert!(hash_fn.description().contains("Knuth"));
}

// Dedicated tests for impl_hash_function macro

#[test]
fn test_impl_hash_function_macro_basic_usage() {
    // Test the macro-generated SimpleIntegerHash
    let hash_fn = SimpleIntegerHash;
    let table_size = 100;

    // Test basic hashing
    let hash1 = hash_fn.hash(&42, table_size);
    let hash2 = hash_fn.hash(&42, table_size);
    assert_eq!(hash1, hash2); // Same input should give same hash
    assert!(hash1 < table_size);

    // Test different inputs give different hashes (likely)
    let hash3 = hash_fn.hash(&99, table_size);
    assert!(hash3 < table_size);

    // Test description
    let desc = hash_fn.description();
    assert!(desc.contains("Knuth"));
    assert!(desc.contains("multiplicative"));
}

#[test]
fn test_impl_hash_function_macro_custom_hash() {
    // Define a custom hash function using the macro
    apas_ai::impl_hash_function!(
        CustomTestHash,
        i32,
        |key: &i32| (*key as N * 7 + 13),
        "CustomTestHash (linear function)"
    );

    let hash_fn = CustomTestHash;
    let table_size = 50;

    // Test basic properties
    let hash1 = hash_fn.hash(&10, table_size);
    let hash2 = hash_fn.hash(&10, table_size);
    assert_eq!(hash1, hash2); // Deterministic
    assert!(hash1 < table_size); // Within bounds

    // Manually verify the hash computation: (10 * 7 + 13) % 50 = 83 % 50 = 33
    assert_eq!(hash1, 33);

    // Test description
    assert_eq!(hash_fn.description(), "CustomTestHash (linear function)");
}

#[test]
fn test_impl_hash_function_macro_edge_cases() {
    let hash_fn = SimpleIntegerHash;

    // Test with small table sizes
    assert_eq!(hash_fn.hash(&42, 1), 0); // Any hash mod 1 = 0

    let hash2 = hash_fn.hash(&42, 2);
    assert!(hash2 < 2); // Either 0 or 1

    // Test with zero
    let hash_zero = hash_fn.hash(&0, 100);
    assert!(hash_zero < 100);

    // Test with negative numbers
    let hash_neg = hash_fn.hash(&-42, 100);
    assert!(hash_neg < 100);
}

#[test]
fn test_impl_hash_function_macro_distribution() {
    let hash_fn = SimpleIntegerHash;
    let table_size = 10;

    // Test that different keys hash to different buckets (mostly)
    let mut seen_hashes = std::collections::HashSet::new();
    for i in 0..50 {
        let hash_value = hash_fn.hash(&i, table_size);
        assert!(hash_value < table_size);
        seen_hashes.insert(hash_value);
    }

    // With 50 keys and 10 buckets, we should see most buckets used
    // (not a strict requirement due to randomness, but likely)
    assert!(seen_hashes.len() >= 5); // At least half the buckets used
}
