//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait as Chap18Trait;
use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Types::Types::*;

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
    let evens = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::filter(&c, &|x| *x % 2 == 0);
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
    let seq = <ArraySeqStEphS<i32> as Chap18Trait<i32>>::new(5, 42);
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
    assert_eq!(<ArraySeqStEphS<i32> as Chap18Trait<i32>>::length(&seq), 4);
}

#[test]
fn test_arrayseqsteph_trait_nth() {
    let seq = ArraySeqStEphSLit![10, 20, 30];
    assert_eq!(*<ArraySeqStEphS<i32> as Chap18Trait<i32>>::nth(&seq, 0), 10);
    assert_eq!(*<ArraySeqStEphS<i32> as Chap18Trait<i32>>::nth(&seq, 2), 30);
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
    let flat = <ArraySeqStEphS<i32> as Chap18Trait<i32>>::flatten(&nested);
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
    let sub = <ArraySeqStEphS<i32> as Chap18Trait<i32>>::subseq(&seq, 1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 4);
}

// ========== Merged from TestArraySeqStEphChap19_Advanced.rs ==========

#[test]
fn test_empty() {
    let e: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    assert_eq!(e.length(), 0);
}

#[test]
fn test_singleton() {
    let s: ArraySeqStEphS<N> = ArraySeqStEphS::singleton(42);
    assert_eq!(s, ArraySeqStEphSLit![42]);
}

#[test]
fn test_map() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 5);
    let b: ArraySeqStEphS<N> = ArraySeqStEphS::map(&a, &|x| x * 2);
    assert_eq!(b, ArraySeqStEphSLit![2, 4, 6, 8, 10]);
}

#[test]
fn test_append() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 3);
    let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 4, 2);
    let c: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &b);
    assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5]);
}

#[test]
fn test_append2() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 3);
    let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 4, 2);
    let c: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &b);
    assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate_true() {
    let y: ArraySeqStEphS<N> = ArraySeqStEphS::deflate(&|&x: &N| x % 2 == 0, &6);
    assert_eq!(y, ArraySeqStEphSLit![6]);
}

#[test]
fn test_deflate_false() {
    let y: ArraySeqStEphS<N> = ArraySeqStEphS::deflate(&|&x: &N| x % 2 == 0, &5);
    assert_eq!(y.length(), 0);
}

#[test]
fn test_filter_even_numbers() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 10);
    let evens: ArraySeqStEphS<N> = ArraySeqStEphS::filter(&a, &|x| *x % 2 == 0);
    assert_eq!(evens, ArraySeqStEphSLit![2, 4, 6, 8, 10]);
}

#[test]
fn test_filter_none() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i, 5);
    let odds_only: ArraySeqStEphS<N> = ArraySeqStEphS::filter(&a, &|x| *x % 2 == 1);
    assert_eq!(odds_only, ArraySeqStEphSLit![1, 3]);
}

#[test]
fn test_update_in_bounds() {
    let mut a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 5);
    let _ = a.update(Pair(2, 99));
    assert_eq!(a, ArraySeqStEphSLit![1, 2, 99, 4, 5]);
}

#[test]
fn test_update_out_of_bounds() {
    let mut a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 3);
    let _ = a.update(Pair(10, 7));
    assert_eq!(a, ArraySeqStEphSLit![1, 2, 3]);
}

#[test]
fn test_isEmpty() {
    let _e: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let e_i32: ArraySeqStEphS<i32> = ArraySeqStEphS::empty();
    assert!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isEmpty(&e_i32));
    let _s: ArraySeqStEphS<N> = ArraySeqStEphS::singleton(7);
    let s_i32: ArraySeqStEphS<i32> = ArraySeqStEphS::singleton(7);
    assert!(!<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isEmpty(&s_i32));
    let _a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i, 2);
    let a_i32: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| i as i32, 2);
    assert!(!<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isEmpty(&a_i32));
}

