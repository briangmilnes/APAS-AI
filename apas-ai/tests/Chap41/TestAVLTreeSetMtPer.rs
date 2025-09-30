//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetMtPer.

use apas_ai::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_empty_set() {
    let s: AVLTreeSetMtPer<N> = AVLTreeSetMtPer::empty();
    assert_eq!(s.size(), 0);
}

#[test]
fn test_singleton_set() {
    let s = AVLTreeSetMtPer::singleton(42);
    assert_eq!(s.size(), 1);
    assert!(s.find(&42));
    assert!(!s.find(&43));
}

#[test]
fn test_insert() {
    let s = AVLTreeSetMtPer::empty();
    let s1 = s.insert(1);
    let s2 = s1.insert(2);
    let s3 = s2.insert(3);

    assert_eq!(s3.size(), 3);
    assert!(s3.find(&1));
    assert!(s3.find(&2));
    assert!(s3.find(&3));
    assert!(!s3.find(&4));
}

#[test]
fn test_delete() {
    let s = AVLTreeSetMtPer::empty();
    let s1 = s.insert(1).insert(2).insert(3);
    let s2 = s1.delete(&2);

    assert_eq!(s2.size(), 2);
    assert!(s2.find(&1));
    assert!(!s2.find(&2));
    assert!(s2.find(&3));
}

#[test]
fn test_union() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = AVLTreeSetMtPer::empty().insert(3).insert(4).insert(5);
    let s3 = s1.union(&s2);

    assert_eq!(s3.size(), 5);
    assert!(s3.find(&1));
    assert!(s3.find(&2));
    assert!(s3.find(&3));
    assert!(s3.find(&4));
    assert!(s3.find(&5));
}

#[test]
fn test_intersection() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3).insert(4);
    let s3 = s1.intersection(&s2);

    assert_eq!(s3.size(), 2);
    assert!(!s3.find(&1));
    assert!(s3.find(&2));
    assert!(s3.find(&3));
    assert!(!s3.find(&4));
}

#[test]
fn test_difference() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3).insert(4);
    let s3 = s1.difference(&s2);

    assert_eq!(s3.size(), 1);
    assert!(s3.find(&1));
    assert!(!s3.find(&2));
    assert!(!s3.find(&3));
    assert!(!s3.find(&4));
}

#[test]
fn test_filter() {
    let s = AVLTreeSetMtPer::empty()
        .insert(1)
        .insert(2)
        .insert(3)
        .insert(4)
        .insert(5);
    let evens = s.filter(|x| x % 2 == 0);

    assert_eq!(evens.size(), 2);
    assert!(!evens.find(&1));
    assert!(evens.find(&2));
    assert!(!evens.find(&3));
    assert!(evens.find(&4));
    assert!(!evens.find(&5));
}

#[test]
fn test_clone() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = s1.clone();

    assert_eq!(s1.size(), s2.size());
    assert!(s2.find(&1));
    assert!(s2.find(&2));
    assert!(s2.find(&3));
}
