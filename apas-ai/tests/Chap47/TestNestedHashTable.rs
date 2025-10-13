//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 47: NestedHashTable - Nested Hash Table implementation

use apas_ai::Chap47::NestedHashTable::NestedHashTable::*;
use apas_ai::Types::Types::*;
use apas_ai::NestedHashTableLit;

#[test]
fn test_nestedhashtablelit_macro_functionality() {
    // Test hash table creation with size
    let empty: NestedHashTable<String, i32> = NestedHashTableLit!(10);
    assert_eq!(empty.size(), 0);
    
    // Test hash table creation with size and key-value pairs
    let with_data: NestedHashTable<String, i32> = NestedHashTableLit!(
        10, 
        ("key1".to_string(), 1),
        ("key2".to_string(), 2)
    );
    assert_eq!(with_data.size(), 2);
}

#[test]
fn test_create_nested_hash_table() {
    let table: NestedHashTable<String, i32> = NestedHashTable::create_table(10);
    assert_eq!(table.size(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_nested_hash_table_insert_and_lookup() {
    let mut table = NestedHashTable::create_table(10);
    
    // Insert some key-value pairs
    table = table.insert("alpha".to_string(), 1);
    table = table.insert("beta".to_string(), 2);
    table = table.insert("gamma".to_string(), 3);
    table = table.insert("delta".to_string(), 4);
    
    assert_eq!(table.size(), 4);
    assert!(!table.is_empty());
    
    // Test lookups
    assert_eq!(table.lookup(&"alpha".to_string()), Some(&1));
    assert_eq!(table.lookup(&"beta".to_string()), Some(&2));
    assert_eq!(table.lookup(&"gamma".to_string()), Some(&3));
    assert_eq!(table.lookup(&"delta".to_string()), Some(&4));
    assert_eq!(table.lookup(&"epsilon".to_string()), None);
}

#[test]
fn test_nested_hash_table_delete() {
    let mut table = NestedHashTable::create_table(10);
    
    // Insert and then delete
    table = table.insert("first".to_string(), 100);
    table = table.insert("second".to_string(), 200);
    table = table.insert("third".to_string(), 300);
    
    assert_eq!(table.size(), 3);
    assert_eq!(table.lookup(&"second".to_string()), Some(&200));
    
    let (table, _deleted) = table.delete(&"second".to_string());
    
    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&"second".to_string()), None);
    assert_eq!(table.lookup(&"first".to_string()), Some(&100));
    assert_eq!(table.lookup(&"third".to_string()), Some(&300));
}

#[test]
fn test_nested_hash_table_update() {
    let mut table = NestedHashTable::create_table(10);
    
    table = table.insert("key".to_string(), 42);
    assert_eq!(table.lookup(&"key".to_string()), Some(&42));
    
    table = table.insert("key".to_string(), 84); // Update existing key
    assert_eq!(table.lookup(&"key".to_string()), Some(&84));
    assert_eq!(table.size(), 1); // Size should remain 1
}

#[test]
fn test_nested_hash_table_integer_keys() {
    let mut table = NestedHashTable::create_table(10);
    
    table = table.insert(10, "ten".to_string());
    table = table.insert(20, "twenty".to_string());
    table = table.insert(30, "thirty".to_string());
    
    assert_eq!(table.lookup(&10), Some(&"ten".to_string()));
    assert_eq!(table.lookup(&20), Some(&"twenty".to_string()));
    assert_eq!(table.lookup(&30), Some(&"thirty".to_string()));
    
    let (table, _deleted) = table.delete(&20);
    assert_eq!(table.lookup(&20), None);
    assert_eq!(table.size(), 2);
}

#[test]
fn test_nested_hash_table_collision_handling() {
    // Use a small table to force collisions
    let mut table = NestedHashTable::create_table(3);
    
    // These strings will likely hash to the same bucket, testing nested structure
    table = table.insert("a".to_string(), 1);
    table = table.insert("d".to_string(), 2); // Might hash to same bucket as "a"
    table = table.insert("g".to_string(), 3); // Might hash to same bucket as "a" and "d"
    table = table.insert("j".to_string(), 4); // Might hash to same bucket again
    
    // All should be retrievable despite collisions
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"d".to_string()), Some(&2));
    assert_eq!(table.lookup(&"g".to_string()), Some(&3));
    assert_eq!(table.lookup(&"j".to_string()), Some(&4));
    
    assert_eq!(table.size(), 4);
}

