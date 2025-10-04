//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Flat Hash Table Tests

use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Chap47::LinearProbing::LinearProbing::*;
use apas_ai::Types::Types::*;

#[test]
fn test_create_empty_flat_table() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let table: FlatHashTable<String, i32, LinearProbingStrategy<String, StringPositionHashFunction>> =
        FlatHashTable::create_table(probe_strategy, 8);

    let (load, size) = table.load_and_size();
    assert_eq!(load, 0);
    assert_eq!(size, 8);

    let stats = table.statistics();
    assert_eq!(stats.num_elements, 0);
    assert_eq!(stats.load_factor, 0.0);
}

#[test]
fn test_insert_and_lookup() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 8);

    table = table.insert("key1".to_string(), 100);
    table = table.insert("key2".to_string(), 200);
    table = table.insert("key3".to_string(), 300);

    assert_eq!(table.lookup(&"key1".to_string()), Some(&100));
    assert_eq!(table.lookup(&"key2".to_string()), Some(&200));
    assert_eq!(table.lookup(&"key3".to_string()), Some(&300));
    assert_eq!(table.lookup(&"nonexistent".to_string()), None);

    let (load, _) = table.load_and_size();
    assert_eq!(load, 3);
}

#[test]
fn test_insert_duplicate_key() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 8);

    table = table.insert("key".to_string(), 100);
    assert_eq!(table.lookup(&"key".to_string()), Some(&100));

    // Insert same key with different value
    table = table.insert("key".to_string(), 200);
    assert_eq!(table.lookup(&"key".to_string()), Some(&200));

    // Should still have only 1 element
    let (load, _) = table.load_and_size();
    assert_eq!(load, 1);
}

#[test]
fn test_delete() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 8);

    table = table.insert("key1".to_string(), 100);
    table = table.insert("key2".to_string(), 200);

    // Delete existing key
    let (new_table, was_deleted) = table.delete(&"key1".to_string());
    assert!(was_deleted);
    assert_eq!(new_table.lookup(&"key1".to_string()), None);
    assert_eq!(new_table.lookup(&"key2".to_string()), Some(&200));

    let (load, _) = new_table.load_and_size();
    assert_eq!(load, 1);

    // Check that deleted entry is marked as Dead
    let (live, dead, _) = new_table.probe_statistics();
    assert_eq!(live, 1);
    assert_eq!(dead, 1);

    // Delete non-existing key
    let (same_table, was_deleted) = new_table.delete(&"nonexistent".to_string());
    assert!(!was_deleted);
    let (load, _) = same_table.load_and_size();
    assert_eq!(load, 1);
}

#[test]
fn test_linear_probing_collision_resolution() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 5);

    // Insert keys that will collide and require probing
    // With StringPositionHashFunction and table size 5:
    // "aa" -> (0+0) % 5 = 0
    // "ff" -> (5+5) % 5 = 0 (collision, should probe to position 1)
    table = table.insert("aa".to_string(), 100);
    table = table.insert("ff".to_string(), 200);

    // Both should be retrievable despite collision
    assert_eq!(table.lookup(&"aa".to_string()), Some(&100));
    assert_eq!(table.lookup(&"ff".to_string()), Some(&200));

    let (load, _) = table.load_and_size();
    assert_eq!(load, 2);
}

#[test]
fn test_resize_on_high_load_factor() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 4);

    let initial_size = table.load_and_size().1;

    // Insert enough elements to trigger resize (load factor > 0.5 for flat tables)
    // With min size 8, need > 4 elements to exceed 0.5 threshold
    table = table.insert("key1".to_string(), 1);
    table = table.insert("key2".to_string(), 2);
    table = table.insert("key3".to_string(), 3);
    table = table.insert("key4".to_string(), 4);
    table = table.insert("key5".to_string(), 5); // This should trigger resize

    let (load, new_size) = table.load_and_size();
    assert_eq!(load, 5);
    assert!(new_size > initial_size); // Table should have grown

    // All elements should still be accessible
    assert_eq!(table.lookup(&"key1".to_string()), Some(&1));
    assert_eq!(table.lookup(&"key2".to_string()), Some(&2));
    assert_eq!(table.lookup(&"key3".to_string()), Some(&3));
}

