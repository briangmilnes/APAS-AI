//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetMtPer.

use apas_ai::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_ai::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerS;
use apas_ai::AVLTreeSetMtPerLit;
use apas_ai::Types::Types::*;

#[test]
fn test_avltreesetmtperlit_macro_type_safety() {
    // Test empty set creation with explicit type
    let empty: AVLTreeSetMtPer<i32> = AVLTreeSetMtPerLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.find(&42));
    
    // Test multi-element set creation
    let multi: AVLTreeSetMtPer<i32> = AVLTreeSetMtPerLit![1, 2, 3];
    assert_eq!(multi.size(), 3);
    assert!(multi.find(&1));
    assert!(multi.find(&2));
    assert!(multi.find(&3));
    assert!(!multi.find(&4));
}

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

#[test]
fn test_union_extended() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2);
    let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3);
    let u = s1.union(&s2);
    assert_eq!(u.size(), 3);
    assert!(u.find(&1));
    assert!(u.find(&2));
    assert!(u.find(&3));
}

#[test]
fn test_intersection_extended() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3).insert(4);
    let i = s1.intersection(&s2);
    assert_eq!(i.size(), 2);
    assert!(i.find(&2));
    assert!(i.find(&3));
}

#[test]
fn test_difference_extended() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = AVLTreeSetMtPer::empty().insert(2);
    let d = s1.difference(&s2);
    assert_eq!(d.size(), 2);
    assert!(d.find(&1));
    assert!(d.find(&3));
}

#[test]
fn test_delete_operation() {
    let s = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = s.delete(&2);
    assert_eq!(s2.size(), 2);
    assert!(!s2.find(&2));
    assert!(s2.find(&1));
    assert!(s2.find(&3));
}

#[test]
fn test_persistence_delete() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = s1.delete(&2);
    assert!(s1.find(&2)); // Original unchanged
    assert!(!s2.find(&2)); // New set without 2
}

#[test]
fn test_large_set() {
    let mut s = AVLTreeSetMtPer::empty();
    for i in 0..100 {
        s = s.insert(i);
    }
    assert_eq!(s.size(), 100);
    assert!(s.find(&0));
    assert!(s.find(&99));
}

#[test]
fn test_negative_numbers() {
    let s = AVLTreeSetMtPer::empty().insert(-5).insert(-3).insert(-7).insert(0);
    assert!(s.find(&-7));
    assert!(s.find(&0));
}

#[test]
fn test_duplicate_insert() {
    let s = AVLTreeSetMtPer::empty().insert(5).insert(5).insert(5);
    assert_eq!(s.size(), 1);
}

#[test]
fn test_intersection_disjoint() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2);
    let s2 = AVLTreeSetMtPer::empty().insert(3).insert(4);
    let i = s1.intersection(&s2);
    assert_eq!(i.size(), 0);
}

#[test]
fn test_union_empty() {
    let s1 = AVLTreeSetMtPer::empty().insert(1);
    let s2 = AVLTreeSetMtPer::empty();
    let u = s1.union(&s2);
    assert_eq!(u.size(), 1);
}
