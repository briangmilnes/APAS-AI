//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap43Claude OrderedTableMtEph.

use apas_ai::Chap43Claude::OrderedTableMtEph::OrderedTableMtEph::*;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerTrait, AVLTreeSeqStPerS};
use apas_ai::Types::Types::*;
use std::sync::Arc;
use std::thread;

#[test]
fn test_ordered_table_mt_eph_new() {
    let table = OrderedTableMtEph::<i32, String>::new();
    assert_eq!(table.size(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_ordered_table_mt_eph_insert_and_lookup() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&1), Some("one".to_string()));
    assert_eq!(table.lookup(&2), Some("two".to_string()));
    assert_eq!(table.lookup(&3), None);
}

#[test]
fn test_ordered_table_mt_eph_delete() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    
    let deleted = table.delete(&1);
    assert_eq!(deleted, Some("one".to_string()));
    assert_eq!(table.size(), 1);
    assert_eq!(table.lookup(&1), None);
    assert_eq!(table.lookup(&2), Some("two".to_string()));
}

#[test]
fn test_ordered_table_mt_eph_parallel_operations() {
    let mut table = OrderedTableMtEph::new();
    
    // Insert test data
    for i in 0..100 {
        table.insert(i, format!("value_{}", i), |_old, new| new.clone());
    }
    
    // Test parallel filter
    let filtered = table.filter(|k, _v| k % 2 == 0);
    assert_eq!(filtered.size(), 50);
    
    // Test parallel map
    let mapped = table.map(|k, v| format!("{}:{}", k, v));
    assert_eq!(mapped.size(), 100);
    
    // Test parallel reduce
    let sum = table.reduce(0, |acc, k, _v| acc + k);
    assert_eq!(sum, (0..100).sum::<i32>());
}

#[test]
fn test_ordered_table_mt_eph_first_key() {
    let mut table = OrderedTableMtEph::new();
    assert_eq!(table.first_key(), None);
    
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.first_key(), Some(1));
}

#[test]
fn test_ordered_table_mt_eph_last_key() {
    let mut table = OrderedTableMtEph::new();
    assert_eq!(table.last_key(), None);
    
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.last_key(), Some(3));
}

#[test]
fn test_ordered_table_mt_eph_previous_key() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(5, "five".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.previous_key(&1), None);
    assert_eq!(table.previous_key(&3), Some(1));
    assert_eq!(table.previous_key(&5), Some(3));
    assert_eq!(table.previous_key(&4), Some(3));
}

#[test]
fn test_ordered_table_mt_eph_next_key() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(5, "five".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.next_key(&1), Some(3));
    assert_eq!(table.next_key(&3), Some(5));
    assert_eq!(table.next_key(&5), None);
    assert_eq!(table.next_key(&2), Some(3));
}

#[test]
fn test_ordered_table_mt_eph_split_key() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());
    
    let (left, right) = table.split_key(&3);
    
    // Left should contain keys < 3
    assert_eq!(left.size(), 2);
    assert_eq!(left.lookup(&1), Some("one".to_string()));
    assert_eq!(left.lookup(&2), Some("two".to_string()));
    assert_eq!(left.lookup(&3), None);
    
    // Right should contain keys >= 3
    assert_eq!(right.size(), 2);
    assert_eq!(right.lookup(&3), Some("three".to_string()));
    assert_eq!(right.lookup(&4), Some("four".to_string()));
    assert_eq!(right.lookup(&1), None);
}

#[test]
fn test_ordered_table_mt_eph_join_key() {
    let mut left = OrderedTableMtEph::new();
    left.insert(1, "one".to_string(), |_old, new| new.clone());
    left.insert(2, "two".to_string(), |_old, new| new.clone());
    
    let mut right = OrderedTableMtEph::new();
    right.insert(3, "three".to_string(), |_old, new| new.clone());
    right.insert(4, "four".to_string(), |_old, new| new.clone());
    
    left.join_key(right);
    
    assert_eq!(left.size(), 4);
    assert_eq!(left.lookup(&1), Some("one".to_string()));
    assert_eq!(left.lookup(&2), Some("two".to_string()));
    assert_eq!(left.lookup(&3), Some("three".to_string()));
    assert_eq!(left.lookup(&4), Some("four".to_string()));
    
    // right is consumed by join_key, so we can't test its size
}

#[test]
fn test_ordered_table_mt_eph_get_key_range() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());
    table.insert(5, "five".to_string(), |_old, new| new.clone());
    
    let range = table.get_key_range(&2, &4);
    
    assert_eq!(range.size(), 3);
    assert_eq!(range.lookup(&2), Some("two".to_string()));
    assert_eq!(range.lookup(&3), Some("three".to_string()));
    assert_eq!(range.lookup(&4), Some("four".to_string()));
    assert_eq!(range.lookup(&1), None);
    assert_eq!(range.lookup(&5), None);
}