#[test]
fn test_entry_states() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 8);

    // Initially all entries should be Empty
    let (live, dead, load_factor) = table.probe_statistics();
    assert_eq!(live, 0);
    assert_eq!(dead, 0);
    assert_eq!(load_factor, 0.0);

    // Insert an element (creates Live entry)
    table = table.insert("key".to_string(), 100);
    let (live, dead, load_factor) = table.probe_statistics();
    assert_eq!(live, 1);
    assert_eq!(dead, 0);
    assert!(load_factor > 0.0);

    // Delete the element (creates Dead entry)
    let (table, was_deleted) = table.delete(&"key".to_string());
    assert!(was_deleted);
    let (live, dead, load_factor) = table.probe_statistics();
    assert_eq!(live, 0);
    assert_eq!(dead, 1);
    assert!(load_factor > 0.0); // Dead entries still count toward load
}

#[test]
fn test_probe_sequence_with_dead_entries() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 8);

    // Insert and then delete to create a Dead entry
    table = table.insert("key1".to_string(), 100);
    let (table, _) = table.delete(&"key1".to_string());

    // Insert another key that might probe through the Dead entry
    let table = table.insert("key2".to_string(), 200);

    assert_eq!(table.lookup(&"key1".to_string()), None);
    assert_eq!(table.lookup(&"key2".to_string()), Some(&200));

    let (live, dead, _) = table.probe_statistics();
    assert_eq!(live, 1);
    // Dead count might be 0 or 1 depending on whether key2 reused the dead slot
    assert!(dead <= 1);
}

#[test]
fn test_integer_keys() {
    let hash_fn = UniversalIntegerHashFunction::new(123, 456);
    let probe_strategy = LinearProbingStrategy::new(hash_fn);
    let mut table = FlatHashTable::create_table(probe_strategy, 16);

    table = table.insert(42, "forty-two".to_string());
    table = table.insert(100, "one-hundred".to_string());
    table = table.insert(7, "seven".to_string());

    assert_eq!(table.lookup(&42), Some(&"forty-two".to_string()));
    assert_eq!(table.lookup(&100), Some(&"one-hundred".to_string()));
    assert_eq!(table.lookup(&7), Some(&"seven".to_string()));
    assert_eq!(table.lookup(&999), None);
}

#[test]
fn test_empty_table_operations() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let table: FlatHashTable<String, i32, LinearProbingStrategy<String, StringPositionHashFunction>> =
        FlatHashTable::create_table(probe_strategy, 8);

    // Lookup on empty table
    assert_eq!(table.lookup(&"any_key".to_string()), None);

    // Delete on empty table
    let (same_table, was_deleted) = table.delete(&"any_key".to_string());
    assert!(!was_deleted);
    assert_eq!(same_table.load_and_size().0, 0);

    // Statistics on empty table
    let stats = table.statistics();
    assert_eq!(stats.num_elements, 0);
    assert_eq!(stats.load_factor, 0.0);
}

#[test]
fn test_full_table_behavior() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 4);

    // Fill the table completely
    table = table.insert("a".to_string(), 1);
    table = table.insert("b".to_string(), 2);

    // Table should resize before becoming completely full
    let (load, size) = table.load_and_size();
    assert!(load < size); // Should have space due to resize

    // Should still be able to insert more
    table = table.insert("c".to_string(), 3);
    table = table.insert("d".to_string(), 4);

    assert_eq!(table.lookup(&"a".to_string()), Some(&1));
    assert_eq!(table.lookup(&"b".to_string()), Some(&2));
    assert_eq!(table.lookup(&"c".to_string()), Some(&3));
    assert_eq!(table.lookup(&"d".to_string()), Some(&4));
}

#[test]
fn test_display_format() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 4);

    table = table.insert("key".to_string(), 100);

    let display_str = format!("{}", table);
    assert!(display_str.contains("FlatHashTable"));
    assert!(display_str.contains("LinearProbing"));
    assert!(display_str.contains("elements: 1"));
    assert!(display_str.contains("key → 100"));
}

#[test]
fn test_statistics() {
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 8);

    table = table.insert("key1".to_string(), 1);
    table = table.insert("key2".to_string(), 2);
    let (table, _) = table.delete(&"key1".to_string());

    let stats = table.statistics();
    assert_eq!(stats.num_elements, 1);
    assert_eq!(stats.table_size, 8);
    assert!(stats.load_factor > 0.0);

    let (live, dead, load_factor) = table.probe_statistics();
    assert_eq!(live, 1);
    assert_eq!(dead, 1);
    assert_eq!(load_factor, 2.0 / 8.0); // (live + dead) / size
}
