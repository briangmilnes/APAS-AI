//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test module for ProbeSequenceExamples
//!
//! Tests the textbook examples for all three advanced probing strategies:
//! - Linear probing examples from textbook sections
//! - Quadratic probing examples with prime table sizes
//! - Double hashing examples with relative prime validation
//! - Comparative analysis of probe sequences
//!
//! Work: O(m) per probe sequence test where m is table size
//! Span: O(1) per probe sequence test

use std::collections::HashSet;

use apas_ai::Chap47::ProbeSequenceExamples::ProbeSequenceExamples::{
    comprehensive_probe_sequence_comparison, example_47_4_linear_probing_clustering,
    example_47_5_quadratic_probing_prime_table, example_47_6_double_hashing_optimal, load_factor_impact_analysis,
    prime_vs_composite_table_comparison, ProbeSequenceAnalyzer, ProbeSequenceVisualization, TextbookExampleResults,
};
use apas_ai::Types::Types::*;

/// Test ProbeSequenceAnalyzer creation and basic functionality
#[test]
fn test_probe_sequence_analyzer_creation() {
    let analyzer = ProbeSequenceAnalyzer::new();
    assert!(analyzer.max_probes > 0);
    assert!(analyzer.visualization_enabled);

    let minimal_analyzer = ProbeSequenceAnalyzer::new_minimal();
    assert!(minimal_analyzer.max_probes > 0);
    assert!(!minimal_analyzer.visualization_enabled);
}

/// Test linear probing analysis
#[test]
fn test_analyze_linear_probing() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "test";
    let table_size = 11;

    let visualization = analyzer.analyze_linear_probing(key, table_size);

    // Validate visualization properties
    assert_eq!(visualization.strategy_name, "Linear Probing");
    assert_eq!(visualization.key, key);
    assert_eq!(visualization.table_size, table_size);
    assert!(!visualization.probe_sequence.is_empty());
    assert!(visualization.sequence_length > 0);
    assert!(visualization.unique_positions > 0);
    assert!(visualization.period > 0);
    assert!(!visualization.collision_pattern.is_empty());

    // All positions should be within table bounds
    for &pos in &visualization.probe_sequence {
        assert!(pos < table_size);
    }

    // Linear probing should produce consecutive positions
    for i in 1..visualization.probe_sequence.len() {
        let expected = (visualization.probe_sequence[i - 1] + 1) % table_size;
        assert_eq!(visualization.probe_sequence[i], expected);
    }
}

/// Test quadratic probing analysis
#[test]
fn test_analyze_quadratic_probing() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "test";
    let table_size = 13; // Prime number
    let c1 = 1;
    let c2 = 1;

    let visualization = analyzer.analyze_quadratic_probing(key, table_size, c1, c2);

    // Validate visualization properties
    assert_eq!(visualization.strategy_name, "Quadratic Probing (c1=1, c2=1)");
    assert_eq!(visualization.key, key);
    assert_eq!(visualization.table_size, table_size);
    assert!(!visualization.probe_sequence.is_empty());
    assert!(visualization.sequence_length > 0);
    assert!(visualization.unique_positions > 0);
    assert!(!visualization.collision_pattern.is_empty());

    // All positions should be within table bounds
    for &pos in &visualization.probe_sequence {
        assert!(pos < table_size);
    }

    // Should not be consecutive like linear probing
    let mut consecutive_count = 0;
    for i in 1..visualization.probe_sequence.len() {
        if visualization.probe_sequence[i] == (visualization.probe_sequence[i - 1] + 1) % table_size {
            consecutive_count += 1;
        }
    }
    // Should have fewer consecutive positions than linear probing
    assert!(consecutive_count < visualization.probe_sequence.len() - 1);
}