#[test]
fn test_ordered_table_mt_eph_rank_key() {
    let mut table = OrderedTableMtEph::new();
    table.insert(10, "ten".to_string(), |_old, new| new.clone());
    table.insert(20, "twenty".to_string(), |_old, new| new.clone());
    table.insert(30, "thirty".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.rank_key(&10), 0);
    assert_eq!(table.rank_key(&20), 1);
    assert_eq!(table.rank_key(&30), 2);
    assert_eq!(table.rank_key(&15), 1); // Between 10 and 20
}

#[test]
fn test_ordered_table_mt_eph_select_key() {
    let mut table = OrderedTableMtEph::new();
    table.insert(10, "ten".to_string(), |_old, new| new.clone());
    table.insert(20, "twenty".to_string(), |_old, new| new.clone());
    table.insert(30, "thirty".to_string(), |_old, new| new.clone());
    
    assert_eq!(table.select_key(0), Some(10));
    assert_eq!(table.select_key(1), Some(20));
    assert_eq!(table.select_key(2), Some(30));
    assert_eq!(table.select_key(3), None);
}

#[test]
fn test_ordered_table_mt_eph_split_rank_key() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());
    
    let (left, right) = table.split_rank_key(2);
    
    // Left should contain first 2 elements
    assert_eq!(left.size(), 2);
    assert_eq!(left.lookup(&1), Some("one".to_string()));
    assert_eq!(left.lookup(&2), Some("two".to_string()));
    
    // Right should contain remaining elements
    assert_eq!(right.size(), 2);
    assert_eq!(right.lookup(&3), Some("three".to_string()));
    assert_eq!(right.lookup(&4), Some("four".to_string()));
}

#[test]
fn test_ordered_table_mt_eph_ephemeral_semantics() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    
    let original_size = table.size();
    
    // Ephemeral operations should modify the original table
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    assert_eq!(table.size(), original_size + 1);
    
    table.delete(&1);
    assert_eq!(table.size(), original_size);
    assert_eq!(table.lookup(&1), None);
}

#[test]
fn test_ordered_table_mt_eph_thread_safety() {
    let table = Arc::new(OrderedTableMtEph::<i32, String>::new());
    let mut handles = vec![];
    
    // Test concurrent reads
    for i in 0..10 {
        let table_clone: Arc<OrderedTableMtEph<i32, String>> = Arc::clone(&table);
        let handle = thread::spawn(move || {
            // Each thread tries to read from the table
            let _ = table_clone.lookup(&i);
            let _ = table_clone.size();
            let _ = table_clone.is_empty();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_ordered_table_mt_eph_collect() {
    let mut table = OrderedTableMtEph::new();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    
    let collected = table.collect();
    assert_eq!(collected.length(), 2);
}

#[test]
fn test_ordered_table_mt_eph_from_sorted_entries() {
    let entries = vec![
        Pair(1, "one".to_string()),
        Pair(2, "two".to_string()),
        Pair(3, "three".to_string()),
    ];
    
    let seq = AVLTreeSeqStPerS::from_vec(entries);
    let table = from_sorted_entries(seq);
    
    assert_eq!(table.size(), 3);
    assert_eq!(table.lookup(&1), Some("one".to_string()));
    assert_eq!(table.lookup(&2), Some("two".to_string()));
    assert_eq!(table.lookup(&3), Some("three".to_string()));
}

#[test]
fn test_ordered_table_mt_eph_empty_operations() {
    let mut table = OrderedTableMtEph::<i32, String>::new();
    
    assert_eq!(table.first_key(), None);
    assert_eq!(table.last_key(), None);
    assert_eq!(table.previous_key(&1), None);
    assert_eq!(table.next_key(&1), None);
    assert_eq!(table.select_key(0), None);
    
    let (left, right) = table.split_key(&1);
    assert_eq!(left.size(), 0);
    assert_eq!(right.size(), 0);
    
    let range = table.get_key_range(&1, &5);
    assert_eq!(range.size(), 0);
}

#[test]
fn test_ordered_table_mt_eph_large_dataset_parallel() {
    let mut table = OrderedTableMtEph::new();
    
    // Insert a large dataset
    for i in 0..1000 {
        table.insert(i, format!("value_{}", i), |_old, new| new.clone());
    }
    
    // Test parallel operations on large dataset
    let even_filtered = table.filter(|k, _v| k % 2 == 0);
    assert_eq!(even_filtered.size(), 500);
    
    let mapped = table.map(|k, v| format!("mapped_{}:{}", k, v));
    assert_eq!(mapped.size(), 1000);
    
    // Test ordering operations
    assert_eq!(table.first_key(), Some(0));
    assert_eq!(table.last_key(), Some(999));
    assert_eq!(table.select_key(500), Some(500));
    assert_eq!(table.rank_key(&500), 500);
}
