//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap47clean::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47clean::LinProbFlatHashTable::LinProbFlatHashTable::*;
use apas_ai::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_ai::Types::Types::*;

type FlatTable = HashTable<i32, String, FlatEntry<i32, String>, ()>;

#[test]
fn test_insert_and_lookup() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    // Initialize table with Empty entries
    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    LinProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    LinProbFlatHashTableStEph::insert(&mut table, 11, "eleven".to_string());

    let result = LinProbFlatHashTableStEph::lookup(&table, &1);
    assert_eq!(result, Some("one".to_string()));

    let result2 = LinProbFlatHashTableStEph::lookup(&table, &11);
    assert_eq!(result2, Some("eleven".to_string()));
}

#[test]
fn test_delete() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    LinProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(LinProbFlatHashTableStEph::delete(&mut table, &1));
    assert_eq!(LinProbFlatHashTableStEph::lookup(&table, &1), None);
}

#[test]
fn test_probe() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    let slot0 = LinProbFlatHashTableStEph::probe(&table, &5, 0);
    let slot1 = LinProbFlatHashTableStEph::probe(&table, &5, 1);
    assert_ne!(slot0, slot1);
    assert!(slot0 < 10);
    assert!(slot1 < 10);
}

#[test]
fn test_find_slot() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    let slot = LinProbFlatHashTableStEph::find_slot(&table, &5);
    assert!(slot < 10);
}

#[test]
fn test_flat_entry_new() {
    let entry: FlatEntry<i32, String> = FlatEntry::new();
    match entry {
        FlatEntry::Empty => assert!(true),
        _ => panic!("Expected Empty"),
    }
}

#[test]
fn test_flat_entry_insert() {
    let mut entry: FlatEntry<i32, String> = FlatEntry::Empty;
    entry.insert(42, "forty-two".to_string());
    match entry {
        FlatEntry::Occupied(k, v) => {
            assert_eq!(k, 42);
            assert_eq!(v, "forty-two");
        }
        _ => panic!("Expected Occupied"),
    }
}

#[test]
fn test_flat_entry_lookup() {
    let mut entry: FlatEntry<i32, String> = FlatEntry::Empty;
    assert_eq!(entry.lookup(&42), None);
    
    entry.insert(42, "forty-two".to_string());
    assert_eq!(entry.lookup(&42), Some("forty-two".to_string()));
    assert_eq!(entry.lookup(&99), None);
}

#[test]
fn test_flat_entry_delete() {
    let mut entry: FlatEntry<i32, String> = FlatEntry::Empty;
    assert!(!entry.delete(&42));
    
    entry.insert(42, "forty-two".to_string());
    assert!(entry.delete(&42));
    match entry {
        FlatEntry::Deleted => assert!(true),
        _ => panic!("Expected Deleted"),
    }
    
    // Deleting from Deleted should return false
    assert!(!entry.delete(&42));
}

#[test]
fn test_default_insert_with_probe() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    // Call the default trait method directly
    <LinProbFlatHashTableStEph as FlatHashTable<i32, String, FlatEntry<i32, String>, ()>>::insert_with_probe(
        &mut table, 5, "five".to_string()
    );
    
    // Verify it was inserted
    let result = LinProbFlatHashTableStEph::lookup(&table, &5);
    assert_eq!(result, Some("five".to_string()));
}

#[test]
fn test_default_lookup_with_probe() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    LinProbFlatHashTableStEph::insert(&mut table, 5, "five".to_string());
    
    // Call the default trait method directly
    let result = <LinProbFlatHashTableStEph as FlatHashTable<i32, String, FlatEntry<i32, String>, ()>>::lookup_with_probe(
        &table, &5
    );
    assert_eq!(result, Some("five".to_string()));
    
    let result_missing = <LinProbFlatHashTableStEph as FlatHashTable<i32, String, FlatEntry<i32, String>, ()>>::lookup_with_probe(
        &table, &99
    );
    assert_eq!(result_missing, None);
}

#[test]
fn test_flat_entry_clone() {
    let entry1: FlatEntry<i32, String> = FlatEntry::Occupied(42, "forty-two".to_string());
    let entry2 = entry1.clone();
    assert_eq!(entry1, entry2);
}

#[test]
fn test_flat_entry_debug() {
    let entry: FlatEntry<i32, String> = FlatEntry::Occupied(42, "forty-two".to_string());
    let debug_str = format!("{:?}", entry);
    assert!(debug_str.contains("Occupied"));
}

#[test]
fn test_flat_entry_partial_eq() {
    let entry1: FlatEntry<i32, String> = FlatEntry::Empty;
    let entry2: FlatEntry<i32, String> = FlatEntry::Empty;
    assert_eq!(entry1, entry2);
    
    let entry3: FlatEntry<i32, String> = FlatEntry::Deleted;
    assert_ne!(entry1, entry3);
    
    let entry4: FlatEntry<i32, String> = FlatEntry::Occupied(42, "forty-two".to_string());
    let entry5: FlatEntry<i32, String> = FlatEntry::Occupied(42, "forty-two".to_string());
    assert_eq!(entry4, entry5);
}

#[test]
fn test_insert_update_existing_key() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    LinProbFlatHashTableStEph::insert(&mut table, 5, "five".to_string());
    assert_eq!(table.num_elements, 1);
    
    // Update existing key
    LinProbFlatHashTableStEph::insert(&mut table, 5, "FIVE".to_string());
    assert_eq!(table.num_elements, 1); // Should still be 1
    assert_eq!(LinProbFlatHashTableStEph::lookup(&table, &5), Some("FIVE".to_string()));
}

#[test]
fn test_insert_into_deleted_slot() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    LinProbFlatHashTableStEph::insert(&mut table, 5, "five".to_string());
    LinProbFlatHashTableStEph::delete(&mut table, &5);
    
    // Now insert into the deleted slot
    LinProbFlatHashTableStEph::insert(&mut table, 5, "FIVE".to_string());
    assert_eq!(LinProbFlatHashTableStEph::lookup(&table, &5), Some("FIVE".to_string()));
}

#[test]
fn test_lookup_through_deleted_entries() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert two keys that hash to the same location
    LinProbFlatHashTableStEph::insert(&mut table, 5, "five".to_string());
    LinProbFlatHashTableStEph::insert(&mut table, 15, "fifteen".to_string());
    
    // Delete the first one
    LinProbFlatHashTableStEph::delete(&mut table, &5);
    
    // Lookup should still find the second one through the deleted entry
    assert_eq!(LinProbFlatHashTableStEph::lookup(&table, &15), Some("fifteen".to_string()));
}

#[test]
fn test_delete_not_found() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    // Try to delete non-existent key
    assert!(!LinProbFlatHashTableStEph::delete(&mut table, &99));
    assert_eq!(table.num_elements, 0);
}

#[test]
fn test_find_slot_with_occupied_matching_key() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert a key
    LinProbFlatHashTableStEph::insert(&mut table, 5, "five".to_string());
    
    // find_slot should return the same slot for the same key
    let slot1 = LinProbFlatHashTableStEph::find_slot(&table, &5);
    let slot2 = LinProbFlatHashTableStEph::find_slot(&table, &5);
    assert_eq!(slot1, slot2);
}

#[test]
fn test_lookup_exhaustive_probe() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable =
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn, 10,
        );

    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }

    // Fill table with non-matching keys
    for i in 0..9 {
        LinProbFlatHashTableStEph::insert(&mut table, i, format!("num{}", i));
    }
    
    // Lookup a key that's not in the table - should probe through all entries
    assert_eq!(LinProbFlatHashTableStEph::lookup(&table, &99), None);
}
