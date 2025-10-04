//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Examples Tests

use apas_ai::Chap47::HashExamples::HashExamples::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Types::Types::*;

#[test]
fn test_example_47_1_hash_function() {
    let hash_fn = example_47_1_hash_function();

    // Test the hash function from Example 47.1
    assert_eq!(hash_fn.hash(&"aa".to_string(), 5), 0); // (0+0) % 5 = 0
    assert_eq!(hash_fn.hash(&"bb".to_string(), 5), 2); // (1+1) % 5 = 2
    assert_eq!(hash_fn.hash(&"cc".to_string(), 5), 4); // (2+2) % 5 = 4
    assert_eq!(hash_fn.hash(&"dd".to_string(), 5), 1); // (3+3) % 5 = 1
    assert_eq!(hash_fn.hash(&"ee".to_string(), 5), 3); // (4+4) % 5 = 3
}

#[test]
fn test_example_47_1_demonstration() {
    let hash_demo = example_47_1_demonstration();

    // Should return hash values for the test strings
    assert_eq!(hash_demo.len(), 10);

    // Check specific mappings from Example 47.1
    let aa_entry = hash_demo.iter().find(|(key, _)| key == "aa").unwrap();
    assert_eq!(aa_entry.1, 0);

    let bb_entry = hash_demo.iter().find(|(key, _)| key == "bb").unwrap();
    assert_eq!(bb_entry.1, 2);

    let cc_entry = hash_demo.iter().find(|(key, _)| key == "cc").unwrap();
    assert_eq!(cc_entry.1, 4);
}

#[test]
fn test_example_47_2_separate_chaining() {
    let table = example_47_2_separate_chaining();

    // Test that all example data is present and accessible
    assert_eq!(table.lookup(&"aa".to_string()), Some(&"a".to_string()));
    assert_eq!(table.lookup(&"bb".to_string()), Some(&"b".to_string()));
    assert_eq!(table.lookup(&"cc".to_string()), Some(&"c".to_string()));
    assert_eq!(table.lookup(&"dd".to_string()), Some(&"d".to_string()));
    assert_eq!(table.lookup(&"ee".to_string()), Some(&"e".to_string()));
    assert_eq!(table.lookup(&"ff".to_string()), Some(&"f".to_string()));
    assert_eq!(table.lookup(&"gg".to_string()), Some(&"g".to_string()));
    assert_eq!(table.lookup(&"hh".to_string()), Some(&"h".to_string()));
    assert_eq!(table.lookup(&"ii".to_string()), Some(&"i".to_string()));
    assert_eq!(table.lookup(&"jj".to_string()), Some(&"j".to_string()));

    let (load, size) = table.load_and_size();
    assert_eq!(load, 10);
    assert_eq!(size, 16); // Auto-resizes from min size 8

    // Should have collisions due to multiple keys mapping to same buckets
    let stats = table.statistics();
    assert!(stats.num_collisions > 0);
    assert!(stats.max_chain_length > 1);
}

#[test]
fn test_example_47_4_probe_sequence() {
    let probe_demo = example_47_4_probe_sequence();

    // Should return probe sequences for the test keys
    assert!(!probe_demo.is_empty());

    // Each key should have a probe sequence of length 8 (table size)
    for (key, sequence) in &probe_demo {
        assert_eq!(sequence.len(), 8);
        assert!(!key.is_empty());

        // All probe values should be in range [0, 8)
        for &probe_value in sequence {
            assert!(probe_value < 8);
        }
    }

    // Different keys should generally have different probe sequences
    if probe_demo.len() > 1 {
        let first_seq = &probe_demo[0].1;
        let second_seq = &probe_demo[1].1;
        assert_ne!(first_seq, second_seq);
    }
}

#[test]
fn test_example_47_5_deleted_entries() {
    let table = example_47_5_deleted_entries();

    // Should have some entries but not the deleted one
    let (load, _) = table.load_and_size();
    assert!(load > 0);

    // Should be able to lookup remaining entries
    // (Exact entries depend on which one was deleted in the example)
    let stats = table.statistics();
    assert!(stats.num_elements > 0);

    // Should have some dead entries from the deletion
    let (live, dead, _) = table.probe_statistics();
    assert!(live > 0);
    assert!(dead > 0);
}

#[test]
fn test_example_47_6_collision_handling() {
    let table = example_47_6_collision_handling();

    // Should contain all the inserted keys
    let (load, _) = table.load_and_size();
    assert_eq!(load, 5); // B, E, A, F, D

    // All keys should be retrievable
    assert_eq!(table.lookup(&"B".to_string()), Some(&"B_value".to_string()));
    assert_eq!(table.lookup(&"E".to_string()), Some(&"E_value".to_string()));
    assert_eq!(table.lookup(&"A".to_string()), Some(&"A_value".to_string()));
    assert_eq!(table.lookup(&"F".to_string()), Some(&"F_value".to_string()));
    assert_eq!(table.lookup(&"D".to_string()), Some(&"D_value".to_string()));
}