/// Test double hashing analysis
#[test]
fn test_analyze_double_hashing() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "test";
    let table_size = 17; // Prime number

    let visualization = analyzer.analyze_double_hashing(key, table_size);

    // Validate visualization properties
    assert_eq!(visualization.strategy_name, "Double Hashing");
    assert_eq!(visualization.key, key);
    assert_eq!(visualization.table_size, table_size);
    assert!(!visualization.probe_sequence.is_empty());
    assert!(visualization.sequence_length > 0);
    assert!(visualization.unique_positions > 0);
    assert!(!visualization.collision_pattern.is_empty());

    // All positions should be within table bounds
    for &pos in &visualization.probe_sequence {
        assert!(pos < table_size);
    }

    // Should have good distribution (not consecutive)
    let mut consecutive_count = 0;
    for i in 1..visualization.probe_sequence.len() {
        if visualization.probe_sequence[i] == (visualization.probe_sequence[i - 1] + 1) % table_size {
            consecutive_count += 1;
        }
    }
    // Should have minimal consecutive positions
    assert!(consecutive_count < visualization.probe_sequence.len() / 2);
}

/// Test probing strategy comparison
#[test]
fn test_compare_probing_strategies() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "comparison";
    let table_size = 11;

    let comparisons = analyzer.compare_probing_strategies(key, table_size);

    // Should have all three strategies
    assert_eq!(comparisons.len(), 3);

    let mut found_linear = false;
    let mut found_quadratic = false;
    let mut found_double = false;

    for comparison in &comparisons {
        assert_eq!(comparison.key, key);
        assert_eq!(comparison.table_size, table_size);
        assert!(!comparison.probe_sequence.is_empty());

        if comparison.strategy_name.contains("Linear") {
            found_linear = true;
        } else if comparison.strategy_name.contains("Quadratic") {
            found_quadratic = true;
        } else if comparison.strategy_name.contains("Double") {
            found_double = true;
        } else {
            panic!("Unexpected strategy name: {}", comparison.strategy_name);
        }
    }

    assert!(found_linear);
    assert!(found_quadratic);
    assert!(found_double);
}

/// Test Example 47.4: Linear Probing Clustering
#[test]
fn test_example_47_4_linear_probing_clustering() {
    let example = example_47_4_linear_probing_clustering();

    // Validate example structure
    assert_eq!(example.example_name, "Example 47.4: Linear Probing Primary Clustering");
    assert!(!example.description.is_empty());
    assert!(example.table_size > 0);
    assert!(!example.keys.is_empty());
    assert!(!example.probe_sequences.is_empty());
    assert!(!example.clustering_analysis.is_empty());
    assert!(!example.performance_summary.is_empty());

    // All probe sequences should be linear probing
    for sequence in &example.probe_sequences {
        assert!(sequence.strategy_name.contains("Linear"));
        assert_eq!(sequence.table_size, example.table_size);
        assert!(!sequence.probe_sequence.is_empty());
    }

    // Should demonstrate clustering behavior
    assert!(example.clustering_analysis.contains("clustering") || example.clustering_analysis.contains("Clustering"));
}

/// Test Example 47.5: Quadratic Probing with Prime Table
#[test]
fn test_example_47_5_quadratic_probing_prime_table() {
    let example = example_47_5_quadratic_probing_prime_table();

    // Validate example structure
    assert_eq!(example.example_name, "Example 47.5: Quadratic Probing with Prime Table");
    assert!(!example.description.is_empty());
    assert!(example.table_size > 0);
    assert!(!example.keys.is_empty());
    assert!(!example.probe_sequences.is_empty());
    assert!(!example.clustering_analysis.is_empty());
    assert!(!example.performance_summary.is_empty());

    // All probe sequences should be quadratic probing
    for sequence in &example.probe_sequences {
        assert!(sequence.strategy_name.contains("Quadratic"));
        assert_eq!(sequence.table_size, example.table_size);
        assert!(!sequence.probe_sequence.is_empty());
    }

    // Should mention prime table size benefits
    assert!(example.description.contains("prime") || example.description.contains("Prime"));
}

