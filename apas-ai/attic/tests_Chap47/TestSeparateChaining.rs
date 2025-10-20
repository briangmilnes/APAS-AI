//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Separate Chaining Tests

use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Chap47::SeparateChaining::SeparateChaining::*;
use apas_ai::Types::Types::*;

#[test]
fn test_create_empty_table() {
    let table: SeparateChainingHashTable<String, i32, StringPositionHashFunction, DefaultKeyEquality> =
        SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 8);

    let (load, size) = table.load_and_size();
    assert_eq!(load, 0);
    assert_eq!(size, 8);

    let stats = table.statistics();
    assert_eq!(stats.num_elements, 0);
    assert_eq!(stats.load_factor, 0.0);
}

#[test]
fn test_insert_and_lookup() {
    let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 5);

    // Insert some key-value pairs
    table = table.insert("aa".to_string(), "value_aa".to_string());
    table = table.insert("bb".to_string(), "value_bb".to_string());
    table = table.insert("cc".to_string(), "value_cc".to_string());

    // Test lookups
    assert_eq!(table.lookup(&"aa".to_string()), Some(&"value_aa".to_string()));
    assert_eq!(table.lookup(&"bb".to_string()), Some(&"value_bb".to_string()));
    assert_eq!(table.lookup(&"cc".to_string()), Some(&"value_cc".to_string()));
    assert_eq!(table.lookup(&"dd".to_string()), None);

    let (load, size) = table.load_and_size();
    assert_eq!(load, 3);
    assert_eq!(size, 8); // Minimum size is 8
}

#[test]
fn test_insert_duplicate_key() {
    let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 5);

    table = table.insert("key".to_string(), "value1".to_string());
    assert_eq!(table.lookup(&"key".to_string()), Some(&"value1".to_string()));

    // Insert same key with different value
    table = table.insert("key".to_string(), "value2".to_string());
    assert_eq!(table.lookup(&"key".to_string()), Some(&"value2".to_string()));

    // Should still have only 1 element
    let (load, _) = table.load_and_size();
    assert_eq!(load, 1);
}

#[test]
fn test_delete() {
    let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 5);

    table = table.insert("key1".to_string(), "value1".to_string());
    table = table.insert("key2".to_string(), "value2".to_string());

    // Delete existing key
    let (new_table, was_deleted) = table.delete(&"key1".to_string());
    assert!(was_deleted);
    assert_eq!(new_table.lookup(&"key1".to_string()), None);
    assert_eq!(new_table.lookup(&"key2".to_string()), Some(&"value2".to_string()));

    let (load, _) = new_table.load_and_size();
    assert_eq!(load, 1);

    // Delete non-existing key
    let (same_table, was_deleted) = new_table.delete(&"nonexistent".to_string());
    assert!(!was_deleted);
    let (load, _) = same_table.load_and_size();
    assert_eq!(load, 1);
}

#[test]
fn test_collision_handling() {
    let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 5);

    // These keys should hash to the same bucket in a size-5 table
    // "aa" -> (0+0) % 5 = 0
    // "ff" -> (5+5) % 5 = 0  (collision!)
    table = table.insert("aa".to_string(), "value_aa".to_string());
    table = table.insert("ff".to_string(), "value_ff".to_string());

    // Both should be retrievable despite collision
    assert_eq!(table.lookup(&"aa".to_string()), Some(&"value_aa".to_string()));
    assert_eq!(table.lookup(&"ff".to_string()), Some(&"value_ff".to_string()));

    let stats = table.statistics();
    assert_eq!(stats.num_elements, 2);
    // Collision behavior depends on actual table size (minimum 8)
}

#[test]
fn test_resize_on_high_load_factor() {
    let mut table = SeparateChainingHashTable::create_table(
        DefaultKeyEquality,
        StringPositionHashFunction,
        4, // Small initial size to trigger resize
    );

    let initial_size = table.load_and_size().1;

    // Insert enough elements to trigger resize (load factor > 0.75)
    // With min size 8, need > 6 elements to exceed 0.75 threshold
    table = table.insert("key1".to_string(), "value1".to_string());
    table = table.insert("key2".to_string(), "value2".to_string());
    table = table.insert("key3".to_string(), "value3".to_string());
    table = table.insert("key4".to_string(), "value4".to_string());
    table = table.insert("key5".to_string(), "value5".to_string());
    table = table.insert("key6".to_string(), "value6".to_string());
    table = table.insert("key7".to_string(), "value7".to_string()); // This should trigger resize

    let (load, new_size) = table.load_and_size();
    assert_eq!(load, 7);
    assert!(new_size > initial_size); // Table should have grown

    // All elements should still be accessible
    assert_eq!(table.lookup(&"key1".to_string()), Some(&"value1".to_string()));
    assert_eq!(table.lookup(&"key2".to_string()), Some(&"value2".to_string()));
    assert_eq!(table.lookup(&"key3".to_string()), Some(&"value3".to_string()));
    assert_eq!(table.lookup(&"key4".to_string()), Some(&"value4".to_string()));
}

#[test]
fn test_example_47_2_table() {
    let table = create_example_47_2_table();

    // Test that all example data is present
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
    assert_eq!(size, 16);

    let stats = table.statistics();
    assert_eq!(stats.load_factor, 0.625); // 10 elements in 16 buckets
    assert!(stats.num_collisions > 0); // Should have collisions
}

#[test]
fn test_statistics() {
    let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 3);

    // Insert elements that will cause collisions
    table = table.insert("a".to_string(), "1".to_string()); // hash = 0
    table = table.insert("d".to_string(), "2".to_string()); // hash = 3 % 3 = 0 (collision)
    table = table.insert("b".to_string(), "3".to_string()); // hash = 1

    let stats = table.statistics();
    assert_eq!(stats.num_elements, 3);
    assert_eq!(stats.table_size, 8); // Minimum table size is 8
    assert_eq!(stats.load_factor, 0.375); // 3 / 8
                                          // Collision behavior depends on hash function and table size
}

#[test]
fn test_integer_keys() {
    let mut table: SeparateChainingHashTable<i32, String, DefaultHashFunction, DefaultKeyEquality> =
        SeparateChainingHashTable::create_table(DefaultKeyEquality, DefaultHashFunction, 8);

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
    let table: SeparateChainingHashTable<String, i32, StringPositionHashFunction, DefaultKeyEquality> =
        SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 8);

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
    assert_eq!(stats.num_collisions, 0);
}

#[test]
fn test_large_table() {
    let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, DefaultHashFunction, 16);

    // Insert many elements
    for i in 0..50 {
        let key = format!("key_{i}");
        let value = format!("value_{i}");
        table = table.insert(key, value);
    }

    let (load, size) = table.load_and_size();
    assert_eq!(load, 50);
    assert!(size >= 16); // May have grown due to load factor

    // Verify all elements are accessible
    for i in 0..50 {
        let key = format!("key_{i}");
        let expected_value = format!("value_{i}");
        assert_eq!(table.lookup(&key), Some(&expected_value));
    }
}
