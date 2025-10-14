//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Tests for Advanced Linear Probing Strategy

use apas_ai::Chap47::AdvancedLinearProbing::AdvancedLinearProbing::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Types::Types::*;

#[test]
fn test_advanced_linear_probing_creation() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    assert_eq!(strategy.strategy_name(), "AdvancedLinearProbing");
    assert!(strategy.to_string().contains("clustering_enabled: true"));
}

#[test]
fn test_advanced_linear_probing_without_clustering() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new_without_clustering(hash_fn);

    assert!(strategy.to_string().contains("clustering_enabled: false"));
}

#[test]
fn test_linear_probing_sequence() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);
    let table_size = 11;
    let key = "test_key".to_string();

    // Test probe sequence: h_i(k) = (h(k) + i) mod m
    let pos0 = strategy.probe_hash(&key, 0, table_size);
    let pos1 = strategy.probe_hash(&key, 1, table_size);
    let pos2 = strategy.probe_hash(&key, 2, table_size);

    // Linear probing should increment by 1 each time
    assert_eq!(pos1, (pos0 + 1) % table_size);
    assert_eq!(pos2, (pos0 + 2) % table_size);
}

#[test]
fn test_primary_clustering_analysis_empty_table() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);
    let table: FlatHashTable<String, String, AdvancedLinearProbingStrategy<String, DefaultHashFunction>> =
        FlatHashTable::create_table(strategy.clone(), 16);

    let metrics = strategy.analyze_primary_clustering(&table);

    assert_eq!(metrics.total_clusters, 0);
    assert_eq!(metrics.max_cluster_size, 0);
    assert_eq!(metrics.avg_cluster_size, 0.0);
    assert_eq!(metrics.clustering_coefficient, 0.0);
    assert_eq!(metrics.probe_variance, 0.0);
}

#[test]
fn test_primary_clustering_analysis_disabled() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new_without_clustering(hash_fn);
    let table: FlatHashTable<String, String, AdvancedLinearProbingStrategy<String, DefaultHashFunction>> =
        FlatHashTable::create_table(strategy.clone(), 16);

    let metrics = strategy.analyze_primary_clustering(&table);

    // All metrics should be zero when clustering analysis is disabled
    assert_eq!(metrics.total_clusters, 0);
    assert_eq!(metrics.max_cluster_size, 0);
    assert_eq!(metrics.avg_cluster_size, 0.0);
    assert_eq!(metrics.clustering_coefficient, 0.0);
    assert_eq!(metrics.probe_variance, 0.0);
}

#[test]
fn test_probe_count_estimation_low_load_factor() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    let load_factor = 0.25;
    let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(load_factor);
    let successful_probes = strategy.estimate_successful_probe_count(load_factor);

    // For α = 0.25: unsuccessful ≈ 1/(1-0.25) = 1.33, successful ≈ (1/0.25)*ln(1/(1-0.25)) ≈ 1.15
    assert!((unsuccessful_probes - 1.33).abs() < 0.01);
    assert!((successful_probes - 1.15).abs() < 0.1);
}

#[test]
fn test_probe_count_estimation_moderate_load_factor() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    let load_factor = 0.5;
    let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(load_factor);
    let successful_probes = strategy.estimate_successful_probe_count(load_factor);

    // For α = 0.5: unsuccessful ≈ 1/(1-0.5) = 2.0, successful ≈ (1/0.5)*ln(1/(1-0.5)) ≈ 1.39
    assert!((unsuccessful_probes - 2.0).abs() < 0.01);
    assert!((successful_probes - 1.39).abs() < 0.1);
}

#[test]
fn test_probe_count_estimation_high_load_factor() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    let load_factor = 0.9;
    let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(load_factor);
    let successful_probes = strategy.estimate_successful_probe_count(load_factor);

    // For α = 0.9: unsuccessful ≈ 1/(1-0.9) = 10.0, successful ≈ (1/0.9)*ln(1/(1-0.9)) ≈ 2.56
    assert!((unsuccessful_probes - 10.0).abs() < 0.01);
    assert!((successful_probes - 2.56).abs() < 0.1);
}

#[test]
fn test_probe_count_estimation_full_table() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    let load_factor = 1.0;
    let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(load_factor);
    let successful_probes = strategy.estimate_successful_probe_count(load_factor);

    // For α = 1.0: both should be infinity
    assert!(unsuccessful_probes.is_infinite());
    assert!(successful_probes.is_infinite());
}

#[test]
fn test_clustering_problematic_detection() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    assert!(!strategy.is_clustering_problematic(0.5)); // α = 0.5 is acceptable
    assert!(!strategy.is_clustering_problematic(0.7)); // α = 0.7 is borderline
    assert!(strategy.is_clustering_problematic(0.75)); // α = 0.75 is problematic
    assert!(strategy.is_clustering_problematic(0.9)); // α = 0.9 is very problematic
}

