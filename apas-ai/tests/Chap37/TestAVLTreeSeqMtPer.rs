//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeqMtPer.

use apas_ai::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_empty() {
    let tree: AVLTreeSeqMtPerS<i32> = AVLTreeSeqMtPerS::empty();
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_new() {
    let tree: AVLTreeSeqMtPerS<i32> = AVLTreeSeqMtPerS::new();
    assert_eq!(tree.length(), 0);
}

#[test]
fn test_singleton() {
    let tree = AVLTreeSeqMtPerS::singleton(42);
    assert_eq!(tree.length(), 1);
    assert!(tree.isSingleton());
    assert_eq!(*tree.nth(0), 42);
}

#[test]
fn test_length() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(tree.length(), 3);
}

#[test]
fn test_nth() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(1), 2);
    assert_eq!(*tree.nth(2), 3);
}

#[test]
fn test_set() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let new_tree = tree.set(1, 99).unwrap();
    assert_eq!(*new_tree.nth(1), 99);
    // Original unchanged (persistent)
    assert_eq!(*tree.nth(1), 2);
}

#[test]
fn test_isempty() {
    let empty: AVLTreeSeqMtPerS<i32> = AVLTreeSeqMtPerS::empty();
    assert!(empty.isEmpty());

    let non_empty = AVLTreeSeqMtPerS::singleton(1);
    assert!(!non_empty.isEmpty());
}

#[test]
fn test_issingleton() {
    let single = AVLTreeSeqMtPerS::singleton(42);
    assert!(single.isSingleton());

    let multiple = AVLTreeSeqMtPerS::from_vec(vec![1, 2]);
    assert!(!multiple.isSingleton());
}

#[test]
fn test_subseq_copy() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let sub = tree.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 4);
}

#[test]
fn test_from_vec() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(2), 3);
}

#[test]
fn test_values_in_order() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let values = tree.values_in_order();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_iter() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let count = tree.into_iter().count();
    assert_eq!(count, 3);
}

#[test]
fn test_clone() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree2 = tree1.clone();
    assert_eq!(tree1.length(), tree2.length());
    assert_eq!(*tree1.nth(0), *tree2.nth(0));
}

#[test]
fn test_multiple_sets() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree2 = tree.set(0, 10).unwrap();
    let tree3 = tree2.set(2, 30).unwrap();
    let tree4 = tree3.set(4, 50).unwrap();

    assert_eq!(*tree4.nth(0), 10);
    assert_eq!(*tree4.nth(2), 30);
    assert_eq!(*tree4.nth(4), 50);
    assert_eq!(*tree.nth(0), 1); // Original unchanged
}

#[test]
fn test_large_sequence() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..100).collect());
    assert_eq!(tree.length(), 100);
    assert_eq!(*tree.nth(0), 0);
    assert_eq!(*tree.nth(99), 99);
}

#[test]
fn test_persistence() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree2 = tree1.set(1, 99).unwrap();
    let tree3 = tree1.set(0, 10).unwrap();

    assert_eq!(*tree1.nth(1), 2);
    assert_eq!(*tree2.nth(1), 99);
    assert_eq!(*tree3.nth(0), 10);
    assert_eq!(*tree1.nth(0), 1); // Original unchanged
}

#[test]
fn test_empty_subseq() {
    let empty: AVLTreeSeqMtPerS<i32> = AVLTreeSeqMtPerS::empty();
    let sub = empty.subseq_copy(0, 0);
    assert_eq!(sub.length(), 0);
}

#[test]
fn test_set_boundary() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let first = tree.set(0, 100).unwrap();
    let last = tree.set(2, 300).unwrap();
    assert_eq!(*first.nth(0), 100);
    assert_eq!(*last.nth(2), 300);
}

#[test]
fn test_eq() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree2 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree3 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 4]);
    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);
}
