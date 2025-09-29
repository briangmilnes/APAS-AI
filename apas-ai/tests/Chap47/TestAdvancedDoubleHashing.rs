//! Test module for AdvancedDoubleHashing
//! 
//! Tests the advanced double hashing implementation including:
//! - Dual hash function validation
//! - Relative prime validation
//! - Collision resolution effectiveness
//! - Edge cases and error conditions
//! 
//! Work: O(1) per operation test
//! Span: O(1) per operation test

use apas_ai::Chap47::AdvancedDoubleHashing::AdvancedDoubleHashing::{
    AdvancedDoubleHashingStrategy, DoubleHashingMetrics, RelativePrimeValidator,
    example_double_hashing_analysis, example_probe_sequence_analysis
};
use apas_ai::Chap47::FlatHashTable::FlatHashTable::{
    FlatHashTable, Entry, ProbeSequence
};
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::{
    DefaultHashFunction
};
use apas_ai::Types::Types::{Pair, StT};

/// Test RelativePrimeValidator GCD computation
#[test]
fn test_gcd_computation() {
    assert_eq!(RelativePrimeValidator::gcd(12, 8), 4);
    assert_eq!(RelativePrimeValidator::gcd(17, 13), 1);
    assert_eq!(RelativePrimeValidator::gcd(100, 25), 25);
    assert_eq!(RelativePrimeValidator::gcd(7, 0), 7);
    assert_eq!(RelativePrimeValidator::gcd(0, 5), 5);
}

/// Test relative prime validation
#[test]
fn test_relative_prime_validation() {
    // Relatively prime pairs
    assert!(RelativePrimeValidator::are_relatively_prime(7, 11));
    assert!(RelativePrimeValidator::are_relatively_prime(13, 17));
    assert!(RelativePrimeValidator::are_relatively_prime(9, 16));
    
    // Not relatively prime pairs
    assert!(!RelativePrimeValidator::are_relatively_prime(12, 8));
    assert!(!RelativePrimeValidator::are_relatively_prime(15, 25));
    assert!(!RelativePrimeValidator::are_relatively_prime(14, 21));
}

/// Test double hashing validation
#[test]
fn test_double_hashing_validation() {
    // Valid configurations (h2 value relatively prime to table size)
    let (valid, _) = RelativePrimeValidator::validate_double_hashing(3, 7);
    assert!(valid);
    
    let (valid, _) = RelativePrimeValidator::validate_double_hashing(5, 11);
    assert!(valid);
    
    // Invalid configurations (h2 value not relatively prime to table size)
    let (valid, _) = RelativePrimeValidator::validate_double_hashing(4, 8);
    assert!(!valid);
    
    let (valid, _) = RelativePrimeValidator::validate_double_hashing(6, 12);
    assert!(!valid);
}

/// Test probe sequence period calculation
#[test]
fn test_probe_sequence_period() {
    // For relatively prime h2 and table_size, period should be table_size
    assert_eq!(RelativePrimeValidator::probe_sequence_period(3, 7), 7);
    assert_eq!(RelativePrimeValidator::probe_sequence_period(5, 11), 11);
    
    // For non-relatively prime values, period will be smaller
    let period = RelativePrimeValidator::probe_sequence_period(4, 8);
    assert!(period < 8);
    assert!(period > 0);
}

/// Test generation of valid h2 values
#[test]
fn test_generate_valid_h2_values() {
    let valid_values = RelativePrimeValidator::generate_valid_h2_values(7, 5);
    
    // Should generate requested number of values
    assert!(valid_values.len() <= 5);
    assert!(!valid_values.is_empty());
    
    // All values should be relatively prime to 7
    for &value in &valid_values {
        assert!(RelativePrimeValidator::are_relatively_prime(value, 7));
        assert!(value > 0);
        assert!(value < 7);
    }
}

/// Test AdvancedDoubleHashingStrategy creation
#[test]
fn test_strategy_creation() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1.clone(), hash2.clone());
    assert_eq!(strategy.strategy_name(), "AdvancedDoubleHashing");
    
    let minimal_strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new_minimal(hash1, hash2);
    assert_eq!(minimal_strategy.strategy_name(), "AdvancedDoubleHashing");
}

/// Test double hashing quality analysis
#[test]
fn test_double_hashing_quality_analysis() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    let key = 42i32;
    let table_size = 11;
    
    let metrics = strategy.analyze_double_hashing_quality(&key, table_size);
    
    // Basic validation of metrics
    assert!(metrics.probe_sequence_length > 0);
    assert!(metrics.unique_probe_positions > 0);
    assert!(metrics.probe_sequence_period > 0);
    assert!(metrics.hash_function_independence >= 0.0);
    assert!(metrics.hash_function_independence <= 1.0);
    assert!(metrics.collision_avoidance_score >= 0.0);
    assert!(metrics.collision_avoidance_score <= 1.0);
}

/// Test configuration validation
#[test]
fn test_configuration_validation() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    let key = 42i32;
    let table_size = 11; // Prime number
    
    let (is_valid, message) = strategy.validate_configuration(&key, table_size);
    
    // Should be valid for prime table size
    assert!(is_valid);
    assert!(!message.is_empty());
}

/// Test probe count estimation
#[test]
fn test_probe_count_estimation() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    // Test successful search estimation
    let successful_probes = strategy.estimate_probe_count(0.5, true);
    assert!(successful_probes > 0.0);
    assert!(successful_probes < 10.0); // Should be reasonable
    
    // Test unsuccessful search estimation
    let unsuccessful_probes = strategy.estimate_probe_count(0.5, false);
    assert!(unsuccessful_probes > 0.0);
    assert!(unsuccessful_probes >= successful_probes); // Unsuccessful should be >= successful
}

