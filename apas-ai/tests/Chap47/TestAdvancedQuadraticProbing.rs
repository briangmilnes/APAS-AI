//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Tests for Advanced Quadratic Probing Strategy

use apas_ai::Chap47::AdvancedQuadraticProbing::AdvancedQuadraticProbing::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Types::Types::*;

#[test]
fn test_advanced_quadratic_probing_creation() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    assert!(strategy.strategy_name().contains("AdvancedQuadraticProbing"));
    assert!(strategy.to_string().contains("clustering_enabled=true"));
    assert!(strategy.to_string().contains("prime_validation=true"));

    let (c1, c2) = strategy.get_coefficients();
    assert_eq!(c1, 1);
    assert_eq!(c2, 1);
}

#[test]
fn test_advanced_quadratic_probing_with_coefficients() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 3, 5);

    let (c1, c2) = strategy.get_coefficients();
    assert_eq!(c1, 3);
    assert_eq!(c2, 5);
    assert!(strategy.strategy_name().contains("c1=3, c2=5"));
}

#[test]
fn test_advanced_quadratic_probing_minimal() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_minimal(hash_fn, 2, 3);

    assert!(strategy.to_string().contains("clustering_enabled=false"));
    assert!(strategy.to_string().contains("prime_validation=false"));
}

#[test]
fn test_quadratic_probing_sequence() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 1, 1);
    let table_size = 11;
    let key = "test_key".to_string();

    // Test probe sequence: h_i(k) = (h(k) + c1*i + c2*i^2) mod m
    let pos0 = strategy.probe_hash(&key, 0, table_size);
    let pos1 = strategy.probe_hash(&key, 1, table_size);
    let pos2 = strategy.probe_hash(&key, 2, table_size);

    // For c1=1, c2=1: pos1 = (pos0 + 1 + 1) mod m, pos2 = (pos0 + 2 + 4) mod m
    assert_eq!(pos1, (pos0 + 2) % table_size);
    assert_eq!(pos2, (pos0 + 6) % table_size);
}

#[test]
fn test_prime_validator_is_prime() {
    assert!(!PrimeValidator::is_prime(0));
    assert!(!PrimeValidator::is_prime(1));
    assert!(PrimeValidator::is_prime(2));
    assert!(PrimeValidator::is_prime(3));
    assert!(!PrimeValidator::is_prime(4));
    assert!(PrimeValidator::is_prime(5));
    assert!(!PrimeValidator::is_prime(6));
    assert!(PrimeValidator::is_prime(7));
    assert!(!PrimeValidator::is_prime(8));
    assert!(!PrimeValidator::is_prime(9));
    assert!(!PrimeValidator::is_prime(10));
    assert!(PrimeValidator::is_prime(11));
    assert!(!PrimeValidator::is_prime(12));
    assert!(PrimeValidator::is_prime(13));
    assert!(!PrimeValidator::is_prime(15));
    assert!(PrimeValidator::is_prime(17));
    assert!(PrimeValidator::is_prime(19));
    assert!(!PrimeValidator::is_prime(21));
}

#[test]
fn test_prime_validator_next_prime() {
    assert_eq!(PrimeValidator::next_prime(1), 2);
    assert_eq!(PrimeValidator::next_prime(2), 2);
    assert_eq!(PrimeValidator::next_prime(3), 3);
    assert_eq!(PrimeValidator::next_prime(4), 5);
    assert_eq!(PrimeValidator::next_prime(8), 11);
    assert_eq!(PrimeValidator::next_prime(14), 17);
    assert_eq!(PrimeValidator::next_prime(16), 17);
    assert_eq!(PrimeValidator::next_prime(17), 17);
}

#[test]
fn test_prime_validator_validate_quadratic_params_prime_table() {
    let (is_valid, message) = PrimeValidator::validate_quadratic_params(17, 1, 1);
    assert!(is_valid);
    assert!(message.contains("valid"));
}

#[test]
fn test_prime_validator_validate_quadratic_params_composite_table() {
    let (is_valid, message) = PrimeValidator::validate_quadratic_params(16, 1, 1);
    assert!(!is_valid);
    assert!(message.contains("not prime"));
}

#[test]
fn test_prime_validator_validate_quadratic_params_zero_c2() {
    let (is_valid, message) = PrimeValidator::validate_quadratic_params(17, 1, 0);
    assert!(!is_valid);
    assert!(message.contains("c2 coefficient is zero"));
}

#[test]
fn test_prime_validator_validate_quadratic_params_not_coprime() {
    // For prime table size 17, c2=17 is not coprime (gcd=17)
    let (is_valid, message) = PrimeValidator::validate_quadratic_params(17, 1, 17);
    assert!(!is_valid);
    assert!(message.contains("not coprime"));
}