#[test]
fn test_nested_hash_table_resize_behavior() {
    let mut table = NestedHashTable::create_table(4);
    
    // Insert many elements to trigger resizing
    for i in 0..20 {
    let key = format!("item{}", i);
    table = table.insert(key, i as i32);
    }
    
    assert_eq!(table.size(), 20);
    
    // Verify all elements are still accessible after resizing
    for i in 0..20 {
    let key = format!("item{}", i);
    assert_eq!(table.lookup(&key), Some(&(i as i32)));
    }
}

#[test]
fn test_nested_hash_table_empty_operations() {
    let table: NestedHashTable<String, i32> = NestedHashTable::create_table(10);
    
    assert!(table.is_empty());
    assert_eq!(table.size(), 0);
    assert_eq!(table.lookup(&"nonexistent".to_string()), None);
    
    // Delete from empty table should not crash
    let (table, _deleted) = table.delete(&"nonexistent".to_string());
    assert!(table.is_empty());
}

#[test]
fn test_nested_hash_table_load_factor_management() {
    let mut table = NestedHashTable::create_table(4);
    
    // Fill table beyond load factor threshold
    table = table.insert("a".to_string(), 1);
    table = table.insert("b".to_string(), 2);
    table = table.insert("c".to_string(), 3);
    table = table.insert("d".to_string(), 4);
    table = table.insert("e".to_string(), 5); // This should trigger resize
    
    // Table should handle load factor gracefully
    assert_eq!(table.size(), 5);
    
    // Verify all elements are accessible
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"b".to_string()), Some(&2));
    assert_eq!(table.lookup(&"c".to_string()), Some(&3));
    assert_eq!(table.lookup(&"d".to_string()), Some(&4));
    assert_eq!(table.lookup(&"e".to_string()), Some(&5));
}

#[test]
fn test_nested_hash_table_with_deletions_and_reinsertions() {
    let mut table = NestedHashTable::create_table(6);
    
    // Insert elements
    table = table.insert("p".to_string(), 10);
    table = table.insert("q".to_string(), 20);
    table = table.insert("r".to_string(), 30);
    table = table.insert("s".to_string(), 40);
    
    // Delete middle elements
    let (table, _deleted) = table.delete(&"q".to_string());
    let (table, _deleted) = table.delete(&"r".to_string());
    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&"q".to_string()), None);
    assert_eq!(table.lookup(&"r".to_string()), None);
    
    // Reinsert with different values
    let table = table.insert("q".to_string(), 99);
    let table = table.insert("r".to_string(), 88);
    assert_eq!(table.size(), 4);
    assert_eq!(table.lookup(&"q".to_string()), Some(&99));
    assert_eq!(table.lookup(&"r".to_string()), Some(&88));
    
    // Other elements should remain intact
    assert_eq!(table.lookup(&"p".to_string()), Some(&10));
    assert_eq!(table.lookup(&"s".to_string()), Some(&40));
}

#[test]
fn test_nested_hash_table_nested_structure() {
    // Use a very small table to force nesting
    let mut table = NestedHashTable::create_table(2);
    
    // Insert many elements that will collide and create nested structure
    for i in 0..12 {
    let key = format!("key{}", i);
    table = table.insert(key, i as i32);
    }
    
    assert_eq!(table.size(), 12);
    
    // Verify all elements are accessible despite nested structure
    for i in 0..12 {
    let key = format!("key{}", i);
    assert_eq!(table.lookup(&key), Some(&(i as i32)));
    }
}

#[test]
fn test_nested_hash_table_bucket_distribution() {
    let mut table = NestedHashTable::create_table(7);
    
    // Insert keys that should distribute across buckets
    let keys = ["apple", "banana", "cherry", "date", "elderberry", "fig", "grape"];
    
    for (i, &key) in keys.iter().enumerate() {
    table = table.insert(key.to_string(), i as i32);
    }
    
    assert_eq!(table.size(), keys.len());
    
    // Verify all keys are retrievable
    for (i, &key) in keys.iter().enumerate() {
    assert_eq!(table.lookup(&key.to_string()), Some(&(i as i32)));
    }
}

