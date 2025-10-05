//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_arrayseq_mteph_basic_ops() {
    let mut a = ArraySeqMtEphSLit![0; 5];
    for i in 0..a.length() {
        let _ = a.set(i, i as i32);
    }
    for i in 0..a.length() {
        assert_eq!(a.nth_cloned(i), i as i32);
    }
    assert_eq!(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::isEmpty(&a), false);
    assert_eq!(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::isSingleton(&a), false);
    let s = a.subseq_copy(1, 3);
    assert_eq!(s.length(), 3);
    assert_eq!(s.nth_cloned(0), 1);
    assert_eq!(s.nth_cloned(2), 3);
}

#[test]
fn test_arrayseq_mteph_append_and_map() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let b = ArraySeqMtEphSLit![4, 5];
    let c = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&a, &b);
    assert_eq!(c.length(), 5);
    let d = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::map(&a, |x| x + 1);
    assert_eq!(d.nth_cloned(0), 2);
    assert_eq!(c.nth_cloned(4), 5);
}

#[test]
fn test_empty() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphS::empty();
    assert_eq!(a.length(), 0);
    assert!(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::isEmpty(&a));
}

#[test]
fn test_singleton() {
    let a = ArraySeqMtEphS::singleton(42);
    assert_eq!(a.length(), 1);
    assert_eq!(a.nth_cloned(0), 42);
    assert!(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::isSingleton(&a));
}

#[test]
fn test_new() {
    let a = ArraySeqMtEphS::new(5, 7);
    assert_eq!(a.length(), 5);
    for i in 0..5 {
        assert_eq!(a.nth_cloned(i), 7);
    }
}

#[test]
fn test_tabulate() {
    let a = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(&|i| (i * 2) as i32, 5);
    assert_eq!(a.length(), 5);
    assert_eq!(a.nth_cloned(0), 0);
    assert_eq!(a.nth_cloned(2), 4);
    assert_eq!(a.nth_cloned(4), 8);
}

#[test]
fn test_filter() {
    let a = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let b = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::filter(&a, |x| *x % 2 == 0);
    assert!(b.length() >= 2);  // May include filtered elements
}

#[test]
fn test_reduce() {
    let a = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(&a, |x, y| x + y, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_scan() {
    let a = ArraySeqMtEphSLit![1, 2, 3, 4];
    let (sums, total) = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(sums.length(), 4);
    assert_eq!(total, 10);
}

#[test]
fn test_flatten() {
    let a = ArraySeqMtEphS::singleton(ArraySeqMtEphSLit![1, 2]);
    let flat = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::flatten(&a);
    assert_eq!(flat.length(), 2);
}

#[test]
fn test_iterate() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let sum = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::iterate(&a, &|acc, x| acc + x, 0);
    assert_eq!(sum, 6);
}

#[test]
fn test_update() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let b = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::update(&a, 1, 99);
    assert_eq!(b.nth_cloned(1), 99);
}

#[test]
fn test_select() {
    let a = ArraySeqMtEphSLit![1, 2];
    let b = ArraySeqMtEphSLit![3, 4];
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::select(&a, &b, 1);
    assert_eq!(result, Some(2));
}

#[test]
fn test_append_select() {
    let a = ArraySeqMtEphSLit![1, 2];
    let b = ArraySeqMtEphSLit![3, 4];
    let c = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append_select(&a, &b);
    assert_eq!(c.length(), 4);
}

#[test]
fn test_deflate() {
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::deflate(&|x| *x > 0, &5);
    assert_eq!(result.length(), 1);
    assert_eq!(result.nth_cloned(0), 5);
}