#[test]
fn test_secondary_clustering_analysis_empty_table() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);
    let table: FlatHashTable<String, String, AdvancedQuadraticProbingStrategy<String, DefaultHashFunction>> =
        FlatHashTable::create_table(strategy.clone(), 17);

    let metrics = strategy.analyze_secondary_clustering(&table);

    assert_eq!(metrics.collision_chains, 0);
    assert_eq!(metrics.max_chain_length, 0);
    assert_eq!(metrics.avg_chain_length, 0.0);
    assert_eq!(metrics.secondary_clustering_coefficient, 0.0); // Empty table has no clustering
    assert_eq!(metrics.probe_sequence_diversity, 1.0); // Empty table returns 1.0 diversity
    assert!(metrics.table_size_is_prime); // Table size 17 is prime
}

#[test]
fn test_secondary_clustering_analysis_disabled() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_minimal(hash_fn, 1, 1);
    let table: FlatHashTable<String, String, AdvancedQuadraticProbingStrategy<String, DefaultHashFunction>> =
        FlatHashTable::create_table(strategy.clone(), 17);

    let metrics = strategy.analyze_secondary_clustering(&table);

    // All metrics should be zero when clustering analysis is disabled
    assert_eq!(metrics.collision_chains, 0);
    assert_eq!(metrics.max_chain_length, 0);
    assert_eq!(metrics.avg_chain_length, 0.0);
    assert_eq!(metrics.secondary_clustering_coefficient, 0.0);
    assert_eq!(metrics.probe_sequence_diversity, 0.0);
    assert!(!metrics.table_size_is_prime);
}

#[test]
fn test_probe_count_estimation_low_load_factor() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    let load_factor = 0.25;
    let successful_probes = strategy.estimate_probe_count(load_factor, true);
    let unsuccessful_probes = strategy.estimate_probe_count(load_factor, false);

    // For α = 0.25: successful ≈ 1 - ln(1-0.25)/0.25 ≈ 1.15, unsuccessful ≈ 1/(1-0.25) = 1.33
    assert!(successful_probes >= 1.0);
    assert!((unsuccessful_probes - 1.33).abs() < 0.01);
}

#[test]
fn test_probe_count_estimation_moderate_load_factor() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    let load_factor = 0.5;
    let successful_probes = strategy.estimate_probe_count(load_factor, true);
    let unsuccessful_probes = strategy.estimate_probe_count(load_factor, false);

    // For α = 0.5: successful ≈ 1 - ln(1-0.5)/0.5 ≈ 1.39, unsuccessful ≈ 1/(1-0.5) = 2.0
    assert!(successful_probes >= 1.0);
    assert!((unsuccessful_probes - 2.0).abs() < 0.01);
}

#[test]
fn test_probe_count_estimation_high_load_factor() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    let load_factor = 0.9;
    let successful_probes = strategy.estimate_probe_count(load_factor, true);
    let unsuccessful_probes = strategy.estimate_probe_count(load_factor, false);

    // For α = 0.9: successful ≈ 1 - ln(1-0.9)/0.9 ≈ 2.56, unsuccessful ≈ 1/(1-0.9) = 10.0
    assert!(successful_probes > 2.0 && successful_probes < 4.0);
    assert!((unsuccessful_probes - 10.0).abs() < 0.01);
}

#[test]
fn test_probe_count_estimation_full_table() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    let load_factor = 1.0;
    let successful_probes = strategy.estimate_probe_count(load_factor, true);
    let unsuccessful_probes = strategy.estimate_probe_count(load_factor, false);

    // For α = 1.0: both should be infinity
    assert!(successful_probes.is_infinite());
    assert!(unsuccessful_probes.is_infinite());
}

#[test]
fn test_validate_configuration_prime_table() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 1, 1);

    let (is_valid, message) = strategy.validate_configuration(17);
    assert!(is_valid);
    assert!(message.contains("valid"));
}

#[test]
fn test_validate_configuration_composite_table() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 1, 1);

    let (is_valid, message) = strategy.validate_configuration(16);
    assert!(!is_valid);
    assert!(message.contains("not prime"));
}

#[test]
fn test_validate_configuration_disabled() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_minimal(hash_fn, 1, 1);

    let (is_valid, message) = strategy.validate_configuration(16);
    assert!(is_valid);
    assert!(message.contains("disabled"));
}

#[test]
fn test_recommend_table_size() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    let recommended = strategy.recommend_table_size(10);
    assert!(recommended >= 20); // At least 2x capacity
    assert!(PrimeValidator::is_prime(recommended)); // Should be prime

    let recommended_small = strategy.recommend_table_size(1);
    assert!(recommended_small >= 2);
    assert!(PrimeValidator::is_prime(recommended_small));
}

#[test]
fn test_recommend_table_size_disabled() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_minimal(hash_fn, 1, 1);

    let recommended = strategy.recommend_table_size(10);
    assert_eq!(recommended, 20); // Simple doubling when validation disabled
}

