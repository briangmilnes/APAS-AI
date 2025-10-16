//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SetStEphClean - CLEAN pattern with Self return types.

use apas_ai::Chap05::SetStEphClean::SetStEphClean::*;
use apas_ai::Types::Types::*;

#[test]
fn test_empty_and_size() {
    let s: Set<i32> = SetStEphCleanTrait::empty();
    assert_eq!(s.size(), 0);
}

#[test]
fn test_singleton() {
    let s: Set<i32> = SetStEphCleanTrait::singleton(42);
    assert_eq!(s.size(), 1);
    assert!(s.mem(&42));
    assert!(!s.mem(&43));
}

#[test]
fn test_mem() {
    let s: Set<i32> = SetStEphCleanTrait::singleton(5);
    assert!(s.mem(&5));
    assert!(!s.mem(&10));
}

#[test]
fn test_insert() {
    let mut s: Set<i32> = SetStEphCleanTrait::empty();
    s.insert(1);
    s.insert(2);
    s.insert(3);
    
    assert_eq!(s.size(), 3);
    assert!(s.mem(&1));
    assert!(s.mem(&2));
    assert!(s.mem(&3));
    assert!(!s.mem(&4));
}

#[test]
fn test_union() {
    let mut s1: Set<i32> = SetStEphCleanTrait::empty();
    s1.insert(1).insert(2).insert(3);
    
    let mut s2: Set<i32> = SetStEphCleanTrait::empty();
    s2.insert(3).insert(4).insert(5);
    
    let s3 = s1.union(&s2);
    
    assert_eq!(s3.size(), 5);
    assert!(s3.mem(&1));
    assert!(s3.mem(&2));
    assert!(s3.mem(&3));
    assert!(s3.mem(&4));
    assert!(s3.mem(&5));
}

#[test]
fn test_intersection() {
    let mut s1: Set<i32> = SetStEphCleanTrait::empty();
    s1.insert(1).insert(2).insert(3).insert(4);
    
    let mut s2: Set<i32> = SetStEphCleanTrait::empty();
    s2.insert(3).insert(4).insert(5).insert(6);
    
    let s3 = s1.intersection(&s2);
    
    assert_eq!(s3.size(), 2);
    assert!(s3.mem(&3));
    assert!(s3.mem(&4));
    assert!(!s3.mem(&1));
    assert!(!s3.mem(&5));
}

#[test]
fn test_cartesian_product() {
    let mut s1: Set<i32> = SetStEphCleanTrait::empty();
    s1.insert(1).insert(2);
    
    let mut s2: Set<char> = SetStEphCleanTrait::empty();
    s2.insert('a').insert('b');
    
    let product = s1.CartesianProduct(&s2);
    
    assert_eq!(product.size(), 4);
    assert!(product.mem(&Pair(1, 'a')));
    assert!(product.mem(&Pair(1, 'b')));
    assert!(product.mem(&Pair(2, 'a')));
    assert!(product.mem(&Pair(2, 'b')));
}

#[test]
fn test_iter() {
    let mut s: Set<i32> = SetStEphCleanTrait::empty();
    s.insert(1).insert(2).insert(3);
    
    let mut count = 0;
    for _ in s.iter() {
        count += 1;
    }
    
    assert_eq!(count, 3);
}

#[test]
fn test_clone() {
    let mut s1: Set<i32> = SetStEphCleanTrait::empty();
    s1.insert(1).insert(2).insert(3);
    
    let s2 = s1.clone();
    
    assert_eq!(s1.size(), s2.size());
    assert!(s2.mem(&1));
    assert!(s2.mem(&2));
    assert!(s2.mem(&3));
}

#[test]
fn test_generic_function() {
    fn process_set<S: SetStEphCleanTrait<i32>>(s: &S) -> N {
        s.size()
    }
    
    let mut s: Set<i32> = SetStEphCleanTrait::empty();
    s.insert(10).insert(20).insert(30);
    
    assert_eq!(process_set(&s), 3);
}

