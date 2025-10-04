//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 45: SortedListPQ - Priority Queue using Sorted List

use apas_ai::Chap45::SortedListPQ::SortedListPQ::*;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait;
use apas_ai::Types::Types::*;

#[test]
fn test_empty_priority_queue() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.find_min(), None);
}

#[test]
fn test_singleton_priority_queue() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::singleton(42);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.find_min(), Some(&42));
}

#[test]
fn test_insert_and_find_min() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(10);
    pq = pq.insert(5);
    pq = pq.insert(20);
    assert_eq!(pq.size(), 3);
    assert_eq!(pq.find_min(), Some(&5));
    
    // Verify sorted order is maintained
    let seq = pq.to_seq();
    assert_eq!(seq.nth(0), &5);
    assert_eq!(seq.nth(1), &10);
    assert_eq!(seq.nth(2), &20);
}

#[test]
fn test_delete_min() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(10);
    pq = pq.insert(5);
    pq = pq.insert(20);
    
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(5));
    assert_eq!(new_pq.size(), 2);
    assert_eq!(new_pq.find_min(), Some(&10));
}

#[test]
fn test_delete_min_from_singleton() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::singleton(42);
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(42));
    assert!(new_pq.is_empty());
}

#[test]
fn test_delete_min_from_empty() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, None);
    assert!(new_pq.is_empty());
}

#[test]
fn test_meld_priority_queues() {
    let pq1 = {
    let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
    p = p.insert(10);
    p = p.insert(20);
    p
    };
    
    let pq2 = {
    let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
    p = p.insert(5);
    p = p.insert(15);
    p = p.insert(25);
    p
    };
    
    let merged_pq = pq1.meld(&pq2);
    assert_eq!(merged_pq.size(), 5);
    assert_eq!(merged_pq.find_min(), Some(&5));
    
    // Verify sorted order after meld
    let seq = merged_pq.to_seq();
    assert_eq!(seq.nth(0), &5);
    assert_eq!(seq.nth(1), &10);
    assert_eq!(seq.nth(2), &15);
    assert_eq!(seq.nth(3), &20);
    assert_eq!(seq.nth(4), &25);
}

#[test]
fn test_meld_with_empty() {
    let pq1 = {
    let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
    p = p.insert(10);
    p = p.insert(5);
    p
    };
    let pq2: SortedListPQ<i32> = SortedListPQTrait::empty();
    
    let merged = pq1.meld(&pq2);
    assert_eq!(merged.size(), 2);
    assert_eq!(merged.find_min(), Some(&5));
    
    let merged_reverse = pq2.meld(&pq1);
    assert_eq!(merged_reverse.size(), 2);
    assert_eq!(merged_reverse.find_min(), Some(&5));
}

#[test]
fn test_from_seq() {
    let seq = ArraySeqStPerS::from_vec(vec![5, 2, 8, 1, 9]);
    let pq: SortedListPQ<i32> = SortedListPQTrait::from_seq(&seq);
    
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));
    
    // Verify elements are sorted
    let sorted_seq = pq.to_seq();
    assert_eq!(sorted_seq.nth(0), &1);
    assert_eq!(sorted_seq.nth(1), &2);
    assert_eq!(sorted_seq.nth(2), &5);
    assert_eq!(sorted_seq.nth(3), &8);
    assert_eq!(sorted_seq.nth(4), &9);
}

#[test]
fn test_from_empty_seq() {
    let empty_seq = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::empty();
    let pq: SortedListPQ<i32> = SortedListPQTrait::from_seq(&empty_seq);
    assert!(pq.is_empty());
    assert_eq!(pq.find_min(), None);
}