#[test]
fn test_nested_hash_table_delete_from_nested_chain() {
    let mut table = NestedHashTable::create_table(3);
    
    // Insert multiple items that likely hash to the same bucket
    table = table.insert("a".to_string(), 1);
    table = table.insert("d".to_string(), 2);
    table = table.insert("g".to_string(), 3);
    table = table.insert("j".to_string(), 4);
    
    // Delete from the middle of a nested chain
    let (table, _deleted) = table.delete(&"d".to_string());
    assert_eq!(table.lookup(&"d".to_string()), None);
    assert_eq!(table.size(), 3);
    
    // Other elements in the chain should still be accessible
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"g".to_string()), Some(&3));
    assert_eq!(table.lookup(&"j".to_string()), Some(&4));
    
    // Delete from the head of a chain
    let (table, _deleted) = table.delete(&"a".to_string());
    assert_eq!(table.lookup(&"a".to_string()), None);
    assert_eq!(table.size(), 2);
    
    // Remaining elements should still be accessible
    assert_eq!(table.lookup(&"g".to_string()), Some(&3));
    assert_eq!(table.lookup(&"j".to_string()), Some(&4));
}

#[test]
fn test_nested_hash_table_update_in_nested_chain() {
    let mut table = NestedHashTable::create_table(3);
    
    // Insert multiple items that likely hash to the same bucket
    table = table.insert("x".to_string(), 10);
    table = table.insert("y".to_string(), 20);
    table = table.insert("z".to_string(), 30);
    
    // Update an item in the nested chain
    table = table.insert("y".to_string(), 200); // Update existing key
    assert_eq!(table.lookup(&"y".to_string()), Some(&200));
    assert_eq!(table.size(), 3); // Size should remain the same
    
    // Other elements should be unaffected
    assert_eq!(table.lookup(&"x".to_string()), Some(&10));
    assert_eq!(table.lookup(&"z".to_string()), Some(&30));
}

#[test]
fn test_nested_hash_table_performance_with_good_distribution() {
    let mut table = NestedHashTable::create_table(11); // Prime number for better distribution
    
    // Insert a variety of keys
    let keys = [
    "alpha", "bravo", "charlie", "delta", "echo", "foxtrot",
    "golf", "hotel", "india", "juliet", "kilo", "lima",
    "mike", "november", "oscar", "papa", "quebec", "romeo"
    ];
    
    for (i, &key) in keys.iter().enumerate() {
    table = table.insert(key.to_string(), i as i32);
    }
    
    assert_eq!(table.size(), keys.len());
    
    // Verify all keys are retrievable
    for (i, &key) in keys.iter().enumerate() {
    assert_eq!(table.lookup(&key.to_string()), Some(&(i as i32)));
    }
}

#[test]
fn test_nested_hash_table_stress_test() {
    let mut table = NestedHashTable::create_table(8);
    
    // Insert many elements to test robustness
    for i in 0..100 {
    let key = format!("stress{:03}", i);
    table = table.insert(key, i as i32);
    }
    
    assert_eq!(table.size(), 100);
    
    // Verify all elements are accessible
    for i in 0..100 {
    let key = format!("stress{:03}", i);
    assert_eq!(table.lookup(&key), Some(&(i as i32)));
    }
    
    // Delete some elements
    for i in (0..100).step_by(3) {
    let key = format!("stress{:03}", i);
    let (new_table, _deleted) = table.delete(&key);
    table = new_table;
    }
    
    // Verify correct elements were deleted
    for i in 0..100 {
    let key = format!("stress{:03}", i);
    if i % 3 == 0 {
        assert_eq!(table.lookup(&key), None);
    } else {
        assert_eq!(table.lookup(&key), Some(&(i as i32)));
    }
    }
}

#[test]
fn test_nested_hash_table_mixed_operations() {
    let mut table = NestedHashTable::create_table(5);
    
    // Mixed sequence of operations
    table = table.insert("first".to_string(), 1);
    table = table.insert("second".to_string(), 2);
    assert_eq!(table.lookup(&"first".to_string()), Some(&1));
    
    let (table, _deleted) = table.delete(&"first".to_string());
    assert_eq!(table.lookup(&"first".to_string()), None);
    assert_eq!(table.size(), 1);
    
    let table = table.insert("third".to_string(), 3);
    let table = table.insert("first".to_string(), 10); // Reinsert with new value
    assert_eq!(table.size(), 3);
    
    let table = table.insert("second".to_string(), 20); // Update existing
    assert_eq!(table.lookup(&"second".to_string()), Some(&20));
    assert_eq!(table.size(), 3); // Size unchanged
    
    // Final verification
    assert_eq!(table.lookup(&"first".to_string()), Some(&10));
    assert_eq!(table.lookup(&"second".to_string()), Some(&20));
    assert_eq!(table.lookup(&"third".to_string()), Some(&3));
}

