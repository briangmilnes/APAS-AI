//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for multi-threaded ephemeral reducer-augmented ordered table implementation.

use apas_ai::AugOrderedTableMtEphLit;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::*;
use apas_ai::Types::Types::*;
use std::sync::Arc;
use std::thread;

#[test]
fn test_empty_table() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table: AugOrderedTableMtEph<String, i32, _> = AugOrderedTableMtEph::empty(max_reducer, i32::MIN);

    assert_eq!(table.size(), 0);
    assert_eq!(table.reduce_val(), i32::MIN);
    assert!(table.is_empty());
    assert!(table.first_key().is_none());
    assert!(table.last_key().is_none());
}

#[test]
fn test_singleton_table() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table = AugOrderedTableMtEph::singleton(42, 100, max_reducer, i32::MIN);

    assert_eq!(table.size(), 1);
    assert_eq!(table.reduce_val(), 100);
    assert_eq!(table.find(&42), Some(100));
    assert_eq!(table.lookup(&42), Some(100));
    assert!(!table.is_empty());
    assert_eq!(table.first_key(), Some(42));
    assert_eq!(table.last_key(), Some(42));
}

#[test]
fn test_insert_and_reduce_val() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableMtEph::empty(max_reducer, i32::MIN);

    table.insert(1, 50, |_old, new| *new);
    assert_eq!(table.size(), 1);
    assert_eq!(table.reduce_val(), 50);

    table.insert(2, 30, |_old, new| *new);
    assert_eq!(table.size(), 2);
    assert_eq!(table.reduce_val(), 50); // max(50, 30) = 50

    table.insert(3, 80, |_old, new| *new);
    assert_eq!(table.size(), 3);
    assert_eq!(table.reduce_val(), 80); // max(50, 30, 80) = 80

    table.insert(4, 20, |_old, new| *new);
    assert_eq!(table.size(), 4);
    assert_eq!(table.reduce_val(), 80); // max(50, 30, 80, 20) = 80
}

#[test]
fn test_thread_safety_basic() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let table = Arc::new(std::sync::Mutex::new(AugOrderedTableMtEph::empty(sum_reducer, 0)));

    let mut handles = vec![];

    // Spawn multiple threads to insert data
    for i in 0..10 {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            let mut t = table_clone.lock().unwrap();
            t.insert(i, i * 10, |_old, new| *new);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let final_table = table.lock().unwrap();
    assert_eq!(final_table.size(), 10);
    // Sum should be 0*10 + 1*10 + ... + 9*10 = 450
    assert_eq!(final_table.reduce_val(), 450);
}

#[test]
fn test_parallel_range_reduction() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableMtEph::empty(max_reducer, 0);

    // Insert a large dataset
    for i in 1..=2000 {
        let value = if i % 100 == 0 { i * 2 } else { i }; // Some peaks every 100
        table.insert(i, value, |_old, new| *new);
    }

    // Test parallel range reduction (should use parallel algorithm for large ranges)
    let range_max = table.reduce_range_parallel(&500, &1500);
    let expected_max = 1400 * 2; // Peak at 1400
    assert_eq!(range_max, expected_max);

    // Compare with sequential range reduction
    let sequential_max = table.reduce_range(&500, &1500);
    assert_eq!(range_max, sequential_max);
}

#[test]
fn test_concurrent_reads() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    // Populate table
    for i in 1..=100 {
        table.insert(i, i, |_old, new| *new);
    }

    let table = Arc::new(table);
    let mut handles = vec![];

    // Spawn multiple reader threads
    for _ in 0..10 {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            let total = table_clone.reduce_val();
            let range_sum = table_clone.reduce_range(&25, &75);
            (total, range_sum)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // All threads should get the same results
    let expected_total = (1..=100).sum::<i32>(); // 5050
    let expected_range = (25..=75).sum::<i32>(); // Sum from 25 to 75

    for (total, range_sum) in results {
        assert_eq!(total, expected_total);
        assert_eq!(range_sum, expected_range);
    }
}

#[test]
fn test_qadsan_multithreaded_scenario() {
    // Multi-threaded QADSAN scenario: concurrent stock price updates
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table = Arc::new(std::sync::Mutex::new(AugOrderedTableMtEph::empty(max_reducer, 0)));

    let mut handles = vec![];

    // Simulate multiple trading venues updating prices concurrently
    let venues = vec!["NYSE", "NASDAQ", "CBOE"];
    for (venue_id, _venue_name) in venues.iter().enumerate() {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            let base_time = (venue_id * 100) as i32;
            for minute in 0..60 {
                let timestamp = base_time + minute;
                let price = 15000 + (minute * 10) + (venue_id as i32 * 50);

                let mut t = table_clone.lock().unwrap();
                t.insert(timestamp, price, |old, new| if old > new { *old } else { *new });
            }
        });
        handles.push(handle);
    }

    // Wait for all venues to finish updating
    for handle in handles {
        handle.join().unwrap();
    }

    let final_table = table.lock().unwrap();

    // Verify we have data from all venues
    assert!(final_table.size() > 0);

    // The maximum price should be from the highest venue with the latest time
    let max_price = final_table.reduce_val();
    assert!(max_price >= 15000); // At least the base price

    // Test range queries for different time periods
    let morning_max = final_table.reduce_range(&0, &100);
    let midday_max = final_table.reduce_range(&100, &200);

    assert!(morning_max > 0);
    assert!(midday_max > 0);
}