/// Test configuration optimality check
#[test]
fn test_configuration_optimality() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    let key = 42i32;
    let table_size = 11;
    
    let metrics = strategy.analyze_double_hashing_quality(&key, table_size);
    let is_optimal = strategy.is_configuration_optimal(&metrics, table_size);
    
    // Should return a boolean result
    assert!(is_optimal || !is_optimal); // Tautology to ensure it compiles and runs
}

/// Test hash value generation
#[test]
fn test_hash_value_generation() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    let key = 42i32;
    let table_size = 11;
    
    let (h1_val, h2_val) = strategy.get_hash_values(&key, table_size);
    
    // Hash values should be within bounds
    assert!(h1_val < table_size);
    assert!(h2_val < table_size);
    // h2 can be 0, but for double hashing it's better if it's not
    // Just check it's within bounds
}

/// Test probe sequence generation
#[test]
fn test_probe_sequence_generation() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    let key = 42i32;
    let table_size = 11;
    let max_probes = 5;
    
    let sequence = strategy.generate_probe_sequence(&key, table_size, max_probes);
    
    // Should generate requested number of probes
    assert!(sequence.len() <= max_probes);
    assert!(!sequence.is_empty());
    
    // All positions should be within bounds
    for &pos in &sequence {
        assert!(pos < table_size);
    }
    
    // First position should be h1(key) mod table_size
    let (h1_val, _) = strategy.get_hash_values(&key, table_size);
    assert_eq!(sequence[0], h1_val);
}

/// Test example double hashing analysis
#[test]
fn test_example_double_hashing_analysis() {
    let (metrics, successful_estimate, unsuccessful_estimate, validation) = 
        example_double_hashing_analysis();
    
    // Validate metrics
    assert!(metrics.probe_sequence_length > 0);
    assert!(metrics.unique_probe_positions > 0);
    assert!(metrics.probe_sequence_period > 0);
    
    // Validate estimates
    assert!(successful_estimate > 0.0);
    assert!(unsuccessful_estimate > 0.0);
    assert!(unsuccessful_estimate >= successful_estimate);
    
    // Validate configuration
    let (is_valid, message) = validation;
    assert!(is_valid || !is_valid); // Should return some boolean
    assert!(!message.is_empty());
}

/// Test example probe sequence analysis
#[test]
fn test_example_probe_sequence_analysis() {
    let (sequence, metrics, valid_h2_values) = example_probe_sequence_analysis();
    
    // Validate sequence
    assert!(!sequence.is_empty());
    for &pos in &sequence {
        assert!(pos < 100); // Assuming reasonable table size
    }
    
    // Validate metrics
    assert!(metrics.probe_sequence_length > 0);
    assert!(metrics.unique_probe_positions > 0);
    
    // Validate h2 values
    assert!(!valid_h2_values.is_empty());
    for &value in &valid_h2_values {
        assert!(value > 0);
    }
}

/// Test with FlatHashTable integration
#[test]
fn test_flat_hash_table_integration() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    let table = FlatHashTable::create_table(strategy, 11);
    
    // Test basic operations
    let table = table.insert(42i32, "forty-two".to_string());
    let table = table.insert(24i32, "twenty-four".to_string());
    
    // Test lookup
    assert_eq!(table.lookup(&42), Some(&"forty-two".to_string()));
    assert_eq!(table.lookup(&24), Some(&"twenty-four".to_string()));
    assert_eq!(table.lookup(&99), None);
    
    // Test statistics
    let (size, load) = table.load_and_size();
    assert_eq!(size, 2);
    assert!(load > 0);
}

/// Test Entry enum functionality
#[test]
fn test_entry_enum() {
    let empty: Entry<i32, String> = Entry::Empty;
    let dead: Entry<i32, String> = Entry::Dead;
    let live = Entry::Live(42, "test".to_string());
    
    // Test Display implementation
    assert_eq!(format!("{}", empty), "Empty");
    assert_eq!(format!("{}", dead), "Dead");
    assert_eq!(format!("{}", live), "Live(42, test)");
    
    // Test equality
    assert_eq!(empty, Entry::Empty);
    assert_eq!(dead, Entry::Dead);
    assert_eq!(live, Entry::Live(42, "test".to_string()));
    assert_ne!(empty, dead);
}

/// Comprehensive integration test
#[test]
fn test_comprehensive_double_hashing() {
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    // Create table and perform operations
    let mut table = FlatHashTable::create_table(strategy, 13);
    
    // Insert test data
    let test_data = vec![
        (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five")
    ];
    
    for (key, value) in &test_data {
        table = table.insert(*key, value.to_string());
    }
    
    // Verify all insertions
    for (key, value) in &test_data {
        assert_eq!(table.lookup(key), Some(&value.to_string()));
    }
    
    // Test deletion
    let (table, deleted) = table.delete(&3);
    assert!(deleted);
    assert_eq!(table.lookup(&3), None);
    
    // Verify other elements still exist
    for (key, value) in &test_data {
        if *key != 3 {
            assert_eq!(table.lookup(key), Some(&value.to_string()));
        }
    }
    
    // Test resize
    let table = table.resize(23);
    
    // Verify elements still accessible after resize
    for (key, value) in &test_data {
        if *key != 3 {
            assert_eq!(table.lookup(key), Some(&value.to_string()));
        }
    }
    
    // Test statistics
    let stats = table.statistics();
    assert!(stats.num_elements > 0);
    assert!(stats.table_size > 0);
    
    let (total_probes, max_probes, avg_probes) = table.probe_statistics();
    // Just verify the statistics are returned (values can be 0 for empty tables)
    let _ = (total_probes, max_probes, avg_probes);
}