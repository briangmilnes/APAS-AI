//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test for Chapter 45: BalancedTreePQ - Priority Queue using AVL Trees

use apas_ai::Chap45::BalancedTreePQ::BalancedTreePQ::*;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStPerSLit, AVLTreeSeqStPerSLit};

#[test]
fn test_empty_priority_queue() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.find_min(), None);
    }

#[test]
fn test_singleton_priority_queue() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::singleton(42);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.find_min(), Some(&42));
    }

#[test]
fn test_insert_and_find_min() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10);
    let pq = pq.insert(5);
    let pq = pq.insert(15);
    let pq = pq.insert(3);
    let pq = pq.insert(8);
    
    assert_eq!(pq.find_min(), Some(&3));
    assert_eq!(pq.size(), 5);
    }

#[test]
fn test_delete_min() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10);
    let pq = pq.insert(5);
    let pq = pq.insert(15);
    let pq = pq.insert(3);
    
    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(3));
    assert_eq!(pq.find_min(), Some(&5));
    assert_eq!(pq.size(), 3);
    
    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(5));
    assert_eq!(pq.find_min(), Some(&10));
    assert_eq!(pq.size(), 2);
    }

#[test]
fn test_delete_min_empty() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, None);
    assert!(pq.is_empty());
    }

#[test]
fn test_meld_two_priority_queues() {
    let pq1: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq1 = pq1.insert(10).insert(5).insert(15);
    
    let pq2: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq2 = pq2.insert(2).insert(8).insert(12);
    
    let melded = pq1.meld(&pq2);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&2));
    
    // Verify all elements are present by extracting in order
    let (melded, min1) = melded.delete_min();
    assert_eq!(min1, Some(2));
    let (melded, min2) = melded.delete_min();
    assert_eq!(min2, Some(5));
    let (melded, min3) = melded.delete_min();
    assert_eq!(min3, Some(8));
    }

#[test]
fn test_meld_with_empty() {
    let pq1: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq1 = pq1.insert(10).insert(5);
    
    let pq2: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    
    let melded = pq1.meld(&pq2);
    assert_eq!(melded.size(), 2);
    assert_eq!(melded.find_min(), Some(&5));
    }

#[test]
fn test_from_seq() {
    let elements = AVLTreeSeqStPerSLit![10, 5, 15, 3, 8, 12];
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::from_seq(&elements);
    
    assert_eq!(pq.size(), 6);
    assert_eq!(pq.find_min(), Some(&3));
    }

#[test]
fn test_to_seq() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15);
    
    let seq = pq.to_seq();
    assert_eq!(seq.length(), 3);
    // Elements should be in sorted order in the sequence
    assert_eq!(seq.nth(0), 5);
    assert_eq!(seq.nth(1), 10);
    assert_eq!(seq.nth(2), 15);
    }

#[test]
fn test_priority_queue_ordering() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let elements = [42, 17, 89, 3, 56, 23, 71, 8];
    
    let mut pq = pq;
    for &elem in &elements {
    pq = pq.insert(elem);
    }
    
    let mut sorted_elements = elements.to_vec();
    sorted_elements.sort();
    
    // Extract all elements and verify they come out in sorted order
    let mut extracted = ArraySeqStPerSLit![];
    let mut current_pq = pq;
    
    while !current_pq.is_empty() {
    let (new_pq, min_val) = current_pq.delete_min();
    if let Some(val) = min_val {
        extracted = extracted.append(&ArraySeqStPerSLit![val]);
    }
    current_pq = new_pq;
    }
    
    for (i, &expected) in sorted_elements.iter().enumerate() {
    assert_eq!(extracted.nth(i as N), expected);
    }
    }

#[test]
fn test_duplicate_elements() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(5).insert(3).insert(5).insert(3).insert(5);
    
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&3));
    
    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(3));
    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(3));
    let (pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(5));
    }

#[test]
fn test_string_priority_queue() {
    let pq: BalancedTreePQ<String> = BalancedTreePQTrait::empty();
    let pq = pq.insert("zebra".to_string())
           .insert("apple".to_string())
           .insert("banana".to_string())
           .insert("cherry".to_string());
    
    assert_eq!(pq.find_min(), Some(&"apple".to_string()));
    
    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some("apple".to_string()));
    assert_eq!(pq.find_min(), Some(&"banana".to_string()));
    }

#[test]
fn test_large_priority_queue() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let mut pq = pq;
    
    // Insert 100 random-order elements
    let elements: ArraySeqStPerS<i32> = ArraySeqStPerSTrait::tabulate(&|i| (i * 17 + 13) % 97, 100);
    for i in 0..elements.length() {
    pq = pq.insert(elements.nth(i));
    }
    
    assert_eq!(pq.size(), 100);
    assert!(!pq.is_empty());
    
    // Extract first 10 elements to verify ordering
    let mut current_pq = pq;
    let mut prev_min = -1;
    
    for _ in 0..10 {
    let (new_pq, min_val) = current_pq.delete_min();
    if let Some(val) = min_val {
        assert!(val >= prev_min);
        prev_min = val;
    }
    current_pq = new_pq;
    }
}