#[test]
fn test_sum_reducer_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    assert_eq!(table.reduce_val(), 60);

    // Test in-place update with thread-safe combine function
    table.insert(2, 25, |old, new| old + new); // Combine old and new
    assert_eq!(table.reduce_val(), 85); // 10 + 45 + 30 = 85
    assert_eq!(table.find(&2), Some(45)); // 20 + 25 = 45

    // Test deletion
    table.delete(&1);
    assert_eq!(table.reduce_val(), 75); // 45 + 30 = 75
    assert_eq!(table.size(), 2);
}

#[test]
fn test_string_concatenation_multithreaded() {
    let concat_reducer = |a: &String, b: &String| format!("{}{}", a, b);
    let mut table = AugOrderedTableMtEph::empty(concat_reducer, String::new());

    table.insert(1, "Hello".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "Hello");

    table.insert(2, " ".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "Hello ");

    table.insert(3, "World".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "Hello World");

    // Test thread-safe modification
    table.insert(2, "Beautiful ".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "HelloBeautiful World");
}

#[test]
fn test_split_and_join_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);
    table.insert(5, 50, |_old, new| *new);
    table.insert(7, 70, |_old, new| *new);

    assert_eq!(table.reduce_val(), 160);

    // Split at key 4
    let (left, right) = table.split_key(&4);

    // Left should have keys 1,3 with values 10,30
    assert_eq!(left.reduce_val(), 40);
    assert_eq!(left.size(), 2);

    // Right should have keys 5,7 with values 50,70
    assert_eq!(right.reduce_val(), 120);
    assert_eq!(right.size(), 2);

    // Join them back (note: this consumes the tables)
    let mut rejoined = left;
    rejoined.join_key(right);
    assert_eq!(rejoined.reduce_val(), 160);
    assert_eq!(rejoined.size(), 4);
}

#[test]
fn test_map_operation_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    assert_eq!(table.reduce_val(), 60);

    // Double all values (creates new table) - uses thread-safe function
    let doubled = table.map(|_k, v| v * 2);
    assert_eq!(doubled.reduce_val(), 120); // 20+40+60 = 120
    assert_eq!(doubled.size(), 3);

    // Original table unchanged
    assert_eq!(table.reduce_val(), 60);
}

#[test]
fn test_filter_operation_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    for i in 1..=10 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    assert_eq!(table.reduce_val(), 550);

    // Filter even keys (creates new table) - uses thread-safe predicate
    let even_table = table.filter(|k, _v| k % 2 == 0);

    // Even keys: 2,4,6,8,10 with values 20,40,60,80,100
    let expected_sum = 20 + 40 + 60 + 80 + 100;
    assert_eq!(even_table.reduce_val(), expected_sum);
    assert_eq!(even_table.size(), 5);

    // Original table unchanged
    assert_eq!(table.reduce_val(), 550);
}

#[test]
fn test_union_operation_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table1 = AugOrderedTableMtEph::empty(sum_reducer.clone(), 0);
    let mut table2 = AugOrderedTableMtEph::empty(sum_reducer, 0);

    table1.insert(1, 10, |_old, new| *new);
    table1.insert(2, 20, |_old, new| *new);

    table2.insert(2, 25, |_old, new| *new); // Overlapping key
    table2.insert(3, 30, |_old, new| *new);

    // Union modifies table1 in place - uses thread-safe combine function
    table1.union(&table2, |v1, v2| v1 + v2);

    // Keys: 1->10, 2->45 (20+25), 3->30
    assert_eq!(table1.reduce_val(), 85); // 10+45+30
    assert_eq!(table1.size(), 3);
    assert_eq!(table1.find(&2), Some(45));
}

#[test]
fn test_intersection_operation_multithreaded() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table1 = AugOrderedTableMtEph::empty(max_reducer.clone(), 0);
    let mut table2 = AugOrderedTableMtEph::empty(max_reducer, 0);

    table1.insert(1, 10, |_old, new| *new);
    table1.insert(2, 20, |_old, new| *new);
    table1.insert(3, 30, |_old, new| *new);

    table2.insert(2, 25, |_old, new| *new);
    table2.insert(3, 15, |_old, new| *new);
    table2.insert(4, 40, |_old, new| *new);

    // Intersection modifies table1 in place - uses thread-safe combine function
    table1.intersection(&table2, |v1, v2| if v1 > v2 { *v1 } else { *v2 });

    // Only keys 2,3 remain: 2->25 (max(20,25)), 3->30 (max(30,15))
    assert_eq!(table1.size(), 2);
    assert_eq!(table1.find(&2), Some(25));
    assert_eq!(table1.find(&3), Some(30));
    assert_eq!(table1.reduce_val(), 30); // max(25, 30) = 30
}