#[test]
fn test_multiple_inserts_and_deletes() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    
    // Insert elements in random order
    pq = pq.insert(30);
    pq = pq.insert(10);
    pq = pq.insert(40);
    pq = pq.insert(20);
    pq = pq.insert(5);
    
    assert_eq!(pq.find_min(), Some(&5));
    
    // Delete in sorted order
    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(5));
    assert_eq!(pq.find_min(), Some(&10));
    
    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(10));
    assert_eq!(pq.find_min(), Some(&20));
    
    let (pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(20));
    assert_eq!(pq.find_min(), Some(&30));
    
    let (pq, min4) = pq.delete_min();
    assert_eq!(min4, Some(30));
    assert_eq!(pq.find_min(), Some(&40));
    
    let (pq, min5) = pq.delete_min();
    assert_eq!(min5, Some(40));
    assert!(pq.is_empty());
}

#[test]
fn test_string_elements() {
    let mut pq: SortedListPQ<String> = SortedListPQTrait::empty();
    pq = pq.insert("banana".to_string());
    pq = pq.insert("apple".to_string());
    pq = pq.insert("zebra".to_string());
    
    assert_eq!(pq.find_min(), Some(&"apple".to_string()));
    
    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some("apple".to_string()));
    assert_eq!(pq.find_min(), Some(&"banana".to_string()));
}

#[test]
fn test_duplicate_elements() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(5);
    pq = pq.insert(5);
    pq = pq.insert(3);
    pq = pq.insert(5);
    
    assert_eq!(pq.size(), 4);
    assert_eq!(pq.find_min(), Some(&3));
    
    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(3));
    
    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(5));
    
    let (pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(5));
    
    let (pq, min4) = pq.delete_min();
    assert_eq!(min4, Some(5));
    
    assert!(pq.is_empty());
}

#[test]
fn test_large_sequence() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    
    // Insert many elements
    for i in (1..=100).rev() {
    pq = pq.insert(i);
    }
    
    assert_eq!(pq.size(), 100);
    assert_eq!(pq.find_min(), Some(&1));
    
    // Verify sorted order
    for expected in 1..=100 {
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(expected));
    pq = new_pq;
    }
    
    assert!(pq.is_empty());
}

#[test]
fn test_meld_large_queues() {
    let pq1 = {
    let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
    for i in (1..=50).step_by(2) { // Odd numbers
        p = p.insert(i);
    }
    p
    };
    
    let pq2 = {
    let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
    for i in (2..=50).step_by(2) { // Even numbers
        p = p.insert(i);
    }
    p
    };
    
    let merged = pq1.meld(&pq2);
    assert_eq!(merged.size(), 50);
    assert_eq!(merged.find_min(), Some(&1));
    
    // Verify all numbers 1-50 are present in order
    let mut current_pq = merged;
    for expected in 1..=50 {
    let (new_pq, min_val) = current_pq.delete_min();
    assert_eq!(min_val, Some(expected));
    current_pq = new_pq;
    }
    
    assert!(current_pq.is_empty());
}

#[test]
fn test_persistent_nature() {
    let pq1: SortedListPQ<i32> = SortedListPQTrait::singleton(10);
    let pq2 = pq1.insert(5);
    let pq3 = pq2.insert(15);
    
    // Original queues should be unchanged
    assert_eq!(pq1.size(), 1);
    assert_eq!(pq1.find_min(), Some(&10));
    
    assert_eq!(pq2.size(), 2);
    assert_eq!(pq2.find_min(), Some(&5));
    
    assert_eq!(pq3.size(), 3);
    assert_eq!(pq3.find_min(), Some(&5));
    
    // Test delete_min persistence
    let (pq4, deleted) = pq3.delete_min();
    assert_eq!(deleted, Some(5));
    assert_eq!(pq4.size(), 2);
    assert_eq!(pq4.find_min(), Some(&10));
    
    // pq3 should be unchanged
    assert_eq!(pq3.size(), 3);
    assert_eq!(pq3.find_min(), Some(&5));
}

#[test]
fn test_to_seq_conversion() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let values = vec![30, 10, 20, 5, 25];
    
    for val in values {
    pq = pq.insert(val);
    }
    
    let seq = pq.to_seq();
    assert_eq!(seq.length(), 5);
    
    // Should be in sorted order
    let expected = vec![5, 10, 20, 25, 30];
    for (i, &expected_val) in expected.iter().enumerate() {
    assert_eq!(seq.nth(i as N), &expected_val);
    }
}