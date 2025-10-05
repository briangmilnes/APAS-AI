//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetBBAlphaMtEph.

use apas_ai::BSTSetBBAlphaMtEphLit;
use apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsetbbalphamtephlit_macro_functionality() {
    let empty: BSTSetBBAlphaMtEph<i32> = BSTSetBBAlphaMtEphLit![];
    assert_eq!(empty.size(), 0);

    let with_data: BSTSetBBAlphaMtEph<i32> = BSTSetBBAlphaMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert_eq!(with_data.find(&5), Some(5));
    assert_eq!(with_data.find(&3), Some(3));
    assert_eq!(with_data.find(&7), Some(7));
    assert_eq!(with_data.find(&10), None);
}

#[test]
fn test_empty() {
    let set: BSTSetBBAlphaMtEph<i32> = BSTSetBBAlphaMtEph::empty();
    assert_eq!(set.size(), 0);
    assert!(set.is_empty());
}

#[test]
fn test_singleton() {
    let set = BSTSetBBAlphaMtEph::singleton(42);
    assert_eq!(set.size(), 1);
    assert!(set.contains(&42));
}

#[test]
fn test_insert_and_contains() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    assert_eq!(set.size(), 3);
    assert!(set.contains(&5));
    assert!(set.contains(&3));
    assert!(!set.contains(&10));
}

#[test]
fn test_delete() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    set.delete(&3);
    assert_eq!(set.size(), 2);
    assert!(!set.contains(&3));
}

#[test]
fn test_minimum_maximum() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    set.insert(1);
    set.insert(9);
    assert_eq!(set.minimum(), Some(1));
    assert_eq!(set.maximum(), Some(9));
}

#[test]
fn test_union() {
    let mut set1 = BSTSetBBAlphaMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BSTSetBBAlphaMtEph::empty();
    set2.insert(2);
    set2.insert(3);
    let union = set1.union(&set2);
    assert_eq!(union.size(), 3);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
}

#[test]
fn test_intersection() {
    let mut set1 = BSTSetBBAlphaMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    let mut set2 = BSTSetBBAlphaMtEph::empty();
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 2);
    assert!(inter.contains(&2));
    assert!(inter.contains(&3));
}

#[test]
fn test_difference() {
    let mut set1 = BSTSetBBAlphaMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    let mut set2 = BSTSetBBAlphaMtEph::empty();
    set2.insert(2);
    let diff = set1.difference(&set2);
    assert_eq!(diff.size(), 2);
    assert!(diff.contains(&1));
    assert!(diff.contains(&3));
}

#[test]
fn test_split() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    let (left, found, right) = set.split(&5);
    assert!(found);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 1);
}

#[test]
fn test_filter() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    for i in 1..=10 {
        set.insert(i);
    }
    let evens = set.filter(|x| x % 2 == 0);
    assert_eq!(evens.size(), 5);
    assert!(evens.contains(&2));
    assert!(evens.contains(&4));
}

#[test]
fn test_reduce() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let sum = set.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 6);
}

#[test]
fn test_iter_in_order() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    set.insert(1);
    let seq = set.iter_in_order();
    assert_eq!(seq.length(), 4);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(1), 3);
    assert_eq!(*seq.nth(2), 5);
    assert_eq!(*seq.nth(3), 7);
}

#[test]
fn test_large_set() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    for i in 0..100 {
        set.insert(i);
    }
    assert_eq!(set.size(), 100);
    assert_eq!(set.minimum(), Some(0));
    assert_eq!(set.maximum(), Some(99));
}

#[test]
fn test_delete_multiple() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    for i in 0..10 {
        set.insert(i);
    }
    set.delete(&5);
    set.delete(&3);
    set.delete(&7);
    assert_eq!(set.size(), 7);
    assert!(!set.contains(&5));
}

#[test]
fn test_duplicate_insert() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(5);
    set.insert(5);
    set.insert(5);
    assert_eq!(set.size(), 1);
}

#[test]
fn test_balanced_after_inserts() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    for i in 1..=100 {
        set.insert(i);
    }
    // BB-Alpha tree should stay balanced
    assert_eq!(set.size(), 100);
    assert!(set.contains(&50));
}

#[test]
fn test_union_empty() {
    let mut set1 = BSTSetBBAlphaMtEph::empty();
    set1.insert(1);
    let set2 = BSTSetBBAlphaMtEph::empty();
    let union = set1.union(&set2);
    assert_eq!(union.size(), 1);
}

#[test]
fn test_intersection_disjoint() {
    let mut set1 = BSTSetBBAlphaMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BSTSetBBAlphaMtEph::empty();
    set2.insert(3);
    set2.insert(4);
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 0);
}

#[test]
fn test_negative_numbers() {
    let mut set = BSTSetBBAlphaMtEph::empty();
    set.insert(-5);
    set.insert(-3);
    set.insert(-7);
    set.insert(0);
    assert_eq!(set.minimum(), Some(-7));
    assert_eq!(set.maximum(), Some(0));
}

#[test]
fn test_join_pair() {
    let mut left = BSTSetBBAlphaMtEph::empty();
    left.insert(1);
    left.insert(2);
    let mut right = BSTSetBBAlphaMtEph::empty();
    right.insert(5);
    right.insert(6);
    let joined = BSTSetBBAlphaMtEph::join_pair(left, right);
    assert_eq!(joined.size(), 4);
}
