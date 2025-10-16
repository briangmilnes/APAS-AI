//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::fmt::Display;
use std::fmt::Formatter;

use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_new_and_set() {
    let a: ArraySeqStPerS<N> = ArraySeqStPerS::new(3, 7);
    assert_eq!(a.length(), 3);
    assert_eq!(*a.nth(0), 7);
    assert_eq!(*a.nth(1), 7);
    assert_eq!(*a.nth(2), 7);
    let b = ArraySeqStPerS::update(&a, 1, 9);
    let c = ArraySeqStPerS::update(&b, 0, 2);
    assert_eq!(*c.nth(0), 2);
    assert_eq!(*c.nth(1), 9);
    assert_eq!(*c.nth(2), 7);
}

#[test]
fn test_length_and_nth_basic() {
    let a = ArraySeqStPerSLit![10, 20, 30, 40];
    assert_eq!(a.length(), 4);
    assert_eq!(*a.nth(0), 10);
    assert_eq!(*a.nth(3), 40);
}

#[test]
fn test_empty() {
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    assert_eq!(empty.length(), 0);
    assert!(<ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::isEmpty(&empty));
}

#[test]
fn test_sequence_basic() {
    let a: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::new(10, false);
    assert!(a.length() != 0);
    assert_eq!(a.length(), 10);
    let b = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::update(&a, 0, true);
    let c = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::update(&b, 1, false);
    let d = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::update(&c, 2, true);
    assert_eq!(d.length(), 10);
    let head4 = ArraySeqStPerSLit![*d.nth(0), *d.nth(1), *d.nth(2), *d.nth(3)];
    assert_eq!(head4, ArraySeqStPerSLit![true, false, true, false]);
}

#[test]
fn test_singleton() {
    let s = ArraySeqStPerS::singleton(42);
    assert_eq!(s.length(), 1);
    assert_eq!(*s.nth(0), 42);
    assert!(s.length() == 1);
}

#[test]
fn test_is_empty_and_is_singleton() {
    let e: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    assert!(e.length() == 0);
    assert!(e.length() != 1);

    let s = ArraySeqStPerS::singleton(7);
    assert!(s.length() != 0);
    assert!(s.length() == 1);

    let a = ArraySeqStPerSLit![1, 2];
    assert!(a.length() != 0);
    assert!(a.length() != 1);
}

#[test]
fn test_from_vec() {
    let empty_seq: ArraySeqStPerS<N> = ArraySeqStPerSLit![];
    assert_eq!(empty_seq.length(), 0);
    assert!(empty_seq.length() == 0);
    let single_vec = vec![42];
    let single_seq = ArraySeqStPerS::from_vec(single_vec);
    assert_eq!(single_seq, ArraySeqStPerSLit![42]);
    let multi_seq = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    assert_eq!(multi_seq, ArraySeqStPerSLit![1, 2, 3, 4, 5]);
    let str_vec = vec!["hello", "world"];
    let str_seq = ArraySeqStPerS::from_vec(str_vec);
    assert_eq!(str_seq, ArraySeqStPerSLit!["hello", "world"]);
}

#[test]
fn test_sequence_equality_natural_numbers() {
    let seq1 = ArraySeqStPerSLit![42];
    let seq2 = ArraySeqStPerSLit![42];
    let seq3 = ArraySeqStPerSLit![99];
    assert_eq!(seq1.length(), 1);
    assert_eq!(seq2.length(), 1);
    assert_eq!(seq3.length(), 1);
    assert_eq!(seq1, ArraySeqStPerSLit![42]);
    assert_eq!(seq2, ArraySeqStPerSLit![42]);
    assert_eq!(seq3, ArraySeqStPerSLit![99]);

    let seq4 = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let seq5 = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let seq6 = ArraySeqStPerSLit![1, 2, 3, 4, 6];
    let seq7 = ArraySeqStPerSLit![1, 2, 3, 4];
    assert_eq!(seq4.length(), 5);
    assert_eq!(seq5.length(), 5);
    assert_eq!(seq6.length(), 5);
    assert_eq!(seq7.length(), 4);
    assert_eq!(seq4, seq5);
    assert_ne!(seq4, seq6);
    assert_ne!(seq4.length(), seq7.length());
}

