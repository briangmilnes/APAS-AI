//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap18/ArraySeqMtEph - ALL trait methods

use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Types::Types::*;

// BASE TRAIT TESTS
#[test]
fn test_new() {
    let seq = ArraySeqMtEphS::new(5, 99);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(0), 99);
    assert_eq!(seq.nth_cloned(4), 99);
}

#[test]
fn test_set() {
    let seq = ArraySeqMtEphS::new(3, 0);
    assert!(seq.set(0, 10).is_ok());
    assert!(seq.set(1, 20).is_ok());
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(1), 20);
    assert!(seq.set(10, 99).is_err()); // out of bounds
}

#[test]
fn test_length() {
    let seq = ArraySeqMtEphS::new(10, 0);
    assert_eq!(seq.length(), 10);
    let empty = ArraySeqMtEphS::empty();
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_nth_cloned() {
    let seq = ArraySeqMtEphSLit![10, 20, 30];
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(1), 20);
    assert_eq!(seq.nth_cloned(2), 30);
}

#[test]
fn test_subseq_copy() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let sub = seq.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(sub.nth_cloned(0), 2);
    assert_eq!(sub.nth_cloned(1), 3);
    assert_eq!(sub.nth_cloned(2), 4);
}

#[test]
fn test_update() {
    let mut seq = ArraySeqMtEphSLit![1, 2, 3];
    ArraySeqMtEphS::update(&mut seq, (1, 99));
    assert_eq!(seq.nth_cloned(1), 99);
}

#[test]
fn test_collect() {
    let seq = ArraySeqMtEphSLit![
        Pair(1, 'a'),
        Pair(1, 'b'),
        Pair(2, 'c'),
        Pair(2, 'd'),
        Pair(3, 'e')
    ];
    let collected = ArraySeqMtEphS::collect(&seq, |a: &i32, b: &i32| a.cmp(b));
    assert!(collected.length() > 0);
}

#[test]
fn test_scan() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let (scanned, total) = ArraySeqMtEphS::scan(&seq, &|a: &i32, b: &i32| a + b, 0);
    assert_eq!(total, 10);
    assert_eq!(scanned.length(), 4);
}

#[test]
fn test_from_vec() {
    let vec = vec![10, 20, 30];
    let seq = ArraySeqMtEphS::from_vec(vec);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(2), 30);
}

#[test]
fn test_iter_cloned() {
    let seq = ArraySeqMtEphSLit![5, 10, 15];
    let vec = seq.iter_cloned();
    assert_eq!(vec, vec![5, 10, 15]);
}

#[test]
fn test_to_vec() {
    let seq = ArraySeqMtEphSLit![7, 14, 21];
    let vec = seq.to_vec();
    assert_eq!(vec, vec![7, 14, 21]);
}

// REDEFINABLE TRAIT TESTS
#[test]
fn test_empty() {
    let seq = ArraySeqMtEphS::<i32>::empty();
    assert_eq!(seq.length(), 0);
    assert!(seq.isEmpty());
}

#[test]
fn test_singleton() {
    let seq = ArraySeqMtEphS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.nth_cloned(0), 42);
    assert!(seq.isSingleton());
}

#[test]
fn test_tabulate() {
    let seq = ArraySeqMtEphS::tabulate(&|i| i * 2, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(0), 0);
    assert_eq!(seq.nth_cloned(4), 8);
}

#[test]
fn test_map() {
    let seq = ArraySeqMtEphSLit![1, 2, 3];
    let mapped = ArraySeqMtEphS::map(&seq, |x: &i32| x * 10);
    assert_eq!(mapped.length(), 3);
    assert_eq!(mapped.nth_cloned(0), 10);
    assert_eq!(mapped.nth_cloned(2), 30);
}

#[test]
fn test_append() {
    let a = ArraySeqMtEphSLit![1, 2];
    let b = ArraySeqMtEphSLit![3, 4];
    let c = ArraySeqMtEphS::append(&a, &b);
    assert_eq!(c.length(), 4);
    assert_eq!(c.nth_cloned(0), 1);
    assert_eq!(c.nth_cloned(3), 4);
}