#[test]
fn test_macro_construction_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;

    let table = AugOrderedTableMtEphLit![
        reducer: sum_reducer, identity: 0,
        1 => 100,
        2 => 200,
        3 => 300
    ];

    assert_eq!(table.size(), 3);
    assert_eq!(table.reduce_val(), 600);
    assert_eq!(table.find(&2), Some(200));

    // Empty table via macro
    let empty_table: AugOrderedTableMtEph<i32, i32, _> = AugOrderedTableMtEphLit![
        reducer: sum_reducer, identity: 0
    ];
    assert_eq!(empty_table.size(), 0);
    assert_eq!(empty_table.reduce_val(), 0);
}

#[test]
fn test_display_and_debug_multithreaded() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table = AugOrderedTableMtEph::singleton(42, 100, max_reducer, 0);

    let display_str = format!("{}", table);
    assert!(display_str.contains("AugOrderedTableMtEph"));
    assert!(display_str.contains("size: 1"));
    assert!(display_str.contains("reduction: 100"));

    let debug_str = format!("{:?}", table);
    assert!(debug_str.contains("AugOrderedTableMtEph"));
    assert!(debug_str.contains("size"));
    assert!(debug_str.contains("cached_reduction"));
}

#[test]
fn test_ordering_operations_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    table.insert(5, 50, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(8, 80, |_old, new| *new);
    table.insert(1, 10, |_old, new| *new);
    table.insert(9, 90, |_old, new| *new);

    // Test ordering operations (these are sequential by nature)
    assert_eq!(table.first_key(), Some(1));
    assert_eq!(table.last_key(), Some(9));
    assert_eq!(table.previous_key(&5), Some(2));
    assert_eq!(table.next_key(&5), Some(8));

    // Test rank and select
    assert_eq!(table.rank_key(&5), 2); // 5 is the 3rd key (0-indexed: rank 2)
    assert_eq!(table.select_key(2), Some(5)); // 3rd key (0-indexed: index 2)
}

#[test]
fn test_reduce_operation_multithreaded() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    // Test general reduce operation (different from reduce_val) - uses thread-safe function
    let sum_of_keys = table.reduce(0, |acc, k, _v| acc + k);
    assert_eq!(sum_of_keys, 6); // 1 + 2 + 3 = 6

    let sum_of_values = table.reduce(0, |acc, _k, v| acc + v);
    assert_eq!(sum_of_values, 60); // 10 + 20 + 30 = 60

    let key_value_product_sum = table.reduce(0, |acc, k, v| acc + (k * v));
    assert_eq!(key_value_product_sum, 140); // 1*10 + 2*20 + 3*30 = 10 + 40 + 90 = 140
}

#[test]
fn test_complex_multithreaded_scenario() {
    // Complex multi-threaded scenario with concurrent operations
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let table = Arc::new(std::sync::Mutex::new(AugOrderedTableMtEph::empty(sum_reducer, 0)));

    let mut handles = vec![];

    // Phase 1: Concurrent insertions
    for thread_id in 0..5 {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            for i in 0..20 {
                let key = thread_id * 100 + i;
                let value = key * 2;
                let mut t = table_clone.lock().unwrap();
                t.insert(key, value, |_old, new| *new);
            }
        });
        handles.push(handle);
    }

    // Wait for insertions to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Phase 2: Concurrent reads and range queries
    let mut read_handles = vec![];
    for _ in 0..3 {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            let t = table_clone.lock().unwrap();
            let total = t.reduce_val();
            let range1 = t.reduce_range(&0, &100);
            let range2 = t.reduce_range(&200, &300);
            (total, range1, range2)
        });
        read_handles.push(handle);
    }

    // Collect read results
    let mut read_results = vec![];
    for handle in read_handles {
        read_results.push(handle.join().unwrap());
    }

    // Verify all readers got consistent results
    let first_result = &read_results[0];
    for result in &read_results[1..] {
        assert_eq!(result.0, first_result.0); // Total should be same
        assert_eq!(result.1, first_result.1); // Range 1 should be same
        assert_eq!(result.2, first_result.2); // Range 2 should be same
    }

    // Verify final state
    let final_table = table.lock().unwrap();
    assert_eq!(final_table.size(), 100); // 5 threads * 20 insertions each
    assert!(final_table.reduce_val() > 0); // Should have positive sum
}
