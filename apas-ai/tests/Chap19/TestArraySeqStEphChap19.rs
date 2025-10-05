//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStEphSLit};

#[test]
fn test_arrayseqstephslit_macro_functionality() {
    // Test empty sequence creation
    let empty: ArraySeqStEphS<i32> = ArraySeqStEphSLit![];
    assert_eq!(empty.length(), 0);
    
    // Test sequence creation with elements
    let with_data: ArraySeqStEphS<i32> = ArraySeqStEphSLit![1, 2, 3];
    assert_eq!(with_data.length(), 3);
    assert_eq!(*with_data.nth(0), 1);
    assert_eq!(*with_data.nth(1), 2);
    assert_eq!(*with_data.nth(2), 3);
}

#[test]
fn test_ephemeral_arrayseq_basic() {
    let mut s: ArraySeqStEphS<N> = ArraySeqStEphSLit![1; 3];
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 1);
    let _ = s.update(Pair(1, 9));
    assert_eq!(*s.nth(1), 9);
}

#[test]
fn test_ephemeral_ch18_map_append_filter() {
    let a = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i, 5);
    let m = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::map(&a, &|x| x + 1);
    let c = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&a, &m);
    assert_eq!(c.length(), 10);
    let evens =
        <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::filter(&c, &|x| if *x % 2 == 0 { true } else { false });
    assert!(evens.length() > 0);
}

#[test]
fn test_iterators_collect() {
    let s = ArraySeqStEphSLit![1, 2, 3];
    let collected: Vec<N> = s.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

// ========== Additional Comprehensive Tests for Untested Functions ==========

#[test]
fn test_arrayseqsteph_trait_empty() {
    let empty = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::empty();
    assert_eq!(empty.length(), 0);
    assert!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isEmpty(&empty));
}

#[test]
fn test_arrayseqsteph_trait_new() {
    let seq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 42);
    }
}

#[test]
fn test_arrayseqsteph_trait_singleton() {
    let seq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::singleton(99);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 99);
    assert!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isSingleton(&seq));
}

#[test]
fn test_arrayseqsteph_trait_length() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::length(&seq), 4);
}

#[test]
fn test_arrayseqsteph_trait_nth() {
    let seq = ArraySeqStEphSLit![10, 20, 30];
    assert_eq!(*<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&seq, 0), 10);
    assert_eq!(*<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&seq, 2), 30);
}

#[test]
fn test_arrayseqsteph_trait_tabulate() {
    let seq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::tabulate(&|i| (i * 3) as i32, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(2), 6);
    assert_eq!(*seq.nth(4), 12);
}

#[test]
fn test_arrayseqsteph_trait_map() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    let doubled = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_arrayseqsteph_trait_filter() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4, 5, 6];
    let evens = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::filter(&seq, &|x| *x % 2 == 0);
    assert_eq!(evens.length(), 3);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
    assert_eq!(*evens.nth(2), 6);
}

#[test]
fn test_arrayseqsteph_trait_append() {
    let a = ArraySeqStEphSLit![1, 2];
    let b = ArraySeqStEphSLit![3, 4, 5];
    let c = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(*c.nth(0), 1);
    assert_eq!(*c.nth(4), 5);
}

#[test]
fn test_arrayseqsteph_trait_append_select() {
    let a = ArraySeqStEphSLit![10, 20];
    let b = ArraySeqStEphSLit![30, 40];
    let c = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::append_select(&a, &b);
    assert_eq!(c.length(), 4);
    assert_eq!(*c.nth(0), 10);
    assert_eq!(*c.nth(3), 40);
}

#[test]
fn test_arrayseqsteph_trait_flatten() {
    let seq1 = ArraySeqStEphSLit![1, 2];
    let seq2 = ArraySeqStEphSLit![3, 4];
    let nested = ArraySeqStEphSLit![seq1, seq2];
    let flat = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::flatten(&nested);
    assert_eq!(flat.length(), 4);
    assert_eq!(*flat.nth(0), 1);
    assert_eq!(*flat.nth(3), 4);
}

#[test]
fn test_arrayseqsteph_trait_reduce() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    let sum = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqsteph_trait_scan() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    let (prefix_sums, total) = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::scan(&seq, &|a, b| a + b, 0);
    assert_eq!(prefix_sums.length(), 4);
    assert_eq!(total, 10);
}

#[test]
fn test_arrayseqsteph_trait_iterate() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    let sum = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqsteph_trait_inject() {
    let base = ArraySeqStEphSLit![0, 0, 0, 0, 0];
    let updates = vec![(1, 10), (3, 30)];
    let result = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::inject(&base, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(1), 10);
    assert_eq!(*result.nth(3), 30);
}

#[test]
fn test_arrayseqsteph_trait_select() {
    let a = ArraySeqStEphSLit![1, 2];
    let b = ArraySeqStEphSLit![3, 4];
    let result = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::select(&a, &b, 1);
    assert_eq!(result, Some(2));
    let result2 = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::select(&a, &b, 2);
    assert_eq!(result2, Some(3));
}

#[test]
fn test_arrayseqsteph_trait_deflate() {
    let result = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::deflate(&|x| *x > 0, &5);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 5);

    let empty = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::deflate(&|x| *x < 0, &5);
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_arrayseqsteph_trait_update() {
    let seq = ArraySeqStEphSLit![1, 2, 3];
    let updated = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::update(&seq, 1, 99);
    assert_eq!(*updated.nth(1), 99);
}

#[test]
fn test_arrayseqsteph_trait_subseq_copy() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let sub = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::subseq_copy(&seq, 1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 4);
}

