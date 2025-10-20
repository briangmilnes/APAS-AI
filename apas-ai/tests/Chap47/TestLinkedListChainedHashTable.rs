//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::collections::LinkedList;

use apas_ai::Chap47::ChainedHashTable::ChainedHashTable::*;
use apas_ai::Chap47::LinkedListChainedHashTableStEph::LinkedListChainedHashTableStEph::*;
use apas_ai::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_ai::Types::Types::*;
use std::rc::Rc;

type LLChainTable = HashTable<i32, String, LinkedList<(i32, String)>, ()>;

#[test]
fn test_linkedlist_entry_new() {
    let entry: LinkedList<(i32, String)> = <LinkedList<(i32, String)> as EntryTrait<i32, String>>::new();
    assert!(entry.is_empty());
}

#[test]
fn test_linkedlist_entry_insert() {
    let mut entry: LinkedList<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    assert_eq!(entry.len(), 1);
    assert_eq!(EntryTrait::lookup(&entry, &1), Some("one".to_string()));
}

#[test]
fn test_linkedlist_entry_update() {
    let mut entry: LinkedList<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    EntryTrait::insert(&mut entry, 1, "ONE".to_string());
    assert_eq!(entry.len(), 1);
    assert_eq!(EntryTrait::lookup(&entry, &1), Some("ONE".to_string()));
}

#[test]
fn test_linkedlist_entry_delete() {
    let mut entry: LinkedList<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    EntryTrait::insert(&mut entry, 2, "two".to_string());
    assert!(EntryTrait::delete(&mut entry, &1));
    assert_eq!(EntryTrait::lookup(&entry, &1), None);
    assert_eq!(EntryTrait::lookup(&entry, &2), Some("two".to_string()));
}

#[test]
fn test_linkedlist_chained_insert_lookup() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: LLChainTable = <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        LinkedList<(i32, String)>,
        (),
    >>::createTable(hash_fn_gen, 10);

    for _ in 0..10 {
        table.table.push(LinkedList::new());
    }

    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 11, "eleven".to_string());

    assert_eq!(
        LinkedListChainedHashTableStEph::lookup(&table, &1),
        Some("one".to_string())
    );
    assert_eq!(
        LinkedListChainedHashTableStEph::lookup(&table, &11),
        Some("eleven".to_string())
    );
}

#[test]
fn test_linkedlist_chained_delete() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: LLChainTable = <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        LinkedList<(i32, String)>,
        (),
    >>::createTable(hash_fn_gen, 10);

    for _ in 0..10 {
        table.table.push(LinkedList::new());
    }

    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(LinkedListChainedHashTableStEph::delete(&mut table, &1));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &1), None);
}
