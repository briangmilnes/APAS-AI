use apas_ai::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_ai::Chap47clean::ChainedHashTable::ChainedHashTable::*;
use apas_ai::Chap47clean::StructChainedHashTable::StructChainedHashTable::*;
use apas_ai::Types::Types::*;

type StructChainTable = HashTable<i32, String, ChainList<i32, String>, ()>;

#[test]
fn test_chainlist_new() {
    let list: ChainList<i32, String> = ChainList::new();
    assert!(list.head.is_none());
}

#[test]
fn test_chainlist_insert() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    assert_eq!(EntryTrait::lookup(&list, &1), Some("one".to_string()));
}

#[test]
fn test_chainlist_insert_multiple() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    EntryTrait::insert(&mut list, 2, "two".to_string());
    EntryTrait::insert(&mut list, 3, "three".to_string());
    assert_eq!(EntryTrait::lookup(&list, &1), Some("one".to_string()));
    assert_eq!(EntryTrait::lookup(&list, &2), Some("two".to_string()));
    assert_eq!(EntryTrait::lookup(&list, &3), Some("three".to_string()));
}

#[test]
fn test_chainlist_update() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    EntryTrait::insert(&mut list, 1, "ONE".to_string());
    assert_eq!(EntryTrait::lookup(&list, &1), Some("ONE".to_string()));
}

#[test]
fn test_chainlist_delete() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    EntryTrait::insert(&mut list, 2, "two".to_string());
    assert!(EntryTrait::delete(&mut list, &1));
    assert_eq!(EntryTrait::lookup(&list, &1), None);
    assert_eq!(EntryTrait::lookup(&list, &2), Some("two".to_string()));
}

#[test]
fn test_chainlist_delete_not_found() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    assert!(!EntryTrait::delete(&mut list, &999));
}

#[test]
fn test_struct_chained_insert_lookup() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: StructChainTable = 
        <StructChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, ChainList<i32, String>, ()>>::createTable(hash_fn, 10);
    
    for _ in 0..10 {
        table.table.push(ChainList::new());
    }
    
    StructChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    StructChainedHashTableStEph::insert(&mut table, 11, "eleven".to_string());
    
    assert_eq!(StructChainedHashTableStEph::lookup(&table, &1), Some("one".to_string()));
    assert_eq!(StructChainedHashTableStEph::lookup(&table, &11), Some("eleven".to_string()));
}

#[test]
fn test_struct_chained_delete() {
    let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 10);
    let mut table: StructChainTable = 
        <StructChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, ChainList<i32, String>, ()>>::createTable(hash_fn, 10);
    
    for _ in 0..10 {
        table.table.push(ChainList::new());
    }
    
    StructChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(StructChainedHashTableStEph::delete(&mut table, &1));
    assert_eq!(StructChainedHashTableStEph::lookup(&table, &1), None);
}