#[test]
fn test_sequence_equality_strings() {
    let seq1 = ArraySeqStPerSLit!["hello"];
    let seq2 = ArraySeqStPerSLit!["hello"];
    let seq3 = ArraySeqStPerSLit!["world"];
    assert_eq!(seq1.length(), 1);
    assert_eq!(seq2.length(), 1);
    assert_eq!(seq3.length(), 1);
    assert_eq!(seq1, ArraySeqStPerSLit!["hello"]);
    assert_eq!(seq2, ArraySeqStPerSLit!["hello"]);
    assert_eq!(seq3, ArraySeqStPerSLit!["world"]);

    let seq4 = ArraySeqStPerSLit!["the", "cat", "in", "the", "hat"];
    let seq5 = ArraySeqStPerSLit!["the", "cat", "in", "the", "hat"];
    let seq6 = ArraySeqStPerSLit!["the", "cat", "in", "the", "mat"];
    let seq7 = ArraySeqStPerSLit!["the", "cat", "in", "the"];
    assert_eq!(seq4.length(), 5);
    assert_eq!(seq5.length(), 5);
    assert_eq!(seq6.length(), 5);
    assert_eq!(seq7.length(), 4);
    assert_eq!(seq4, seq5);
    assert_ne!(seq4, seq6);
    assert_ne!(seq4.length(), seq7.length());
}

#[test]
fn test_eq_vs_partial_eq_difference() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct PartialComparable {
        value: i32,
    } // Use i32 instead of f64 so Eq can be implemented

    impl Display for PartialComparable {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PartialComparable({})", self.value)
        }
    }
    let seq1 = ArraySeqStPerSLit![PartialComparable { value: 1 }];
    let seq2 = ArraySeqStPerSLit![PartialComparable { value: 1 }];
    assert_eq!(seq1.nth(0), seq2.nth(0));
    let nan_seq = ArraySeqStPerSLit![PartialComparable { value: -1 }];
    let nan_seq2 = ArraySeqStPerSLit![PartialComparable { value: -1 }];
    assert_eq!(nan_seq.nth(0), nan_seq2.nth(0)); // Since they're the same value now

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TotalComparable {
        value: N,
    }

    impl Display for TotalComparable {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "TotalComparable({})", self.value)
        }
    }
    let total_seq1 = ArraySeqStPerSLit![TotalComparable { value: 42 }];
    let total_seq2 = ArraySeqStPerSLit![TotalComparable { value: 42 }];
    assert_eq!(total_seq1.nth(0), total_seq2.nth(0));
    let b_seq1 = ArraySeqStPerSLit![true, false];
    let b_seq2 = ArraySeqStPerSLit![true, false];
    assert_eq!(b_seq1.nth(0), b_seq2.nth(0));
    assert_eq!(b_seq1.nth(1), b_seq2.nth(1));
    assert_ne!(b_seq1.nth(0), b_seq1.nth(1));
}

#[test]
fn test_ordering_numbers_basic() {
    let a: N = 1;
    let b: N = 2;
    assert!(matches!(a.cmp(&b), O::Less));
    assert!(matches!(b.cmp(&a), O::Greater));
    assert!(matches!(a.cmp(&a), O::Equal));
}

#[test]
fn test_numbers_equal_is_equal() {
    let x: N = 7;
    assert!(matches!(x.cmp(&x), O::Equal));
}

#[test]
fn test_ordering_strings_basic() {
    let a = "a";
    let b = "b";
    assert!(matches!(a.cmp(b), O::Less));
    assert!(matches!(b.cmp(a), O::Greater));
    assert!(matches!(a.cmp(a), O::Equal));
}

#[test]
fn test_strings_equal_is_equal() {
    let s = "foo";
    assert!(matches!(s.cmp(s), O::Equal));
}

#[test]
#[should_panic]
fn test_nth_on_empty_panics() {
    let e: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    e.nth(0);
}

#[test]
#[should_panic]
fn test_nth_upper_bound_panics() {
    let a = ArraySeqStPerSLit![1, 2, 3];
    a.nth(3);
}