/// Test Example 47.6: Double Hashing Optimal
#[test]
fn test_example_47_6_double_hashing_optimal() {
    let example = example_47_6_double_hashing_optimal();

    // Validate example structure
    assert_eq!(
        example.example_name,
        "Example 47.6: Double Hashing Optimal Distribution"
    );
    assert!(!example.description.is_empty());
    assert!(example.table_size > 0);
    assert!(!example.keys.is_empty());
    assert!(!example.probe_sequences.is_empty());
    assert!(!example.clustering_analysis.is_empty());
    assert!(!example.performance_summary.is_empty());

    // All probe sequences should be double hashing
    for sequence in &example.probe_sequences {
        assert!(sequence.strategy_name.contains("Double"));
        assert_eq!(sequence.table_size, example.table_size);
        assert!(!sequence.probe_sequence.is_empty());
    }

    // Should demonstrate optimal performance
    assert!(
        example.performance_summary.contains("optimal")
            || example.performance_summary.contains("Optimal")
            || example.performance_summary.contains("best")
    );
}

/// Test comprehensive probe sequence comparison
#[test]
fn test_comprehensive_probe_sequence_comparison() {
    let comparison = comprehensive_probe_sequence_comparison();

    // Validate comparison structure
    assert!(!comparison.example_name.is_empty());
    assert!(!comparison.description.is_empty());
    assert!(comparison.table_size > 0);
    assert!(!comparison.keys.is_empty());
    assert!(!comparison.probe_sequences.is_empty());
    assert!(!comparison.clustering_analysis.is_empty());
    assert!(!comparison.performance_summary.is_empty());

    // Should have sequences from all three strategies
    let mut strategies = HashSet::new();
    for sequence in &comparison.probe_sequences {
        strategies.insert(sequence.strategy_name.clone());
        assert_eq!(sequence.table_size, comparison.table_size);
        assert!(!sequence.probe_sequence.is_empty());
    }

    // Check that we have all three strategy types
    let has_linear = strategies.iter().any(|s| s.contains("Linear"));
    let has_quadratic = strategies.iter().any(|s| s.contains("Quadratic"));
    let has_double = strategies.iter().any(|s| s.contains("Double"));

    assert!(has_linear);
    assert!(has_quadratic);
    assert!(has_double);
}

/// Test load factor impact analysis
#[test]
fn test_load_factor_impact_analysis() {
    let analysis = load_factor_impact_analysis();

    // Should have multiple load factor scenarios
    assert!(!analysis.is_empty());
    assert!(analysis.len() >= 3); // At least a few load factors

    // Load factors should be in ascending order
    for i in 1..analysis.len() {
        assert!(analysis[i - 1].0 <= analysis[i].0);
    }

    // Each load factor should have valid visualizations
    for (load_factor, visualizations) in &analysis {
        assert!(*load_factor >= 0.0);
        assert!(*load_factor <= 1.0);
        assert!(!visualizations.is_empty());

        // Should have all three strategies
        let mut strategies = HashSet::new();
        for viz in visualizations {
            strategies.insert(viz.strategy_name.clone());
            assert!(!viz.probe_sequence.is_empty());
        }

        // Check that we have all three strategy types
        let has_linear = strategies.iter().any(|s| s.contains("Linear"));
        let has_quadratic = strategies.iter().any(|s| s.contains("Quadratic"));
        let has_double = strategies.iter().any(|s| s.contains("Double"));

        assert!(has_linear);
        assert!(has_quadratic);
        assert!(has_double);
    }
}

/// Test prime vs composite table comparison
#[test]
fn test_prime_vs_composite_table_comparison() {
    let (prime_example, composite_example) = prime_vs_composite_table_comparison();

    // Validate prime example
    assert!(!prime_example.example_name.is_empty());
    assert!(!prime_example.description.is_empty());
    assert!(prime_example.table_size > 0);
    assert!(!prime_example.probe_sequences.is_empty());

    // Validate composite example
    assert!(!composite_example.example_name.is_empty());
    assert!(!composite_example.description.is_empty());
    assert!(composite_example.table_size > 0);
    assert!(!composite_example.probe_sequences.is_empty());

    // Prime and composite should have different table sizes
    assert_ne!(prime_example.table_size, composite_example.table_size);

    // Should demonstrate the difference
    assert!(prime_example.description.contains("prime") || prime_example.description.contains("Prime"));
    assert!(composite_example.description.contains("composite") || composite_example.description.contains("Composite"));
}