#[test]
fn test_filter() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5, 6];
    let filtered = ArraySeqMtEphS::filter(&seq, &|x: &i32| *x % 2 == 0);
    assert!(filtered.length() == 3);
}

#[test]
fn test_isEmpty() {
    let empty = ArraySeqMtEphS::<i32>::empty();
    assert!(empty.isEmpty());
    let non_empty = ArraySeqMtEphSLit![1];
    assert!(!non_empty.isEmpty());
}

#[test]
fn test_isSingleton() {
    let single = ArraySeqMtEphS::singleton(99);
    assert!(single.isSingleton());
    let empty = ArraySeqMtEphS::<i32>::empty();
    assert!(!empty.isSingleton());
    let multi = ArraySeqMtEphSLit![1, 2];
    assert!(!multi.isSingleton());
}

#[test]
fn test_iterate() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = ArraySeqMtEphS::iterate(&seq, &|acc: &i32, x: &i32| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_reduce() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = ArraySeqMtEphS::reduce(&seq, |a: &i32, b: &i32| a + b, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_flatten() {
    let inner1 = ArraySeqMtEphSLit![1, 2];
    let inner2 = ArraySeqMtEphSLit![3, 4];
    let outer = ArraySeqMtEphSLit![inner1, inner2];
    let flat = ArraySeqMtEphS::flatten(&outer);
    assert_eq!(flat.length(), 4);
    assert_eq!(flat.nth_cloned(0), 1);
    assert_eq!(flat.nth_cloned(3), 4);
}

#[test]
fn test_inject() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let updates = ArraySeqMtEphSLit![Pair(1, 99), Pair(3, 77)];
    let result = ArraySeqMtEphS::inject(&seq, &updates);
    assert_eq!(result.nth_cloned(1), 99);
    assert_eq!(result.nth_cloned(3), 77);
    assert_eq!(result.nth_cloned(0), 1); // unchanged
}

#[test]
fn test_ninject() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let updates = ArraySeqMtEphSLit![Pair(1, 99), Pair(1, 88), Pair(3, 77)];
    let result = ArraySeqMtEphS::ninject(&seq, &updates);
    assert_eq!(result.length(), 5);
    // ninject handles multiple updates per index
}

#[test]
fn test_projInds() {
    let seq = ArraySeqMtEphSLit![10, 20, 30, 40, 50];
    let indices = vec![0, 2, 4];
    let result = ArraySeqMtEphS::projInds(&seq, indices);
    assert_eq!(result.length(), 3);
    assert_eq!(result.nth_cloned(0), 10);
    assert_eq!(result.nth_cloned(1), 30);
    assert_eq!(result.nth_cloned(2), 50);
}

#[test]
fn test_zip() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let b = ArraySeqMtEphSLit![10, 20, 30];
    let zipped = ArraySeqMtEphS::zip(&a, &b);
    assert_eq!(zipped.length(), 3);
    let first = zipped.nth_cloned(0);
    assert_eq!(first.0, 1);
    assert_eq!(first.1, 10);
}

#[test]
fn test_unzip() {
    let pairs = ArraySeqMtEphSLit![Pair(1, 'a'), Pair(2, 'b'), Pair(3, 'c')];
    let (a, b) = ArraySeqMtEphS::unzip(&pairs);
    assert_eq!(a.length(), 3);
    assert_eq!(b.length(), 3);
    assert_eq!(a.nth_cloned(0), 1);
    assert_eq!(b.nth_cloned(0), 'a');
}

#[test]
fn test_nth() {
    let seq = ArraySeqMtEphSLit![100, 200, 300];
    assert_eq!(*seq.nth(0), 100);
    assert_eq!(*seq.nth(2), 300);
}

#[test]
fn test_macro() {
    let seq = ArraySeqMtEphSLit![5, 10, 15, 20];
    assert_eq!(seq.length(), 4);
    assert_eq!(seq.nth_cloned(1), 10);
}

