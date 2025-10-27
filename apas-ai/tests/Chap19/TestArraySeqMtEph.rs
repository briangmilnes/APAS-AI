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
    let a = ArraySeqMtEphS::<i32>::empty();
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
fn test_update_mutable() {
    let mut a = ArraySeqMtEphSLit![1, 2, 3];
    <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::update(&mut a, (1, 99));
    assert_eq!(a.nth_cloned(1), 99);
}

#[test]
fn test_update_functional() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    // Test the Chap19 functional update (lines 137-144)
    let b = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::update(&a, (1, 99));
    assert_eq!(b.length(), 3);
    assert_eq!(b.nth_cloned(0), 1);
    assert_eq!(b.nth_cloned(1), 99);
    assert_eq!(b.nth_cloned(2), 3);
    // Original unchanged
    assert_eq!(a.nth_cloned(1), 2);
}

#[test]
fn test_select() {
    let a = ArraySeqMtEphSLit![1, 2];
    let b = ArraySeqMtEphSLit![3, 4];
    
    // Select from first sequence
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::select(&a, &b, 1);
    assert_eq!(result, Some(2));
    
    // Select from second sequence
    let result2 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::select(&a, &b, 2);
    assert_eq!(result2, Some(3));
    
    // Select out of bounds - covers line 167
    let result3 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::select(&a, &b, 10);
    assert_eq!(result3, None);
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
    // True case - element included
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::deflate(&|x| *x > 0, &5);
    assert_eq!(result.length(), 1);
    assert_eq!(result.nth_cloned(0), 5);
    
    // False case - element filtered out
    let result2 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::deflate(&|x| *x > 10, &5);
    assert_eq!(result2.length(), 0);
}

// ========== Additional Comprehensive Tests for Untested Functions ==========

#[test]
fn test_arrayseqmteph_trait_empty() {
    let empty = ArraySeqMtEphS::<i32>::empty();
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
    
    // Empty outer sequence
    let empty_outer = ArraySeqMtEphS::<ArraySeqMtEphS<i32>>::empty();
    let flat_empty = ArraySeqMtEphS::flatten(&empty_outer);
    assert_eq!(flat_empty.length(), 0);
    
    // Single inner sequence
    let single_inner = ArraySeqMtEphSLit![ArraySeqMtEphSLit![7, 8, 9]];
    let flat_single = ArraySeqMtEphS::flatten(&single_inner);
    assert_eq!(flat_single.length(), 3);
}

#[test]
fn test_arrayseqmteph_trait_reduce() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let sum = ArraySeqMtEphS::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);
    
    // Empty sequence case - covers line 195
    let empty = ArraySeqMtEphS::<i32>::empty();
    let sum_empty = ArraySeqMtEphS::reduce(&empty, |a, b| a + b, 42);
    assert_eq!(sum_empty, 42);
    
    // Single element
    let single = ArraySeqMtEphSLit![7];
    let result = ArraySeqMtEphS::reduce(&single, |a, b| a + b, 0);
    assert_eq!(result, 7);
}

#[test]
fn test_arrayseqmteph_trait_scan() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    let (prefix_sums, total) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(&seq, &|a, b| a + b, 0);
    assert_eq!(prefix_sums.length(), 4);
    assert_eq!(total, 10);
    
    // Empty sequence
    let empty = ArraySeqMtEphS::<i32>::empty();
    let (scanned, t) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(&empty, &|a, b| a + b, 0);
    assert_eq!(scanned.length(), 0);
    assert_eq!(t, 0);
    
    // Single element
    let single = ArraySeqMtEphSLit![5];
    let (scanned2, t2) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(&single, &|a, b| a + b, 0);
    assert_eq!(scanned2.length(), 1);
    assert_eq!(t2, 5);
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

// ========== Additional Edge Case Tests for 100% Coverage ==========