#[test]
fn test_isSingleton() {
    let _e: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let e_i32: ArraySeqStEphS<i32> = ArraySeqStEphS::empty();
    assert!(!<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isSingleton(&e_i32));
    let _s: ArraySeqStEphS<N> = ArraySeqStEphS::singleton(7);
    let s_i32: ArraySeqStEphS<i32> = ArraySeqStEphS::singleton(7);
    assert!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isSingleton(&s_i32));
    let _a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i, 2);
    let a_i32: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| i as i32, 2);
    assert!(!<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::isSingleton(&a_i32));
}

#[test]
fn test_iterate_sum() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 5);
    let sum: N = ArraySeqStEphS::iterate(&a, &|acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_iterate_concat() {
    let a: ArraySeqStEphS<&str> = ArraySeqStEphS::tabulate(
        &|i| match i {
            | 0 => "a",
            | 1 => "b",
            | _ => "c",
        },
        3,
    );
    let res: String = ArraySeqStEphS::iterate(&a, &|acc: &String, x: &&str| format!("{acc}{x}"), String::new());
    assert_eq!(res, "abc");
}

#[test]
fn test_map_empty() {
    let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|_| 0, 0);
    let m: ArraySeqStEphS<N> = ArraySeqStEphS::map(&e, &|x| x + 1);
    assert_eq!(m.length(), 0);
}

#[test]
fn test_append_with_empty() {
    let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|_| 0, 0);
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 3);
    let left: ArraySeqStEphS<N> = ArraySeqStEphS::append(&e, &a);
    assert_eq!(left, ArraySeqStEphSLit![1, 2, 3]);
    let right: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &e);
    assert_eq!(right, ArraySeqStEphSLit![1, 2, 3]);
}

#[test]
fn test_append2_equivalence() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 3);
    let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 4, 2);
    let c1: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &b);
    let c2: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &b);
    assert_eq!(c1, c2);
}

#[test]
fn test_filter_empty_sequence() {
    let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|_| 0, 0);
    let f: ArraySeqStEphS<N> = ArraySeqStEphS::filter(&e, &|_| true);
    assert_eq!(f.length(), 0);
}

#[test]
fn test_select_boundary() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 2);
    let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 3, 2);
    assert_eq!(ArraySeqStEphS::select(&a, &b, 0), Some(1));
    assert_eq!(ArraySeqStEphS::select(&a, &b, 1), Some(2));
    assert_eq!(ArraySeqStEphS::select(&a, &b, 2), Some(3));
    assert_eq!(ArraySeqStEphS::select(&a, &b, 3), Some(4));
    assert_eq!(ArraySeqStEphS::select(&a, &b, 4), None);
}

#[test]
fn test_subseq_basic() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| (i + 1) * 10, 5);
    let s = a.subseq(1, 3);
    assert_eq!(s, ArraySeqStEphSLit![20, 30, 40]);
}

#[test]
fn test_reduce_sum_basic_ch19() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let r: N = ArraySeqStEphS::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|_| 0, 0);
    let re: N = ArraySeqStEphS::reduce(&e, &sum_fn, 42);
    assert_eq!(re, 42);
    let s: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|_| 7, 1);
    let rs: N = ArraySeqStEphS::reduce(&s, &sum_fn, 0);
    assert_eq!(rs, 7);
}

#[test]
fn test_scan_sum_basic_ch19() {
    let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let (prefixes, total) = ArraySeqStEphS::scan(&a, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 1);
    assert_eq!(*prefixes.nth(4), 15);
    assert_eq!(total, 15);
}

#[test]
fn test_flatten_ch19() {
    let s1: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 1, 2);
    let s2: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(&|i| i + 3, 2);
    let nested: ArraySeqStEphS<ArraySeqStEphS<N>> = ArraySeqStEphSLit![s1, s2];
    let flat: ArraySeqStEphS<N> = ArraySeqStEphS::flatten(&nested);
    assert_eq!(flat, ArraySeqStEphSLit![1, 2, 3, 4]);
}