/// Test probe sequence visualization properties
#[test]
fn test_probe_sequence_visualization_properties() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "properties";
    let table_size = 13;

    let linear_viz = analyzer.analyze_linear_probing(key, table_size);
    let quad_viz = analyzer.analyze_quadratic_probing(key, table_size, 1, 1);
    let double_viz = analyzer.analyze_double_hashing(key, table_size);

    // All should have same key and table size
    for viz in [&linear_viz, &quad_viz, &double_viz] {
        assert_eq!(viz.key, key);
        assert_eq!(viz.table_size, table_size);
        assert!(viz.sequence_length > 0);
        assert!(viz.unique_positions > 0);
        assert!(viz.unique_positions <= viz.sequence_length);
        assert!(viz.period > 0);
        assert!(!viz.collision_pattern.is_empty());
    }

    // Linear probing should have period equal to table size
    assert_eq!(linear_viz.period, table_size);

    // Unique positions should vary by strategy
    // Linear probing might have fewer unique positions due to clustering
    assert!(double_viz.unique_positions >= linear_viz.unique_positions);
}

/// Test analyzer with different configurations
#[test]
fn test_analyzer_configurations() {
    let full_analyzer = ProbeSequenceAnalyzer::new();
    let minimal_analyzer = ProbeSequenceAnalyzer::new_minimal();

    let key = "config";
    let table_size = 11;

    // Both should produce valid results
    let full_result = full_analyzer.analyze_linear_probing(key, table_size);
    let minimal_result = minimal_analyzer.analyze_linear_probing(key, table_size);

    // Results should have same basic properties
    assert_eq!(full_result.strategy_name, minimal_result.strategy_name);
    assert_eq!(full_result.key, minimal_result.key);
    assert_eq!(full_result.table_size, minimal_result.table_size);
    // Probe sequences might differ in length due to different configurations
    assert!(!full_result.probe_sequence.is_empty());
    assert!(!minimal_result.probe_sequence.is_empty());

    // Visualization details might differ
    // (minimal analyzer might have less detailed collision patterns)
}

/// Test edge cases with small table sizes
#[test]
fn test_edge_cases_small_tables() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "small";

    // Test with table size 1
    let viz1 = analyzer.analyze_linear_probing(key, 1);
    assert_eq!(viz1.table_size, 1);
    assert_eq!(viz1.probe_sequence, vec![0]);
    assert_eq!(viz1.sequence_length, 1);
    assert_eq!(viz1.unique_positions, 1);

    // Test with table size 2
    let viz2 = analyzer.analyze_linear_probing(key, 2);
    assert_eq!(viz2.table_size, 2);
    assert!(viz2.sequence_length <= 2);
    assert!(viz2.unique_positions <= 2);

    // Test with table size 3
    let viz3 = analyzer.analyze_quadratic_probing(key, 3, 1, 1);
    assert_eq!(viz3.table_size, 3);
    assert!(viz3.sequence_length <= 3);
    assert!(viz3.unique_positions <= 3);
}

/// Test with different key types and patterns
#[test]
fn test_different_key_patterns() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let table_size = 11;

    let keys = vec!["", "a", "test", "longer_key", "123", "special!@#"];

    for key in &keys {
        let linear_viz = analyzer.analyze_linear_probing(key, table_size);
        let quad_viz = analyzer.analyze_quadratic_probing(key, table_size, 1, 1);
        let double_viz = analyzer.analyze_double_hashing(key, table_size);

        // All should produce valid results
        for viz in [&linear_viz, &quad_viz, &double_viz] {
            assert_eq!(viz.key, *key);
            assert_eq!(viz.table_size, table_size);
            assert!(!viz.probe_sequence.is_empty());
            assert!(viz.sequence_length > 0);
            assert!(viz.unique_positions > 0);

            // All positions should be valid
            for &pos in &viz.probe_sequence {
                assert!(pos < table_size);
            }
        }
    }
}

