use apas_ai::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_ai::Chap47clean::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47clean::LinProbFlatHashTable::LinProbFlatHashTable::*;
use apas_ai::Types::Types::*;

type FlatTable = HashTable<i32, String, FlatEntry<i32, String>, ()>;

#[test]
fn test_insert_and_lookup() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: FlatTable = 
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(hash_fn, 10);
    
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
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(hash_fn, 10);
    
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
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(hash_fn, 10);
    
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
        <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(hash_fn, 10);
    
    for _ in 0..10 {
        table.table.push(FlatEntry::Empty);
    }
    
    let slot = LinProbFlatHashTableStEph::find_slot(&table, &5);
    assert!(slot < 10);
}