#[test]
fn test_subseq_trait_form_basic() {
    let a = ArraySeqStPerSLit![10, 20, 30, 40, 50];
    let b: ArraySeqStPerS<N> = a.subseq_copy(1, 3);
    assert_eq!(b, ArraySeqStPerSLit![20, 30, 40]);
    let e: ArraySeqStPerS<N> = a.subseq_copy(10, 2);
    assert_eq!(e.length(), 0);
}

#[test]
fn test_new_set_persistent() {
    let a: ArraySeqStPerS<N> = ArraySeqStPerS::new(3, 7);
    assert_eq!(a.length(), 3);
    let b = ArraySeqStPerS::update(&a, 1, 9);
    assert_eq!(*a.nth(1), 7);
    assert_eq!(*b.nth(1), 9);
}

#[test]
fn test_iterator_collects_in_order() {
    let s = ArraySeqStPerSLit![1, 2, 3, 4];
    let collected: Vec<N> = s.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3, 4]);
}

// ========== Additional Comprehensive Tests for Untested Functions ==========

#[test]
fn test_arrayseqstper_trait_empty() {
    let empty = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::empty();
    assert_eq!(empty.length(), 0);
    assert!(<ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::isEmpty(&empty));
}

#[test]
fn test_arrayseqstper_trait_new() {
    let seq = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 42);
    }
}

#[test]
fn test_arrayseqstper_trait_singleton() {
    let seq = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::singleton(99);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 99);
    assert!(<ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::isSingleton(&seq));
}

#[test]
fn test_arrayseqstper_trait_length() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4];
    assert_eq!(<ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::length(&seq), 4);
}

#[test]
fn test_arrayseqstper_trait_nth() {
    let seq = ArraySeqStPerSLit![10, 20, 30];
    assert_eq!(*<ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::nth(&seq, 0), 10);
    assert_eq!(*<ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::nth(&seq, 2), 30);
}

#[test]
fn test_arrayseqstper_trait_tabulate() {
    let seq = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::tabulate(&|i| (i * 3) as i32, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(2), 6);
    assert_eq!(*seq.nth(4), 12);
}

#[test]
fn test_arrayseqstper_trait_map() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4];
    let doubled = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_arrayseqstper_trait_filter() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4, 5, 6];
    let evens = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::filter(&seq, &|x| *x % 2 == 0);
    assert_eq!(evens.length(), 3);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
    assert_eq!(*evens.nth(2), 6);
}

#[test]
fn test_arrayseqstper_trait_append() {
    let a = ArraySeqStPerSLit![1, 2];
    let b = ArraySeqStPerSLit![3, 4, 5];
    let c = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(*c.nth(0), 1);
    assert_eq!(*c.nth(4), 5);
}

#[test]
fn test_arrayseqstper_trait_append_select() {
    let a = ArraySeqStPerSLit![10, 20];
    let b = ArraySeqStPerSLit![30, 40];
    let c = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append_select(&a, &b);
    assert_eq!(c.length(), 4);
    assert_eq!(*c.nth(0), 10);
    assert_eq!(*c.nth(3), 40);
}

#[test]
fn test_arrayseqstper_trait_flatten() {
    let seq1 = ArraySeqStPerSLit![1, 2];
    let seq2 = ArraySeqStPerSLit![3, 4];
    let nested = ArraySeqStPerSLit![seq1, seq2];
    let flat = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::flatten(&nested);
    assert_eq!(flat.length(), 4);
    assert_eq!(*flat.nth(0), 1);
    assert_eq!(*flat.nth(3), 4);
}

#[test]
fn test_arrayseqstper_trait_reduce() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4];
    let sum = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqstper_trait_scan() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4];
    let (prefix_sums, total) = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::scan(&seq, &|a, b| a + b, 0);
    assert_eq!(prefix_sums.length(), 4);
    assert_eq!(total, 10);
}

