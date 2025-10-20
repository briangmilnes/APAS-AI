//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 47: LinearProbing - Hash Table with Linear Probing

use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Chap47::LinearProbing::LinearProbing::*;
use apas_ai::Types::Types::*;

#[test]
fn test_create_linear_probing_string_table() {
    let table = create_linear_probing_string_table::<i32>(10);
    assert_eq!(table.load_and_size().0, 0);
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_create_linear_probing_integer_table() {
    let table = create_linear_probing_integer_table::<String>(10, 12345);
    assert_eq!(table.load_and_size().0, 0);
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_linear_probing_insert_and_lookup() {
    let mut table = create_linear_probing_string_table::<i32>(10);

    // Insert some key-value pairs
    table = table.insert("apple".to_string(), 1);
    table = table.insert("banana".to_string(), 2);
    table = table.insert("cherry".to_string(), 3);

    assert_eq!(table.load_and_size().0, 3);
    assert!(table.load_and_size().0 != 0);

    // Test lookups
    assert_eq!(table.lookup(&"apple".to_string()), Some(&1));
    assert_eq!(table.lookup(&"banana".to_string()), Some(&2));
    assert_eq!(table.lookup(&"cherry".to_string()), Some(&3));
    assert_eq!(table.lookup(&"grape".to_string()), None);
}

#[test]
fn test_linear_probing_delete() {
    let mut table = create_linear_probing_string_table::<i32>(10);

    // Insert and then delete
    table = table.insert("key1".to_string(), 100);
    table = table.insert("key2".to_string(), 200);

    assert_eq!(table.load_and_size().0, 2);
    assert_eq!(table.lookup(&"key1".to_string()), Some(&100));

    let (table, _deleted) = table.delete(&"key1".to_string());

    assert_eq!(table.load_and_size().0, 1);
    assert_eq!(table.lookup(&"key1".to_string()), None);
    assert_eq!(table.lookup(&"key2".to_string()), Some(&200));
}

#[test]
fn test_linear_probing_update() {
    let mut table = create_linear_probing_string_table::<i32>(10);

    table = table.insert("key".to_string(), 42);
    assert_eq!(table.lookup(&"key".to_string()), Some(&42));

    table = table.insert("key".to_string(), 84); // Update existing key
    assert_eq!(table.lookup(&"key".to_string()), Some(&84));
    assert_eq!(table.load_and_size().0, 1); // Size should remain 1
}

#[test]
fn test_linear_probing_integer_operations() {
    let mut table = create_linear_probing_integer_table::<String>(10, 54321);

    table = table.insert(1, "one".to_string());
    table = table.insert(2, "two".to_string());
    table = table.insert(3, "three".to_string());

    assert_eq!(table.lookup(&1), Some(&"one".to_string()));
    assert_eq!(table.lookup(&2), Some(&"two".to_string()));
    assert_eq!(table.lookup(&3), Some(&"three".to_string()));

    let (table, _deleted) = table.delete(&2);
    assert_eq!(table.lookup(&2), None);
    assert_eq!(table.load_and_size().0, 2);
}

#[test]
fn test_linear_probing_collision_handling() {
    // Use a small table to force collisions
    let mut table = create_linear_probing_string_table::<i32>(3);

    // These strings might hash to the same slot, testing linear probing
    table = table.insert("a".to_string(), 1);
    table = table.insert("b".to_string(), 2);
    table = table.insert("c".to_string(), 3);

    // All should be retrievable despite potential collisions
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"b".to_string()), Some(&2));
    assert_eq!(table.lookup(&"c".to_string()), Some(&3));

    assert_eq!(table.load_and_size().0, 3);
}

#[test]
fn test_linear_probing_resize_behavior() {
    let mut table = create_linear_probing_string_table::<i32>(5);

    // Insert many elements to trigger resizing
    for i in 0..20 {
        let key = format!("key{i}");
        table = table.insert(key, i);
    }

    assert_eq!(table.load_and_size().0, 20);

    // Verify all elements are still accessible after resizing
    for i in 0..20 {
        let key = format!("key{i}");
        assert_eq!(table.lookup(&key), Some(&{ i }));
    }
}

#[test]
fn test_linear_probing_empty_operations() {
    let table = create_linear_probing_string_table::<i32>(10);

    assert!(table.load_and_size().0 == 0);
    assert_eq!(table.load_and_size().0, 0);
    assert_eq!(table.lookup(&"nonexistent".to_string()), None);

    // Delete from empty table should not crash
    let (table, _deleted) = table.delete(&"nonexistent".to_string());
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_linear_probing_strategy_creation() {
    let strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    assert!(strategy.strategy_name().contains("LinearProbing"));

    // Test probe sequence generation
    let key = "test".to_string();
    let table_size = 10;

    let hash0 = strategy.probe_hash(&key, 0, table_size);
    let hash1 = strategy.probe_hash(&key, 1, table_size);
    let hash2 = strategy.probe_hash(&key, 2, table_size);

    // Linear probing should produce consecutive slots (modulo table size)
    assert_eq!(hash1, (hash0 + 1) % table_size);
    assert_eq!(hash2, (hash0 + 2) % table_size);
}

#[test]
fn test_linear_probing_load_factor_management() {
    let mut table = create_linear_probing_string_table::<i32>(4);

    // Fill table to trigger load factor management
    table = table.insert("a".to_string(), 1);
    table = table.insert("b".to_string(), 2);
    table = table.insert("c".to_string(), 3);
    table = table.insert("d".to_string(), 4);

    // Table should handle high load factor gracefully
    assert_eq!(table.load_and_size().0, 4);

    // Verify all elements are accessible
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"b".to_string()), Some(&2));
    assert_eq!(table.lookup(&"c".to_string()), Some(&3));
    assert_eq!(table.lookup(&"d".to_string()), Some(&4));
}

#[test]
fn test_linear_probing_pattern_verification() {
    let strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let key = "pattern".to_string();
    let table_size = 7;

    // Generate a sequence of probe positions
    let mut positions = Vec::new();
    for i in 0..table_size {
        positions.push(strategy.probe_hash(&key, i, table_size));
    }

    // Verify linear progression
    let base = positions[0];
    for (i, &pos) in positions.iter().enumerate().skip(1).take(table_size - 1) {
        assert_eq!(pos, (base + i) % table_size);
    }
}

#[test]
fn test_linear_probing_with_deletions_and_reinsertions() {
    let mut table = create_linear_probing_string_table::<i32>(8);

    // Insert elements
    table = table.insert("x".to_string(), 10);
    table = table.insert("y".to_string(), 20);
    table = table.insert("z".to_string(), 30);

    // Delete middle element
    let (mut table, _deleted) = table.delete(&"y".to_string());
    assert_eq!(table.load_and_size().0, 2);
    assert_eq!(table.lookup(&"y".to_string()), None);

    // Reinsert with different value
    table = table.insert("y".to_string(), 99);
    assert_eq!(table.load_and_size().0, 3);
    assert_eq!(table.lookup(&"y".to_string()), Some(&99));

    // Other elements should remain intact
    assert_eq!(table.lookup(&"x".to_string()), Some(&10));
    assert_eq!(table.lookup(&"z".to_string()), Some(&30));
}
