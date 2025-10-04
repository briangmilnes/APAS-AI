//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 47: QuadraticProbing - Hash Table with Quadratic Probing

use apas_ai::Chap47::QuadraticProbing::QuadraticProbing::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Types::Types::*;

#[test]
fn test_create_quadratic_probing_string_table() {
    let table = create_quadratic_probing_string_table::<i32>(10);
    assert_eq!(table.load_and_size().1, 0);
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_create_quadratic_probing_integer_table() {
    let table = create_quadratic_probing_integer_table::<String>(10, 54321);
    assert_eq!(table.load_and_size().1, 0);
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_quadratic_probing_insert_and_lookup() {
    let mut table = create_quadratic_probing_string_table::<i32>(10);
    
    // Insert some key-value pairs
    table = table.insert("alpha".to_string(), 1);
    table = table.insert("beta".to_string(), 2);
    table = table.insert("gamma".to_string(), 3);
    table = table.insert("delta".to_string(), 4);
    
    assert_eq!(table.load_and_size().1, 4);
    assert!(!table.load_and_size().0 == 0);
    
    // Test lookups
    assert_eq!(table.lookup(&"alpha".to_string()), Some(&1));
    assert_eq!(table.lookup(&"beta".to_string()), Some(&2));
    assert_eq!(table.lookup(&"gamma".to_string()), Some(&3));
    assert_eq!(table.lookup(&"delta".to_string()), Some(&4));
    assert_eq!(table.lookup(&"epsilon".to_string()), None);
}

#[test]
fn test_quadratic_probing_delete() {
    let mut table = create_quadratic_probing_string_table::<i32>(10);
    
    // Insert and then delete
    table = table.insert("first".to_string(), 100);
    table = table.insert("second".to_string(), 200);
    table = table.insert("third".to_string(), 300);
    
    assert_eq!(table.load_and_size().1, 3);
    assert_eq!(table.lookup(&"second".to_string()), Some(&200));
    
    let (table, _deleted) = table.delete(&"second".to_string());
    
    assert_eq!(table.load_and_size().1, 2);
    assert_eq!(table.lookup(&"second".to_string()), None);
    assert_eq!(table.lookup(&"first".to_string()), Some(&100));
    assert_eq!(table.lookup(&"third".to_string()), Some(&300));
}

#[test]
fn test_quadratic_probing_update() {
    let mut table = create_quadratic_probing_string_table::<i32>(10);
    
    table = table.insert("key".to_string(), 42);
    assert_eq!(table.lookup(&"key".to_string()), Some(&42));
    
    table = table.insert("key".to_string(), 84); // Update existing key
    assert_eq!(table.lookup(&"key".to_string()), Some(&84));
    assert_eq!(table.load_and_size().1, 1); // Size should remain 1
}

#[test]
fn test_quadratic_probing_integer_operations() {
    let mut table = create_quadratic_probing_integer_table::<String>(10, 98765);
    
    table = table.insert(10, "ten".to_string());
    table = table.insert(20, "twenty".to_string());
    table = table.insert(30, "thirty".to_string());
    
    assert_eq!(table.lookup(&10), Some(&"ten".to_string()));
    assert_eq!(table.lookup(&20), Some(&"twenty".to_string()));
    assert_eq!(table.lookup(&30), Some(&"thirty".to_string()));
    
    let (table, _deleted) = table.delete(&20);
    assert_eq!(table.lookup(&20), None);
    assert_eq!(table.load_and_size().1, 2);
}

#[test]
fn test_quadratic_probing_strategy_creation() {
    let strategy = QuadraticProbingStrategy::new(StringPositionHashFunction, 1, 3);
    assert!(strategy.strategy_name().contains("QuadraticProbing"));
    assert!(strategy.strategy_name().contains("c1=1"));
    assert!(strategy.strategy_name().contains("c2=3"));
}

#[test]
fn test_quadratic_probing_standard_strategy() {
    let strategy = QuadraticProbingStrategy::standard(StringPositionHashFunction);
    assert!(strategy.strategy_name().contains("c1=1"));
    assert!(strategy.strategy_name().contains("c2=1"));
}

#[test]
fn test_quadratic_probe_sequence() {
    let strategy = QuadraticProbingStrategy::new(StringPositionHashFunction, 1, 1);
    let key = "test".to_string();
    let table_size = 10;
    
    let hash0 = strategy.probe_hash(&key, 0, table_size);
    let hash1 = strategy.probe_hash(&key, 1, table_size);
    let hash2 = strategy.probe_hash(&key, 2, table_size);
    let hash3 = strategy.probe_hash(&key, 3, table_size);
    
    // Quadratic probing: h(k) + c1*i + c2*i^2
    // With c1=1, c2=1: base, base+1+1, base+2+4, base+3+9
    let base = StringPositionHashFunction.hash(&key, table_size);
    assert_eq!(hash0, base % table_size);
    assert_eq!(hash1, (base + 1 + 1) % table_size);
    assert_eq!(hash2, (base + 2 + 4) % table_size);
    assert_eq!(hash3, (base + 3 + 9) % table_size);
}

#[test]
fn test_quadratic_probing_collision_handling() {
    // Use a small table to force collisions
    let mut table = create_quadratic_probing_string_table::<i32>(5);
    
    // These strings might hash to the same slot, testing quadratic probing
    table = table.insert("x".to_string(), 1);
    table = table.insert("y".to_string(), 2);
    table = table.insert("z".to_string(), 3);
    table = table.insert("w".to_string(), 4);
    
    // All should be retrievable despite potential collisions
    assert_eq!(table.lookup(&"x".to_string()), Some(&1));
    assert_eq!(table.lookup(&"y".to_string()), Some(&2));
    assert_eq!(table.lookup(&"z".to_string()), Some(&3));
    assert_eq!(table.lookup(&"w".to_string()), Some(&4));
    
    assert_eq!(table.load_and_size().1, 4);
}

#[test]
fn test_quadratic_probing_resize_behavior() {
    let mut table = create_quadratic_probing_string_table::<i32>(7);
    
    // Insert many elements to trigger resizing
    for i in 0..25 {
    let key = format!("item{}", i);
    table = table.insert(key, i as i32);
    }
    
    assert_eq!(table.load_and_size().1, 25);
    
    // Verify all elements are still accessible after resizing
    for i in 0..25 {
    let key = format!("item{}", i);
    assert_eq!(table.lookup(&key), Some(&(i as i32)));
    }
}

#[test]
fn test_quadratic_probing_empty_operations() {
    let table = create_quadratic_probing_string_table::<i32>(10);
    
    assert!(table.load_and_size().0 == 0);
    assert_eq!(table.load_and_size().1, 0);
    assert_eq!(table.lookup(&"nonexistent".to_string()), None);
    
    // Delete from empty table should not crash
    let (table, _deleted) = table.delete(&"nonexistent".to_string());
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_quadratic_probing_different_coefficients() {
    // Test with different c1, c2 values
    let strategy1 = QuadraticProbingStrategy::new(StringPositionHashFunction, 2, 3);
    let strategy2 = QuadraticProbingStrategy::new(StringPositionHashFunction, 5, 7);
    
    let key = "probe".to_string();
    let table_size = 11;
    
    // Different strategies should produce different probe sequences
    let seq1_0 = strategy1.probe_hash(&key, 0, table_size);
    let seq1_1 = strategy1.probe_hash(&key, 1, table_size);
    let seq2_0 = strategy2.probe_hash(&key, 0, table_size);
    let seq2_1 = strategy2.probe_hash(&key, 1, table_size);
    
    // Base hash should be same
    assert_eq!(seq1_0, seq2_0);
    
    // First probe should differ due to different coefficients
    // strategy1: base + 2*1 + 3*1 = base + 5
    // strategy2: base + 5*1 + 7*1 = base + 12
    assert_ne!(seq1_1, seq2_1);
}

#[test]
fn test_quadratic_probing_load_factor_management() {
    let mut table = create_quadratic_probing_string_table::<i32>(6);
    
    // Fill table to test load factor management
    table = table.insert("a".to_string(), 1);
    table = table.insert("b".to_string(), 2);
    table = table.insert("c".to_string(), 3);
    table = table.insert("d".to_string(), 4);
    table = table.insert("e".to_string(), 5);
    
    // Table should handle moderate load factor gracefully
    assert_eq!(table.load_and_size().1, 5);
    
    // Verify all elements are accessible
    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"b".to_string()), Some(&2));
    assert_eq!(table.lookup(&"c".to_string()), Some(&3));
    assert_eq!(table.lookup(&"d".to_string()), Some(&4));
    assert_eq!(table.lookup(&"e".to_string()), Some(&5));
}

#[test]
fn test_quadratic_probing_with_deletions_and_reinsertions() {
    let mut table = create_quadratic_probing_string_table::<i32>(8);
    
    // Insert elements
    table = table.insert("p".to_string(), 10);
    table = table.insert("q".to_string(), 20);
    table = table.insert("r".to_string(), 30);
    table = table.insert("s".to_string(), 40);
    
    // Delete middle elements
    let (table, _deleted) = table.delete(&"q".to_string());
    let (mut table, _deleted) = table.delete(&"r".to_string());
    assert_eq!(table.load_and_size().1, 2);
    assert_eq!(table.lookup(&"q".to_string()), None);
    assert_eq!(table.lookup(&"r".to_string()), None);
    
    // Reinsert with different values
    table = table.insert("q".to_string(), 99);
    table = table.insert("r".to_string(), 88);
    assert_eq!(table.load_and_size().1, 4);
    assert_eq!(table.lookup(&"q".to_string()), Some(&99));
    assert_eq!(table.lookup(&"r".to_string()), Some(&88));
    
    // Other elements should remain intact
    assert_eq!(table.lookup(&"p".to_string()), Some(&10));
    assert_eq!(table.lookup(&"s".to_string()), Some(&40));
}

#[test]
fn test_quadratic_probe_mathematical_properties() {
    let strategy = QuadraticProbingStrategy::new(StringPositionHashFunction, 3, 2);
    let key = "math".to_string();
    let table_size = 13;
    
    // Generate probe sequence
    let mut positions = Vec::new();
    for i in 0..6 {
    positions.push(strategy.probe_hash(&key, i, table_size));
    }
    
    // Verify quadratic progression: h(k) + 3i + 2i^2
    let base = StringPositionHashFunction.hash(&key, table_size);
    for (i, &pos) in positions.iter().enumerate() {
    let expected = (base + 3 * i + 2 * i * i) % table_size;
    assert_eq!(pos, expected);
    }
}

#[test]
fn test_quadratic_probing_prime_table_sizes() {
    // Prime table sizes can help with quadratic probing distribution
    let mut table = create_quadratic_probing_string_table::<i32>(13); // Prime
    
    // Insert elements
    for i in 0..8 {
    let key = format!("prime{}", i);
    table = table.insert(key, i as i32 * 10);
    }
    
    assert_eq!(table.load_and_size().1, 8);
    
    // Verify retrieval
    for i in 0..8 {
    let key = format!("prime{}", i);
    assert_eq!(table.lookup(&key), Some(&(i as i32 * 10)));
    }
}