#[test]
fn test_nested_hash_table_edge_cases() {
    // Test with minimum table size
    let mut table = NestedHashTable::create_table(1);
    
    // All elements will hash to the same bucket
    table = table.insert("a".to_string(), 1);
    table = table.insert("b".to_string(), 2);
    table = table.insert("c".to_string(), 3);
    
    assert_eq!(table.size(), 3);
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"b".to_string()), Some(&2));
    assert_eq!(table.lookup(&"c".to_string()), Some(&3));
    
    // Test with empty string key
    table = table.insert("".to_string(), 0);
    assert_eq!(table.lookup(&"".to_string()), Some(&0));
    assert_eq!(table.size(), 4);
    
    // Delete empty string key
    let (table, _deleted) = table.delete(&"".to_string());
    assert_eq!(table.lookup(&"".to_string()), None);
    assert_eq!(table.size(), 3);
}

#[test]
fn test_nested_hash_table_integer_hash_behavior() {
    let mut table = NestedHashTable::create_table(7);
    
    // Insert integer keys that might have interesting hash properties
    let test_data = [
    (0, "zero".to_string()),
    (1, "one".to_string()),
    (7, "seven".to_string()),  // Same as table size
    (14, "fourteen".to_string()), // Multiple of table size
    (100, "hundred".to_string()),
    (-5, "negative".to_string()),
    ];
    
    for &(key, ref value) in &test_data {
    table = table.insert(key, value.clone());
    }
    
    assert_eq!(table.size(), test_data.len());
    
    // Verify all entries
    for &(key, ref value) in &test_data {
    assert_eq!(table.lookup(&key), Some(value));
    }
}

#[test]
fn test_nested_hash_table_display_functionality() {
    let mut table = NestedHashTable::create_table(5);
    
    table = table.insert("alpha".to_string(), 1);
    table = table.insert("beta".to_string(), 2);
    table = table.insert("gamma".to_string(), 3);
    
    // Test that display functionality works (doesn't crash)
    let display_str = format!("{}", table);
    assert!(display_str.contains("NestedHashTable"));
    assert!(display_str.contains("size: 3"));
}

#[test]
fn test_nested_hash_table_debug_functionality() {
    let table: NestedHashTable<String, i32> = NestedHashTable::create_table(5);
    
    // Test that debug functionality works (doesn't crash)
    let debug_str = format!("{:?}", table);
    assert!(debug_str.contains("NestedHashTable"));
    assert!(debug_str.contains("num_elements: 0"));
}

#[test]
fn test_nested_hash_table_clone_functionality() {
    let mut table = NestedHashTable::create_table(5);
    table = table.insert("test".to_string(), 42);
    
    let cloned_table = table.clone();
    
    assert_eq!(table.size(), cloned_table.size());
    assert_eq!(table.lookup(&"test".to_string()), cloned_table.lookup(&"test".to_string()));
    
    // Verify independence - modify original
    let modified_table = table.insert("new".to_string(), 99);
    assert_eq!(modified_table.size(), 2);
    assert_eq!(cloned_table.size(), 1); // Clone should be unchanged
}

#[test]
fn test_nested_hash_table_large_scale_operations() {
    let mut table = NestedHashTable::create_table(16);

    // Insert a smaller number of elements for reasonable test time
    for i in 0..100 {
    let key = format!("large{:04}", i);
    table = table.insert(key, i as i32);
    }

    assert_eq!(table.size(), 100);

    // Spot check some elements
    assert_eq!(table.lookup(&"large0000".to_string()), Some(&0));
    assert_eq!(table.lookup(&"large0050".to_string()), Some(&50));
    assert_eq!(table.lookup(&"large0099".to_string()), Some(&99));

    // Delete every 10th element
    for i in (0..100).step_by(10) {
    let key = format!("large{:04}", i);
    let (new_table, _deleted) = table.delete(&key);
    table = new_table;
    }

    assert_eq!(table.size(), 90); // 100 - 10 deletions

    // Verify deletions
    assert_eq!(table.lookup(&"large0000".to_string()), None);
    assert_eq!(table.lookup(&"large0010".to_string()), None);
    assert_eq!(table.lookup(&"large0001".to_string()), Some(&1)); // Should still exist
}