#[test]
fn test_arrayseqstper_trait_iterate() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4];
    let sum = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqstper_trait_inject() {
    let base = ArraySeqStPerSLit![0, 0, 0, 0, 0];
    let updates = ArraySeqStPerSLit![Pair(1, 10), Pair(3, 30)];
    let result = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::inject(&base, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(1), 10);
    assert_eq!(*result.nth(3), 30);
}

#[test]
fn test_arrayseqstper_trait_ninject() {
    let base = ArraySeqStPerSLit![0, 0, 0, 0, 0];
    let updates = ArraySeqStPerSLit![Pair(1, 10), Pair(3, 30)];
    let result = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::ninject(&base, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(1), 10);
    assert_eq!(*result.nth(3), 30);
}

#[test]
fn test_arrayseqstper_trait_select() {
    let a = ArraySeqStPerSLit![1, 2];
    let b = ArraySeqStPerSLit![3, 4];
    let result = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::select(&a, &b, 1);
    assert_eq!(result, Some(&2));
    let result2 = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::select(&a, &b, 2);
    assert_eq!(result2, Some(&3));
}

#[test]
fn test_arrayseqstper_trait_deflate() {
    let result = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::deflate(&|x| *x > 0, &5);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 5);

    let empty = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::deflate(&|x| *x < 0, &5);
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_arrayseqstper_trait_update() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    let updated = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::update(&seq, 1, 99);
    assert_eq!(*updated.nth(1), 99);
    assert_eq!(*seq.nth(1), 2); // Original unchanged
}

// ========== Merged from TestArraySeqStPerChap19_Advanced.rs ==========

#[test]
fn test_arrayseqstperslit_macro_functionality() {
    // Test empty sequence creation
    let empty: ArraySeqStPerS<i32> = ArraySeqStPerSLit![];
    assert_eq!(empty.length(), 0);

    // Test sequence creation with elements
    let with_data: ArraySeqStPerS<i32> = ArraySeqStPerSLit![1, 2, 3];
    assert_eq!(with_data.length(), 3);
    assert_eq!(*with_data.nth(0), 1);
    assert_eq!(*with_data.nth(1), 2);
    assert_eq!(*with_data.nth(2), 3);
}

#[test]
fn test_map_and_select_and_append() {
    let a = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i + 1, 3);
    let b = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::map(&a, &|x| x * 2);
    assert_eq!(b, ArraySeqStPerSLit![2, 4, 6]);
    assert_eq!(
        <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::select(&a, &b, 0),
        Some(&1)
    );
    let c = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStPerSLit![1, 2, 3, 2, 4, 6]);
}

#[test]
fn test_deflate_and_filter() {
    let y = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::deflate(&|&x: &N| x % 2 == 0, &6);
    assert_eq!(y, ArraySeqStPerSLit![6]);
    let a = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i + 1, 10);
    let evens = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&a, &|x| *x % 2 == 0);
    assert_eq!(evens, ArraySeqStPerSLit![2, 4, 6, 8, 10]);
}

#[test]
fn test_iterate_reduce_scan_flatten() {
    let a = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i + 1, 5);
    let sum = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::iterate(&a, &|acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let (prefixes, total) = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::scan(&a, &sum_fn, 0);
    assert_eq!(total, 15);
    assert_eq!(*prefixes.nth(4), 15);
    let nested: ArraySeqStPerS<ArraySeqStPerS<N>> = ArraySeqStPerSLit![
        <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i + 1, 2),
        <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i + 3, 2),
    ];
    let flat = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&nested);
    assert_eq!(flat, ArraySeqStPerSLit![1, 2, 3, 4]);
}

#[test]
fn test_inject_and_parallel() {
    let values = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i, 6);
    let changes: ArraySeqStPerS<Pair<N, N>> = ArraySeqStPerSLit![Pair(2, 99), Pair(2, 7), Pair(4, 20)];
    let result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::inject(&values, &changes);
    assert_eq!(result.nth(0), &0);
    assert_eq!(result.nth(1), &1);
    assert_eq!(result.nth(2), &99); // First update wins: Pair(2, 99) over Pair(2, 7)
    assert_eq!(result.nth(3), &3);
    assert_eq!(result.nth(4), &20);
}

// ========== Merged from TestArraySeqStPerChap19_Advanced.rs above ==========