/// Test collision pattern analysis
#[test]
fn test_collision_pattern_analysis() {
    let analyzer = ProbeSequenceAnalyzer::new();
    let key = "collision";
    let table_size = 7;

    let linear_viz = analyzer.analyze_linear_probing(key, table_size);
    let quad_viz = analyzer.analyze_quadratic_probing(key, table_size, 1, 1);
    let double_viz = analyzer.analyze_double_hashing(key, table_size);

    // All should have collision pattern descriptions
    assert!(!linear_viz.collision_pattern.is_empty());
    assert!(!quad_viz.collision_pattern.is_empty());
    assert!(!double_viz.collision_pattern.is_empty());

    // Patterns should be different for different strategies
    assert_ne!(linear_viz.collision_pattern, quad_viz.collision_pattern);
    assert_ne!(quad_viz.collision_pattern, double_viz.collision_pattern);

    // Linear probing should mention consecutive or clustering
    assert!(
        linear_viz.collision_pattern.contains("consecutive")
            || linear_viz.collision_pattern.contains("clustering")
            || linear_viz.collision_pattern.contains("linear")
    );
}

/// Comprehensive integration test
#[test]
fn test_comprehensive_probe_sequence_integration() {
    // Test all textbook examples
    let example_47_4 = example_47_4_linear_probing_clustering();
    let example_47_5 = example_47_5_quadratic_probing_prime_table();
    let example_47_6 = example_47_6_double_hashing_optimal();
    let comprehensive = comprehensive_probe_sequence_comparison();

    // All examples should be valid
    for example in [&example_47_4, &example_47_5, &example_47_6, &comprehensive] {
        assert!(!example.example_name.is_empty());
        assert!(!example.description.is_empty());
        assert!(example.table_size > 0);
        assert!(!example.keys.is_empty());
        assert!(!example.probe_sequences.is_empty());
        assert!(!example.clustering_analysis.is_empty());
        assert!(!example.performance_summary.is_empty());

        // All probe sequences should be valid
        for sequence in &example.probe_sequences {
            assert!(!sequence.strategy_name.is_empty());
            assert!(!sequence.key.is_empty());
            assert_eq!(sequence.table_size, example.table_size);
            assert!(!sequence.probe_sequence.is_empty());
            assert!(sequence.sequence_length > 0);
            assert!(sequence.unique_positions > 0);

            // All positions should be within bounds
            for &pos in &sequence.probe_sequence {
                assert!(pos < sequence.table_size);
            }
        }
    }

    // Test load factor analysis
    let load_analysis = load_factor_impact_analysis();
    assert!(!load_analysis.is_empty());

    for (load_factor, visualizations) in &load_analysis {
        assert!(*load_factor >= 0.0);
        assert!(*load_factor <= 1.0);
        assert!(!visualizations.is_empty());

        for viz in visualizations {
            assert!(!viz.strategy_name.is_empty());
            assert!(!viz.probe_sequence.is_empty());
        }
    }

    // Test prime vs composite comparison
    let (prime_ex, composite_ex) = prime_vs_composite_table_comparison();
    assert_ne!(prime_ex.table_size, composite_ex.table_size);
    assert!(!prime_ex.probe_sequences.is_empty());
    assert!(!composite_ex.probe_sequences.is_empty());

    // Test analyzer functionality
    let analyzer = ProbeSequenceAnalyzer::new();
    let comparisons = analyzer.compare_probing_strategies("integration", 13);
    assert_eq!(comparisons.len(), 3);

    // Verify all strategies are represented
    let strategies: HashSet<_> = comparisons.iter().map(|c| c.strategy_name.clone()).collect();

    // Check that we have all three strategy types
    let has_linear = strategies.iter().any(|s| s.contains("Linear"));
    let has_quadratic = strategies.iter().any(|s| s.contains("Quadratic"));
    let has_double = strategies.iter().any(|s| s.contains("Double"));

    assert!(has_linear);
    assert!(has_quadratic);
    assert!(has_double);
}