#[test]
fn test_primary_clustering_metrics_display() {
    let metrics = PrimaryClusteringMetrics {
        total_clusters: 3,
        max_cluster_size: 5,
        avg_cluster_size: 3.5,
        clustering_coefficient: 1.2,
        probe_variance: 2.1,
    };

    let display_str = format!("{metrics}");
    assert!(display_str.contains("Primary Clustering Metrics:"));
    assert!(display_str.contains("Total clusters: 3"));
    assert!(display_str.contains("Max cluster size: 5"));
    assert!(display_str.contains("Avg cluster size: 3.50"));
    assert!(display_str.contains("Clustering coefficient: 1.200"));
    assert!(display_str.contains("Probe variance: 2.100"));
}

#[test]
fn test_example_primary_clustering_analysis() {
    let (_metrics, unsuccessful_probes, successful_probes) = example_primary_clustering_analysis();

    // Verify the example returns valid metrics
    assert!(unsuccessful_probes > 0.0);
    assert!(successful_probes > 0.0);
    assert!(unsuccessful_probes >= successful_probes); // Unsuccessful should be >= successful

    // For α = 0.5, unsuccessful should be around 2.0
    assert!((unsuccessful_probes - 2.0).abs() < 0.1);
}

#[test]
fn test_linear_probing_with_different_table_sizes() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);
    let key = "size_test".to_string();

    // Test with different table sizes
    for table_size in [7, 11, 13, 17, 19] {
        let pos0 = strategy.probe_hash(&key, 0, table_size);
        let pos1 = strategy.probe_hash(&key, 1, table_size);

        assert!(pos0 < table_size);
        assert!(pos1 < table_size);
        assert_eq!(pos1, (pos0 + 1) % table_size);
    }
}

#[test]
fn test_linear_probing_wraparound() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);
    let table_size = 5;
    let key = "wrap_test".to_string();

    // Generate enough probes to ensure wraparound
    let mut positions = Vec::new();
    for i in 0..table_size {
        positions.push(strategy.probe_hash(&key, i, table_size));
    }

    // All positions should be different and within bounds
    for pos in &positions {
        assert!(*pos < table_size);
    }

    // Should cover all positions in table_size probes
    let mut unique_positions: Vec<_> = positions.clone();
    unique_positions.sort();
    unique_positions.dedup();
    assert_eq!(unique_positions.len(), table_size);
}

#[test]
fn test_linear_probing_deterministic() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);
    let table_size = 11;
    let key = "deterministic_test".to_string();

    // Same key should always produce same probe sequence
    let pos1_first = strategy.probe_hash(&key, 1, table_size);
    let pos1_second = strategy.probe_hash(&key, 1, table_size);
    let pos5_first = strategy.probe_hash(&key, 5, table_size);
    let pos5_second = strategy.probe_hash(&key, 5, table_size);

    assert_eq!(pos1_first, pos1_second);
    assert_eq!(pos5_first, pos5_second);
}

#[test]
fn test_linear_probing_different_keys() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);
    let table_size = 13;

    let key1 = "key1".to_string();
    let key2 = "key2".to_string();

    let pos1_key1 = strategy.probe_hash(&key1, 0, table_size);
    let pos1_key2 = strategy.probe_hash(&key2, 0, table_size);

    // Different keys may hash to different initial positions
    // (though they could be the same due to hash collisions)
    assert!(pos1_key1 < table_size);
    assert!(pos1_key2 < table_size);

    // But their probe sequences should follow linear pattern
    let pos2_key1 = strategy.probe_hash(&key1, 1, table_size);
    let pos2_key2 = strategy.probe_hash(&key2, 1, table_size);

    assert_eq!(pos2_key1, (pos1_key1 + 1) % table_size);
    assert_eq!(pos2_key2, (pos1_key2 + 1) % table_size);
}

#[test]
fn test_linear_probing_edge_cases() {
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

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
    let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
        AdvancedLinearProbingStrategy::new(hash_fn);

    // Test with zero load factor
    let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(0.0);
    let successful_probes = strategy.estimate_successful_probe_count(0.0);

    assert_eq!(unsuccessful_probes, 1.0); // Should be exactly 1
    assert_eq!(successful_probes, 1.0); // Should be exactly 1

    // Test with very small load factor
    let small_lf = 0.01;
    let unsuccessful_small = strategy.estimate_unsuccessful_probe_count(small_lf);
    let successful_small = strategy.estimate_successful_probe_count(small_lf);

    assert!((unsuccessful_small - 1.01).abs() < 0.01);
    assert!(successful_small > 1.0 && successful_small < 1.1);
}
