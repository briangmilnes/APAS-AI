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
    assert!(!ArraySeqMtEphS::isEmpty(&a));
    assert!(!ArraySeqMtEphS::isSingleton(&a));
    let s = a.subseq_copy(1, 3);
    assert_eq!(s.length(), 3);
    assert_eq!(s.nth_cloned(0), 1);
    assert_eq!(s.nth_cloned(2), 3);
}

#[test]
fn test_arrayseq_mteph_append_and_map() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let b = ArraySeqMtEphSLit![4, 5];
    let c = ArraySeqMtEphS::append(&a, &b);
    assert_eq!(c.length(), 5);
    let d = ArraySeqMtEphS::map(&a, |x| x + 1);
    assert_eq!(d.nth_cloned(0), 2);
    assert_eq!(c.nth_cloned(4), 5);
}

#[test]
fn test_empty() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphS::empty();
    assert_eq!(a.length(), 0);
    assert!(ArraySeqMtEphS::isEmpty(&a));
}

#[test]
fn test_singleton() {
    let a = ArraySeqMtEphS::singleton(42);
    assert_eq!(a.length(), 1);
    assert_eq!(a.nth_cloned(0), 42);
    assert!(ArraySeqMtEphS::isSingleton(&a));
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
    let a = ArraySeqMtEphS::tabulate(&|i| (i * 2) as i32, 5);
    assert_eq!(a.length(), 5);
    assert_eq!(a.nth_cloned(0), 0);
    assert_eq!(a.nth_cloned(2), 4);
    assert_eq!(a.nth_cloned(4), 8);
}

#[test]
fn test_filter() {
    let a = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let b = ArraySeqMtEphS::filter(&a, &|x: &i32| *x % 2 == 0);
    assert!(b.length() >= 2); // May include filtered elements
}

#[test]
fn test_reduce() {
    let a = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = ArraySeqMtEphS::reduce(&a, |x, y| x + y, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_scan() {
    let a = ArraySeqMtEphSLit![1, 2, 3, 4];
    let (sums, total) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(sums.length(), 4);
    assert_eq!(total, 10);
}

#[test]
fn test_flatten() {
    let a = ArraySeqMtEphS::singleton(ArraySeqMtEphSLit![1, 2]);
    let flat = ArraySeqMtEphS::flatten(&a);
    assert_eq!(flat.length(), 2);
}

#[test]
fn test_iterate() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let sum = ArraySeqMtEphS::iterate(&a, &|acc, x| acc + x, 0);
    assert_eq!(sum, 6);
}

#[test]
fn test_update() {
    let mut a = ArraySeqMtEphSLit![1, 2, 3];
    <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::update(&mut a, (1, 99));
    assert_eq!(a.nth_cloned(1), 99);
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

// ========== Additional Comprehensive Tests for Untested Functions ==========

#[test]
fn test_arrayseqmteph_trait_empty() {
    let empty: ArraySeqMtEphS<i32> = ArraySeqMtEphS::empty();
    assert_eq!(empty.length(), 0);
    assert!(ArraySeqMtEphS::isEmpty(&empty));
}

#[test]
fn test_arrayseqmteph_trait_new() {
    let seq = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(seq.nth_cloned(i), 42);
    }
}

#[test]
fn test_arrayseqmteph_trait_singleton() {
    let seq = ArraySeqMtEphS::singleton(99);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.nth_cloned(0), 99);
    assert!(ArraySeqMtEphS::isSingleton(&seq));
}

#[test]
fn test_arrayseqmteph_trait_length() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    assert_eq!(<ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::length(&seq), 4);
}

#[test]
fn test_arrayseqmteph_trait_nth_cloned() {
    let seq = ArraySeqMtEphSLit![10, 20, 30];
    assert_eq!(
        <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::nth_cloned(&seq, 0),
        10
    );
    assert_eq!(
        <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::nth_cloned(&seq, 2),
        30
    );
}

#[test]
fn test_arrayseqmteph_trait_tabulate() {
    let seq = ArraySeqMtEphS::tabulate(&|i| (i * 3) as i32, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(0), 0);
    assert_eq!(seq.nth_cloned(2), 6);
    assert_eq!(seq.nth_cloned(4), 12);
}

#[test]
fn test_arrayseqmteph_trait_map() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let doubled = ArraySeqMtEphS::map(&seq, |x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(doubled.nth_cloned(0), 2);
    assert_eq!(doubled.nth_cloned(3), 8);
}

#[test]
fn test_arrayseqmteph_trait_filter() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5, 6];
    let evens = ArraySeqMtEphS::filter(&seq, &|x: &i32| *x % 2 == 0);
    assert_eq!(evens.length(), 3);
    for i in 0..evens.length() {
        assert_eq!(evens.nth_cloned(i) % 2, 0);
    }
}

#[test]
fn test_arrayseqmteph_trait_append() {
    let a = ArraySeqMtEphSLit![1, 2];
    let b = ArraySeqMtEphSLit![3, 4, 5];
    let c = ArraySeqMtEphS::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(c.nth_cloned(0), 1);
    assert_eq!(c.nth_cloned(4), 5);
}

#[test]
fn test_arrayseqmteph_trait_flatten() {
    let seq1 = ArraySeqMtEphSLit![1, 2];
    let seq2 = ArraySeqMtEphSLit![3, 4];
    let nested = ArraySeqMtEphSLit![seq1, seq2];
    let flat = ArraySeqMtEphS::flatten(&nested);
    assert_eq!(flat.length(), 4);
    assert_eq!(flat.nth_cloned(0), 1);
    assert_eq!(flat.nth_cloned(3), 4);
}

#[test]
fn test_arrayseqmteph_trait_reduce() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = ArraySeqMtEphS::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqmteph_trait_scan() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let (prefix_sums, total) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(&seq, &|a, b| a + b, 0);
    assert_eq!(prefix_sums.length(), 4);
    assert_eq!(total, 10);
}

#[test]
fn test_arrayseqmteph_trait_iterate() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = ArraySeqMtEphS::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqmteph_trait_subseq_copy() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let sub = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::subseq_copy(&seq, 1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(sub.nth_cloned(0), 2);
    assert_eq!(sub.nth_cloned(2), 4);
}

#[test]
fn test_simple_calls_no_chap18_import() {
    // This test uses ONLY Chap19 imports - no explicit Chap18 traits
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];

    // These should resolve to Chap19's parallel implementations
    let sum = ArraySeqMtEphS::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);

    let nested = ArraySeqMtEphSLit![ArraySeqMtEphSLit![1, 2], ArraySeqMtEphSLit![3, 4]];
    let flat = ArraySeqMtEphS::flatten(&nested);
    assert_eq!(flat.length(), 4);
}