#[test]
fn test_exercise_47_1_nested_implementation() {
    let solution = exercise_47_1_nested_implementation();

    // Should contain explanation of nested table implementation
    assert!(solution.contains("Exercise 47.1"));
    assert!(solution.contains("nested hash table"));
    assert!(solution.contains("Table ADT"));
    assert!(solution.contains("createTable"));
    assert!(solution.contains("insert"));
    assert!(solution.contains("lookup"));
    assert!(solution.contains("O(1 + α)"));
}

#[test]
fn test_exercise_47_2_size_reduction() {
    let solution = exercise_47_2_size_reduction();

    // Should contain analysis of table size reduction
    assert!(solution.contains("Exercise 47.2"));
    assert!(solution.contains("reduce hash table size"));
    assert!(solution.contains("load factor"));
    assert!(solution.contains("α < 0.25"));
    assert!(solution.contains("memory usage"));
    assert!(solution.contains("rehash"));
}

#[test]
fn test_exercise_47_3_resize_implementation() {
    let solution = exercise_47_3_resize_implementation();

    // Should contain resize operation details
    assert!(solution.contains("Exercise 47.3"));
    assert!(solution.contains("resize operation"));
    assert!(solution.contains("separate chaining"));
    assert!(solution.contains("Θ(n)"));
    assert!(solution.contains("amortized"));
    assert!(solution.contains("O(1)"));
}

#[test]
fn test_exercise_47_6_higher_order() {
    let solution = exercise_47_6_higher_order();

    // Should contain higher-order function explanation
    assert!(solution.contains("Exercise 47.6"));
    assert!(solution.contains("higher-order function"));
    assert!(solution.contains("probe_operation"));
    assert!(solution.contains("lookup"));
    assert!(solution.contains("insert"));
    assert!(solution.contains("delete"));
}

#[test]
fn test_exercise_47_7_complete_implementation() {
    let solution = exercise_47_7_complete_implementation();

    // Should contain complete implementation details
    assert!(solution.contains("Exercise 47.7"));
    assert!(solution.contains("flat hash table"));
    assert!(solution.contains("resize"));
    assert!(solution.contains("load factor"));
    assert!(solution.contains("statistics"));
    assert!(solution.contains("probe sequence"));
}

#[test]
fn test_run_all_examples() {
    let output = run_all_examples();

    // Should contain output from all examples
    assert!(output.contains("Chapter 47 Hash Table Examples"));
    assert!(output.contains("Example 47.1"));
    assert!(output.contains("Example 47.2"));
    assert!(output.contains("Example 47.4"));
    assert!(output.contains("Exercise Solutions"));

    // Should contain hash values and statistics
    assert!(output.contains("hash("));
    assert!(output.contains("load_factor"));
    assert!(output.contains("probe"));

    // Should be substantial output
    assert!(output.len() > 1000);
}

#[test]
fn test_hash_function_consistency() {
    let hash_fn = example_47_1_hash_function();

    // Same input should always produce same output
    let key = "test".to_string();
    let table_size = 10;

    let hash1 = hash_fn.hash(&key, table_size);
    let hash2 = hash_fn.hash(&key, table_size);
    let hash3 = hash_fn.hash(&key, table_size);

    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
    assert!(hash1 < table_size);
}

#[test]
fn test_example_data_integrity() {
    // Test that examples maintain data integrity across operations
    let table = example_47_2_separate_chaining();

    // Count total elements
    let (load, _) = table.load_and_size();
    assert_eq!(load, 10);

    // Verify no data loss in collision handling
    let stats = table.statistics();
    assert_eq!(stats.num_elements, 10);

    // All original keys should be present
    let test_keys = vec!["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"];
    for key in test_keys {
        assert!(table.lookup(&key.to_string()).is_some());
    }
}

#[test]
fn test_textbook_fidelity() {
    // Verify that examples match textbook specifications exactly

    // Example 47.1: String position hash function
    let hash_fn = example_47_1_hash_function();
    assert_eq!(hash_fn.hash(&"aa".to_string(), 5), 0);
    assert_eq!(hash_fn.hash(&"ff".to_string(), 5), 0); // Should collide with "aa"

    // Example 47.2: Separate chaining with specific bucket distribution
    let table = example_47_2_separate_chaining();
    let stats = table.statistics();

    // With 10 elements in 16 buckets (auto-resized from min 8), load factor is 0.625
    assert_eq!(stats.load_factor, 0.625);

    // Should have exactly the collisions predicted by the hash function
    assert!(stats.num_collisions > 0);
}
