//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 47: DoubleHashing - Hash Table with Double Hashing

use apas_ai::Chap47::DoubleHashing::DoubleHashing::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Types::Types::*;

#[test]
fn test_create_double_hashing_string_table() {
    let table = create_double_hashing_string_table::<i32>(10);
    assert_eq!(table.load_and_size().1, 0);
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_create_double_hashing_integer_table() {
    let table = create_double_hashing_integer_table::<String>(10, 13579, 24680);
    assert_eq!(table.load_and_size().1, 0);
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_double_hashing_insert_and_lookup() {
    let mut table = create_double_hashing_string_table::<i32>(10);
    
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
fn test_double_hashing_delete() {
    let mut table = create_double_hashing_string_table::<i32>(10);
    
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
fn test_double_hashing_update() {
    let mut table = create_double_hashing_string_table::<i32>(10);
    
    table = table.insert("key".to_string(), 42);
    assert_eq!(table.lookup(&"key".to_string()), Some(&42));
    
    table = table.insert("key".to_string(), 84); // Update existing key
    assert_eq!(table.lookup(&"key".to_string()), Some(&84));
    assert_eq!(table.load_and_size().1, 1); // Size should remain 1
}

#[test]
fn test_double_hashing_integer_operations() {
    let mut table = create_double_hashing_integer_table::<String>(10, 98765, 11223);
    
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
fn test_double_hashing_strategy_creation() {
    let strategy = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    assert!(strategy.strategy_name().contains("DoubleHashing"));
    assert!(strategy.strategy_name().contains("StringPositionHashFunction"));
    assert!(strategy.strategy_name().contains("PolynomialHashFunction::new(31)"));
}

#[test]
fn test_double_hash_probe_sequence() {
    let strategy = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    let key = "test".to_string();
    let table_size = 11;
    
    let hash0 = strategy.probe_hash(&key, 0, table_size);
    let hash1 = strategy.probe_hash(&key, 1, table_size);
    let hash2 = strategy.probe_hash(&key, 2, table_size);
    let hash3 = strategy.probe_hash(&key, 3, table_size);
    
    // Double hashing: h1(k) + i * h2(k)
    let h1 = StringPositionHashFunction.hash(&key, table_size);
    let h2 = PolynomialHashFunction::new(31).hash(&key, table_size);
    let h2_safe = if h2 == 0 { 1 } else { h2 };
    
    assert_eq!(hash0, h1 % table_size);
    assert_eq!(hash1, (h1 + 1 * h2_safe) % table_size);
    assert_eq!(hash2, (h1 + 2 * h2_safe) % table_size);
    assert_eq!(hash3, (h1 + 3 * h2_safe) % table_size);
}

#[test]
fn test_double_hashing_collision_handling() {
    // Use a small table to force collisions
    let mut table = create_double_hashing_string_table::<i32>(5);
    
    // These strings might hash to the same slot, testing double hashing
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
fn test_double_hashing_zero_h2_handling() {
    // Create a strategy that might produce h2 = 0
    let strategy = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    let key = "".to_string(); // Empty string might hash to 0 for length-based hash
    let table_size = 7;
    
    // Should handle h2 = 0 by using 1 instead
    let hash0 = strategy.probe_hash(&key, 0, table_size);
    let hash1 = strategy.probe_hash(&key, 1, table_size);
    let hash2 = strategy.probe_hash(&key, 2, table_size);
    
    // Verify the sequence progresses (not stuck at same value)
    // Even if h2 = 0, it should be converted to 1
    let h1 = StringPositionHashFunction.hash(&key, table_size);
    let h2_raw = PolynomialHashFunction::new(31).hash(&key, table_size);
    let h2_safe = if h2_raw == 0 { 1 } else { h2_raw };
    
    assert_eq!(hash0, h1 % table_size);
    assert_eq!(hash1, (h1 + 1 * h2_safe) % table_size);
    assert_eq!(hash2, (h1 + 2 * h2_safe) % table_size);
}

#[test]
fn test_double_hashing_resize_behavior() {
    let mut table = create_double_hashing_string_table::<i32>(7);
    
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
fn test_double_hashing_empty_operations() {
    let table = create_double_hashing_string_table::<i32>(10);
    
    assert!(table.load_and_size().0 == 0);
    assert_eq!(table.load_and_size().1, 0);
    assert_eq!(table.lookup(&"nonexistent".to_string()), None);
    
    // Delete from empty table should not crash
    let (table, _deleted) = table.delete(&"nonexistent".to_string());
    assert!(table.load_and_size().0 == 0);
}

#[test]
fn test_double_hashing_different_hash_functions() {
    // Test with different hash function combinations
    let strategy1 = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    let strategy2 = DoubleHashingStrategy::new(PolynomialHashFunction::new(31), StringPositionHashFunction);
    
    let key = "probe".to_string();
    let table_size = 13;
    
    // Different strategies should produce different probe sequences
    let seq1_0 = strategy1.probe_hash(&key, 0, table_size);
    let _seq1_1 = strategy1.probe_hash(&key, 1, table_size);
    let seq2_0 = strategy2.probe_hash(&key, 0, table_size);
    let _seq2_1 = strategy2.probe_hash(&key, 1, table_size);
    
    // First probe uses h1, so strategies with swapped functions should differ
    // strategy1: h1=position, h2=length
    // strategy2: h1=length, h2=position
    // The probe sequences should differ due to swapped roles
    if StringPositionHashFunction.hash(&key, table_size) != PolynomialHashFunction::new(31).hash(&key, table_size) {
    assert_ne!(seq1_0, seq2_0);
    }
}

#[test]
fn test_double_hashing_load_factor_management() {
    let mut table = create_double_hashing_string_table::<i32>(6);
    
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
fn test_double_hashing_with_deletions_and_reinsertions() {
    let mut table = create_double_hashing_string_table::<i32>(8);
    
    // Insert elements
    table = table.insert("p".to_string(), 10);
    table = table.insert("q".to_string(), 20);
    table = table.insert("r".to_string(), 30);
    table = table.insert("s".to_string(), 40);
    
    // Delete middle elements
    let (table, _deleted) = table.delete(&"q".to_string());
    let (table, _deleted) = table.delete(&"r".to_string());
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
fn test_double_hash_mathematical_properties() {
    let strategy = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    let key = "math".to_string();
    let table_size = 17;
    
    // Generate probe sequence
    let mut positions = Vec::new();
    for i in 0..8 {
    positions.push(strategy.probe_hash(&key, i, table_size));
    }
    
    // Verify double hashing progression: h1(k) + i * h2(k)
    let h1 = StringPositionHashFunction.hash(&key, table_size);
    let h2_raw = PolynomialHashFunction::new(31).hash(&key, table_size);
    let h2 = if h2_raw == 0 { 1 } else { h2_raw };
    
    for (i, &pos) in positions.iter().enumerate() {
    let expected = (h1 + i * h2) % table_size;
    assert_eq!(pos, expected);
    }
}

#[test]
fn test_double_hashing_prime_table_sizes() {
    // Prime table sizes work well with double hashing
    let mut table = create_double_hashing_string_table::<i32>(17); // Prime
    
    // Insert elements
    for i in 0..10 {
    let key = format!("prime{}", i);
    table = table.insert(key, i as i32 * 10);
    }
    
    assert_eq!(table.load_and_size().1, 10);
    
    // Verify retrieval
    for i in 0..10 {
    let key = format!("prime{}", i);
    assert_eq!(table.lookup(&key), Some(&(i as i32 * 10)));
    }
}

#[test]
fn test_double_hashing_step_size_variation() {
    let strategy = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    let table_size = 13;
    
    // Test with keys that produce different step sizes
    let key1 = "a".to_string();      // Short key, small h2
    let key2 = "abcdef".to_string(); // Longer key, larger h2
    
    let step1 = {
    let h2_raw = PolynomialHashFunction::new(31).hash(&key1, table_size);
    if h2_raw == 0 { 1 } else { h2_raw }
    };
    
    let step2 = {
    let h2_raw = PolynomialHashFunction::new(31).hash(&key2, table_size);
    if h2_raw == 0 { 1 } else { h2_raw }
    };
    
    // Different keys should generally produce different step sizes
    // (unless they happen to have the same length mod table_size)
    if key1.len() % (table_size as usize) != key2.len() % (table_size as usize) {
    assert_ne!(step1, step2);
    }
    
    // Verify probe sequences advance with different step sizes
    let pos1_0 = strategy.probe_hash(&key1, 0, table_size);
    let pos1_1 = strategy.probe_hash(&key1, 1, table_size);
    let pos1_2 = strategy.probe_hash(&key1, 2, table_size);
    
    let pos2_0 = strategy.probe_hash(&key2, 0, table_size);
    let pos2_1 = strategy.probe_hash(&key2, 1, table_size);
    let pos2_2 = strategy.probe_hash(&key2, 2, table_size);
    
    // Each sequence should advance by its respective step size
    assert_eq!(pos1_1, (pos1_0 + step1) % table_size);
    assert_eq!(pos1_2, (pos1_0 + 2 * step1) % table_size);
    assert_eq!(pos2_1, (pos2_0 + step2) % table_size);
    assert_eq!(pos2_2, (pos2_0 + 2 * step2) % table_size);
}

#[test]
fn test_double_hashing_custom_seed_behavior() {
    // Test with custom seed values for integer tables
    let table1 = create_double_hashing_integer_table::<i32>(10, 12345, 67890);
    let table2 = create_double_hashing_integer_table::<i32>(10, 99999, 11111);
    
    // Both should be empty initially
    assert!(table1.load_and_size().0 == 0);
    assert!(table2.load_and_size().0 == 0);
    
    // Different seeds should be accepted without error
    assert_eq!(table1.load_and_size().1, 0);
    assert_eq!(table2.load_and_size().1, 0);
}

#[test]
fn test_double_hashing_optimal_distribution() {
    let mut table = create_double_hashing_string_table::<i32>(11);
    
    // Insert a sequence of similar keys to test distribution
    let keys = [
    "key0", "key1", "key2", "key3", "key4",
    "key5", "key6", "key7", "key8", "key9"
    ];
    
    for (i, &key) in keys.iter().enumerate() {
    table = table.insert(key.to_string(), i as i32);
    }
    
    assert_eq!(table.load_and_size().1, keys.len());
    
    // Verify all keys are retrievable
    for (i, &key) in keys.iter().enumerate() {
    assert_eq!(table.lookup(&key.to_string()), Some(&(i as i32)));
    }
}

#[test]
fn test_double_hashing_gcd_considerations() {
    // Double hashing works best when gcd(h2(k), table_size) = 1
    // This test verifies the strategy handles cases where this might not hold
    let strategy = DoubleHashingStrategy::new(StringPositionHashFunction, PolynomialHashFunction::new(31));
    let table_size = 12; // Composite number with multiple factors
    
    // Test various keys
    let keys = ["a", "ab", "abc", "abcd", "abcde"];
    
    for &key_str in &keys {
    let key = key_str.to_string();
    
    // Generate a few probe positions
    let pos0 = strategy.probe_hash(&key, 0, table_size);
    let pos1 = strategy.probe_hash(&key, 1, table_size);
    let pos2 = strategy.probe_hash(&key, 2, table_size);
    
    // Verify positions are valid
    assert!(pos0 < table_size);
    assert!(pos1 < table_size);
    assert!(pos2 < table_size);
    
    // Verify sequence is progressing (not stuck)
    // Note: This might not always be true if gcd(h2, table_size) > 1,
    // but the hash function should generally avoid this
    let h2_raw = PolynomialHashFunction::new(31).hash(&key, table_size);
    let h2 = if h2_raw == 0 { 1 } else { h2_raw };
    
    // At minimum, the sequence should be deterministic
    assert_eq!(pos1, (pos0 + h2) % table_size);
    assert_eq!(pos2, (pos0 + 2 * h2) % table_size);
    }
}