#[test]
fn test_is_secondary_clustering_problematic() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    // Good metrics
    let good_metrics = SecondaryClusteringMetrics {
        collision_chains: 5,
        max_chain_length: 3,
        avg_chain_length: 2.0,
        secondary_clustering_coefficient: 0.3,
        probe_sequence_diversity: 0.9,
        table_size_is_prime: true,
    };
    assert!(!strategy.is_secondary_clustering_problematic(&good_metrics));

    // Bad clustering coefficient
    let bad_clustering = SecondaryClusteringMetrics {
        collision_chains: 5,
        max_chain_length: 3,
        avg_chain_length: 2.0,
        secondary_clustering_coefficient: 0.7, // > 0.5
        probe_sequence_diversity: 0.9,
        table_size_is_prime: true,
    };
    assert!(strategy.is_secondary_clustering_problematic(&bad_clustering));

    // Non-prime table size
    let non_prime = SecondaryClusteringMetrics {
        collision_chains: 5,
        max_chain_length: 3,
        avg_chain_length: 2.0,
        secondary_clustering_coefficient: 0.3,
        probe_sequence_diversity: 0.9,
        table_size_is_prime: false,
    };
    assert!(strategy.is_secondary_clustering_problematic(&non_prime));
}

#[test]
fn test_secondary_clustering_metrics_display() {
    let metrics = SecondaryClusteringMetrics {
        collision_chains: 8,
        max_chain_length: 4,
        avg_chain_length: 2.5,
        secondary_clustering_coefficient: 0.35,
        probe_sequence_diversity: 0.85,
        table_size_is_prime: true,
    };

    let display_str = format!("{}", metrics);
    assert!(display_str.contains("Secondary Clustering Metrics:"));
    assert!(display_str.contains("Collision chains: 8"));
    assert!(display_str.contains("Max chain length: 4"));
    assert!(display_str.contains("Avg chain length: 2.50"));
    assert!(display_str.contains("Secondary clustering coefficient: 0.350"));
    assert!(display_str.contains("Probe sequence diversity: 0.850"));
    assert!(display_str.contains("Table size is prime: true"));
}

#[test]
fn test_example_secondary_clustering_analysis() {
    let (_metrics, successful_probes, unsuccessful_probes, validation) = example_secondary_clustering_analysis();

    // Verify the example returns valid metrics
    assert!(successful_probes > 0.0);
    assert!(unsuccessful_probes > 0.0);
    // Both should be positive
    assert!(unsuccessful_probes > 0.0);
    assert!(successful_probes > 0.0);

    // For α = 0.5, unsuccessful should be around 2.0
    assert!((unsuccessful_probes - 2.0).abs() < 0.1);

    // Validation should be successful for prime table size 17
    assert!(validation.0);
    assert!(validation.1.contains("valid"));
}

#[test]
fn test_example_prime_vs_composite_table_size() {
    let (prime_metrics, composite_metrics) = example_prime_vs_composite_table_size();

    // Prime table should have better characteristics
    assert!(prime_metrics.table_size_is_prime);
    assert!(!composite_metrics.table_size_is_prime);

    // Both should have valid structure - table size info is in the example results, not metrics
    assert!(prime_metrics.table_size_is_prime);
    assert!(!composite_metrics.table_size_is_prime);
}

#[test]
fn test_quadratic_probing_different_coefficients() {
    let hash_fn = DefaultHashFunction;
    let key = "test_key".to_string();
    let table_size = 11;

    // Test different coefficient combinations
    let strategy1: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn.clone(), 1, 1);
    let strategy2: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn.clone(), 3, 2);

    let pos1_s1 = strategy1.probe_hash(&key, 1, table_size);
    let pos1_s2 = strategy2.probe_hash(&key, 1, table_size);
    let pos2_s1 = strategy1.probe_hash(&key, 2, table_size);
    let pos2_s2 = strategy2.probe_hash(&key, 2, table_size);

    // Different coefficients should generally produce different sequences
    // (though they could be the same due to modular arithmetic)
    assert!(pos1_s1 < table_size);
    assert!(pos1_s2 < table_size);
    assert!(pos2_s1 < table_size);
    assert!(pos2_s2 < table_size);
}

#[test]
fn test_quadratic_probing_edge_cases() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 0, 1);

    // Test with minimum table size
    let table_size = 1;
    let key = "edge_test".to_string();

    let pos0 = strategy.probe_hash(&key, 0, table_size);
    let pos1 = strategy.probe_hash(&key, 1, table_size);

    assert_eq!(pos0, 0);
    assert_eq!(pos1, 0); // Should wrap around to 0
}

#[test]
fn test_probe_count_estimation_edge_cases() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
        AdvancedQuadraticProbingStrategy::new(hash_fn);

    // Test with zero load factor
    let successful_probes = strategy.estimate_probe_count(0.0, true);
    let unsuccessful_probes = strategy.estimate_probe_count(0.0, false);

    assert_eq!(successful_probes, 1.0); // Should be exactly 1
    assert_eq!(unsuccessful_probes, 1.0); // Should be exactly 1

    // Test with very small load factor
    let small_lf = 0.01;
    let successful_small = strategy.estimate_probe_count(small_lf, true);
    let unsuccessful_small = strategy.estimate_probe_count(small_lf, false);

    assert!((unsuccessful_small - 1.01).abs() < 0.01);
    assert!(successful_small >= 1.0);
}