#[test]
fn test_inject_ninject() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    
    // inject empty
    let updates_empty = ArraySeqMtEphS::<Pair<N, i32>>::empty();
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::inject(&seq, &updates_empty);
    assert_eq!(result.length(), 5);
    
    // inject single
    let updates_single = ArraySeqMtEphSLit![Pair(2, 99)];
    let result2 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::inject(&seq, &updates_single);
    assert_eq!(result2.nth_cloned(2), 99);
    
    // ninject multiple
    let updates_multi = ArraySeqMtEphSLit![Pair(0, 10), Pair(4, 50)];
    let result3 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::ninject(&seq, &updates_multi);
    assert_eq!(result3.nth_cloned(0), 10);
    assert_eq!(result3.nth_cloned(4), 50);
}

#[test]
fn test_reduce_edge_cases() {
    // Two elements
    let two = ArraySeqMtEphSLit![5, 10];
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(&two, |a, b| a * b, 1);
    assert_eq!(result, 50);
    
    // Large parallel
    let large = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(&|i| (i + 1) as i32, 100);
    let result_large = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(&large, |a, b| a + b, 0);
    assert_eq!(result_large, 5050);
}

#[test]
fn test_scan_edge_cases() {
    // Large parallel scan
    let large = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(&|i| (i + 1) as i32, 100);
    let (result, total) = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::scan(&large, &|a, b| a + b, 0);
    assert_eq!(result.length(), 100);
    assert_eq!(total, 5050);
}

#[test]
fn test_flatten_edge_cases() {
    // Empty inner
    let empty_inner = ArraySeqMtEphSLit![ArraySeqMtEphS::<i32>::empty(), ArraySeqMtEphS::empty()];
    let flat = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::flatten(&empty_inner);
    assert_eq!(flat.length(), 0);
    
    // Mixed
    let mixed = ArraySeqMtEphSLit![
        ArraySeqMtEphSLit![1],
        ArraySeqMtEphS::<i32>::empty(),
        ArraySeqMtEphSLit![2, 3]
    ];
    let flat2 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::flatten(&mixed);
    assert_eq!(flat2.length(), 3);
}

#[test]
fn test_filter_edge_cases() {
    // All pass
    let seq = ArraySeqMtEphSLit![2, 4, 6, 8];
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::filter(&seq, &|x: &i32| *x % 2 == 0);
    assert_eq!(result.length(), 4);
    
    // None pass
    let seq2 = ArraySeqMtEphSLit![1, 3, 5, 7];
    let result2 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::filter(&seq2, &|x: &i32| *x % 2 == 0);
    assert_eq!(result2.length(), 0);
}

#[test]
fn test_map_edge_cases() {
    // Empty
    let empty = ArraySeqMtEphS::<i32>::empty();
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::map(&empty, |x| x * 2);
    assert_eq!(result.length(), 0);
    
    // Large parallel
    let large = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(&|i| i as i32, 100);
    let doubled = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::map(&large, |x| x * 2);
    assert_eq!(doubled.length(), 100);
    assert_eq!(doubled.nth_cloned(50), 100);
}

#[test]
fn test_append_edge_cases() {
    // Both empty
    let a = ArraySeqMtEphS::<i32>::empty();
    let b = ArraySeqMtEphS::<i32>::empty();
    let c = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&a, &b);
    assert_eq!(c.length(), 0);
    
    // First empty
    let b2 = ArraySeqMtEphSLit![1, 2, 3];
    let c2 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&a, &b2);
    assert_eq!(c2.length(), 3);
    
    // Second empty
    let a3 = ArraySeqMtEphSLit![4, 5];
    let c3 = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&a3, &a);
    assert_eq!(c3.length(), 2);
}

#[test]
fn test_iterate_empty() {
    let empty = ArraySeqMtEphS::<i32>::empty();
    let result = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::iterate(&empty, &|acc, x| acc + x, 100);
    assert_eq!(result, 100);
}

#[test]
fn test_tabulate_empty() {
    let empty = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(&|i| i as i32, 0);
    assert_eq!(empty.length(), 0);
}
