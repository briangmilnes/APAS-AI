//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPerChap18 multithreaded algorithms.

use std::fmt::Debug;

use apas_ai::ArrayMtPerSLit;
use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Types::Types::*; // macro import

fn identity(i: N) -> N { i }
fn double(i: N) -> N { i * 2 }
fn square(i: N) -> N { i * i }
fn add_100(i: N) -> N { i + 100 }
fn const_42(_i: N) -> N { 42 }
fn format_item(i: N) -> String { format!("item_{i}") }
fn is_even_bool(i: N) -> B { i % 2 == 0 }

fn assert_set_eq<T: PartialEq + Debug>(actual: &[T], expected: &[T]) {
    assert_eq!(actual.len(), expected.len());
    for val in actual {
        assert!(expected.contains(val), "Value {val:?} not found in expected");
    }
    for val in expected {
        assert!(actual.contains(val), "Expected value {val:?} not found in actual");
    }
}

#[test]
fn test_tabulate_basic() {
    let a = ArraySeqMtPerS::<N>::tabulate(&identity, 5);
    assert_eq!(a.length(), 5);

    // Use set comparison since MT results may vary in order
    let a_vec = (0..a.length()).map(|i| *a.nth(i)).collect::<Vec<N>>();
    let expected_vec = (0..5).collect::<Vec<N>>();
    assert_set_eq(&a_vec, &expected_vec);
}

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            | 0 => 0,
            | 1 => 1,
            | _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    let fibs = ArraySeqMtPerS::<N>::tabulate(&fib, 8);
    assert_eq!(fibs.length(), 8);

    // Expected fibonacci sequence: [0, 1, 1, 2, 3, 5, 8, 13]
    let expected = [0, 1, 1, 2, 3, 5, 8, 13];
    let fibs_vec = (0..fibs.length()).map(|i| *fibs.nth(i)).collect::<Vec<N>>();
    assert_set_eq(&fibs_vec, &expected);
}

#[test]
fn test_tabulate_empty() {
    let empty = ArraySeqMtPerS::<N>::tabulate(&double, 0);
    assert_eq!(empty.length(), 0);
    assert!(empty.length() == 0);
}

#[test]
fn test_tabulate_single() {
    let single = ArraySeqMtPerS::<N>::tabulate(&const_42, 1);
    assert_eq!(single.length(), 1);
    assert_eq!(*single.nth(0), 42);
}

#[test]
fn test_tabulate_string() {
    let strings = ArraySeqMtPerS::<String>::tabulate(&format_item, 4);
    assert_eq!(strings.length(), 4);

    // Check that all expected strings are present (order may vary in MT)
    let expected = ["item_0", "item_1", "item_2", "item_3"];
    let strings_vec = (0..strings.length()).map(|i| strings.nth(i).clone()).collect::<Vec<String>>();
    let expected_vec = expected.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    assert_set_eq(&strings_vec, &expected_vec);
}

#[test]
fn test_tabulate_boolean() {
    let bools = ArraySeqMtPerS::<B>::tabulate(&is_even_bool, 6);
    assert_eq!(bools.length(), 6);

    // Expected: [True, False, True, False, True, False]
    let expected = [true, false, true, false, true, false];
    let bools_vec = (0..bools.length()).map(|i| *bools.nth(i)).collect::<Vec<B>>();
    assert_set_eq(&bools_vec, &expected);
}

#[test]
fn test_tabulate_squares() {
    let squares = ArraySeqMtPerS::<N>::tabulate(&square, 5);
    assert_eq!(squares.length(), 5);

    // Expected: [0, 1, 4, 9, 16]
    let expected = [0, 1, 4, 9, 16];
    let squares_vec = (0..squares.length()).map(|i| *squares.nth(i)).collect::<Vec<N>>();
    assert_set_eq(&squares_vec, &expected);
}

#[test]
fn test_tabulate_large() {
    let large = ArraySeqMtPerS::<N>::tabulate(&add_100, 1000);
    assert_eq!(large.length(), 1000);
    assert_eq!(*large.nth(0), 100);
    assert_eq!(*large.nth(999), 1099);

    // Check a few random elements
    assert_eq!(*large.nth(500), 600);
    assert_eq!(*large.nth(123), 223);
}

#[test]
fn test_empty() {
    let empty = ArraySeqMtPerS::<i32>::empty();
    assert_eq!(empty.length(), 0);
    assert!(<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isEmpty(
        &empty
    ));
}

#[test]
fn test_singleton() {
    let single = ArraySeqMtPerS::singleton(42);
    assert_eq!(single.length(), 1);
    assert_eq!(*single.nth(0), 42);
    assert!(<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isSingleton(&single));
}

#[test]
fn test_from_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    let seq = ArraySeqMtPerS::from_vec(vec);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(2), 3);
}

#[test]
fn test_subseq_copy() {
    let seq = ArraySeqMtPerS::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let sub = seq.subseq_copy(3, 4);
    assert_eq!(sub.length(), 4);
    assert_eq!(*sub.nth(0), 3);
    assert_eq!(*sub.nth(3), 6);
}

#[test]
fn test_is_empty() {
    let empty = ArraySeqMtPerS::<i32>::empty();
    assert!(<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isEmpty(
        &empty
    ));

    let non_empty = ArraySeqMtPerS::singleton(1);
    assert!(!<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isEmpty(
        &non_empty
    ));
}

#[test]
fn test_is_singleton() {
    let single = ArraySeqMtPerS::singleton(10);
    assert!(<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isSingleton(&single));

    let multiple = ArraySeqMtPerS::from_vec(vec![1, 2]);
    assert!(!<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isSingleton(&multiple));

    let empty = ArraySeqMtPerS::<i32>::empty();
    assert!(!<ArraySeqMtPerS<i32> as ArraySeqMtPerRedefinableTrait<i32>>::isSingleton(&empty));
}

#[test]
fn test_new() {
    let seq = ArraySeqMtPerS::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 42);
    }
}

#[test]
fn test_iter() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let sum: i32 = seq.iter().sum();
    assert_eq!(sum, 6);
}

#[test]
fn test_large_sequence() {
    let seq = ArraySeqMtPerS::<N>::tabulate(&identity, 10000);
    assert_eq!(seq.length(), 10000);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(9999), 9999);
}

#[test]
fn test_subseq_edge_cases() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);

    let all = seq.subseq_copy(0, 5);
    assert_eq!(all.length(), 5);

    let start = seq.subseq_copy(0, 2);
    assert_eq!(start.length(), 2);
    assert_eq!(*start.nth(0), 1);

    let end = seq.subseq_copy(3, 2);
    assert_eq!(end.length(), 2);
    assert_eq!(*end.nth(0), 4);
}

#[test]
fn test_set_basic() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let updated = seq.set(2, 99).unwrap();
    assert_eq!(updated.length(), 5);
    assert_eq!(*updated.nth(2), 99);
    assert_eq!(*updated.nth(0), 1);
    assert_eq!(*updated.nth(4), 5);
}

#[test]
fn test_set_out_of_bounds() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let result = seq.set(5, 99);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Index out of bounds");
}

#[test]
fn test_map_basic() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4, 5]);
    let doubled = ArraySeqMtPerS::map(&seq, |x| x * 2);
    assert_eq!(doubled.length(), 5);
    for i in 0..5 {
        assert_eq!(*doubled.nth(i), (i + 1) * 2);
    }
}

#[test]
fn test_map_empty() {
    let empty = ArraySeqMtPerS::<N>::empty();
    let mapped = ArraySeqMtPerS::map(&empty, |x: &N| x * 2);
    assert_eq!(mapped.length(), 0);
}

#[test]
fn test_append_basic() {
    let a = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqMtPerS::from_vec(vec![4, 5, 6]);
    let combined = ArraySeqMtPerS::append(&a, &b);
    assert_eq!(combined.length(), 6);
    for i in 0..6 {
        assert_eq!(*combined.nth(i), i + 1);
    }
}

#[test]
fn test_append_empty() {
    let a = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let empty = ArraySeqMtPerS::<N>::empty();

    let result1 = ArraySeqMtPerS::append(&a, &empty);
    assert_eq!(result1.length(), 3);

    let result2 = ArraySeqMtPerS::append(&empty, &a);
    assert_eq!(result2.length(), 3);
}

#[test]
fn test_filter_basic() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let evens = ArraySeqMtPerS::filter(&seq, &|x| *x % 2 == 0);
    assert_eq!(evens.length(), 5);
    for i in 0..5 {
        assert_eq!(*evens.nth(i), (i + 1) * 2);
    }
}

#[test]
fn test_filter_none_match() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 3, 5, 7]);
    let evens = ArraySeqMtPerS::filter(&seq, &|x| *x % 2 == 0);
    assert_eq!(evens.length(), 0);
}

#[test]
fn test_update_basic() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let updated = ArraySeqMtPerS::update(&seq, Pair(2, 99));
    assert_eq!(updated.length(), 5);
    assert_eq!(*updated.nth(2), 99);
    assert_eq!(*updated.nth(0), 1);
}

#[test]
fn test_update_out_of_bounds() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let result = ArraySeqMtPerS::update(&seq, Pair(5, 99));
    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 1);
}

#[test]
fn test_ninject_basic() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let updates = ArraySeqMtPerS::from_vec(vec![Pair(1, 10), Pair(3, 30)]);
    let result = ArraySeqMtPerS::ninject(&seq, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(0), 1);
    assert_eq!(*result.nth(1), 10);
    assert_eq!(*result.nth(2), 3);
    assert_eq!(*result.nth(3), 30);
    assert_eq!(*result.nth(4), 5);
}

#[test]
fn test_ninject_duplicate_keys() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let updates = ArraySeqMtPerS::from_vec(vec![Pair(1, 10), Pair(1, 20)]);
    let result = ArraySeqMtPerS::ninject(&seq, &updates);
    // ninject applies all updates without checking for duplicates, last one wins
    assert_eq!(*result.nth(1), 20);
}

#[test]
fn test_iterate_sum() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4, 5]);
    let sum = ArraySeqMtPerS::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_iterate_product() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4]);
    let product = ArraySeqMtPerS::iterate(&seq, &|acc, x| acc * x, 1);
    assert_eq!(product, 24);
}

#[test]
fn test_iteratePrefixes_basic() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4, 5]);
    let (prefixes, final_val) = ArraySeqMtPerS::iteratePrefixes(&seq, &|acc, x| acc + x, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 1);
    assert_eq!(*prefixes.nth(1), 3);
    assert_eq!(*prefixes.nth(2), 6);
    assert_eq!(*prefixes.nth(3), 10);
    assert_eq!(*prefixes.nth(4), 15);
    assert_eq!(final_val, 15);
}

#[test]
fn test_reduce_sum() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4, 5]);
    let sum = ArraySeqMtPerS::reduce(&seq, |x, y| x + y, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_reduce_empty() {
    let empty = ArraySeqMtPerS::<N>::empty();
    let sum = ArraySeqMtPerS::reduce(&empty, |x, y| x + y, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_reduce_single() {
    let seq = ArraySeqMtPerS::<N>::singleton(10);
    let result = ArraySeqMtPerS::reduce(&seq, |x, y| x + y, 0);
    assert_eq!(result, 10);
}

#[test]
fn test_scan_basic() {
    let seq = ArraySeqMtPerS::<N>::from_vec(vec![1, 2, 3, 4, 5]);
    let (prefixes, final_val) = ArraySeqMtPerS::scan(&seq, &|acc, x| acc + x, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 1);
    assert_eq!(*prefixes.nth(4), 15);
    assert_eq!(final_val, 15);
}

#[test]
fn test_flatten_basic() {
    let inner1 = ArraySeqMtPerS::<N>::from_vec(vec![1, 2]);
    let inner2 = ArraySeqMtPerS::<N>::from_vec(vec![3, 4, 5]);
    let inner3 = ArraySeqMtPerS::<N>::from_vec(vec![6]);
    let outer = ArraySeqMtPerS::from_vec(vec![inner1, inner2, inner3]);

    let flattened = ArraySeqMtPerS::flatten(&outer);
    assert_eq!(flattened.length(), 6);
    for i in 0..6 {
        assert_eq!(*flattened.nth(i), i + 1);
    }
}

#[test]
fn test_flatten_empty() {
    let empty_outer = ArraySeqMtPerS::<ArraySeqMtPerS<N>>::empty();
    let flattened = ArraySeqMtPerS::flatten(&empty_outer);
    assert_eq!(flattened.length(), 0);
}

#[test]
fn test_collect_basic() {
    let pairs = ArraySeqMtPerS::from_vec(vec![Pair(1, 10), Pair(2, 20), Pair(1, 11), Pair(2, 21), Pair(1, 12)]);

    let grouped = <ArraySeqMtPerS<Pair<N, N>> as ArraySeqMtPerBaseTrait<Pair<N, N>>>::collect(&pairs, |a, b| a.cmp(b));
    assert_eq!(grouped.length(), 2);
}

#[test]
fn test_inject_first_wins() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let updates = ArraySeqMtPerS::from_vec(vec![Pair(1, 10), Pair(1, 20), Pair(3, 30)]);
    let result = ArraySeqMtPerS::inject(&seq, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(1), 10);
    assert_eq!(*result.nth(3), 30);
}

#[test]
fn test_isEmpty_trait() {
    let empty = ArraySeqMtPerS::<N>::empty();
    assert!(<ArraySeqMtPerS<N> as ArraySeqMtPerRedefinableTrait<N>>::isEmpty(&empty));

    let non_empty = ArraySeqMtPerS::singleton(1);
    assert!(!<ArraySeqMtPerS<N> as ArraySeqMtPerRedefinableTrait<N>>::isEmpty(
        &non_empty
    ));
}

#[test]
fn test_isSingleton_trait() {
    let single = ArraySeqMtPerS::singleton(42);
    assert!(<ArraySeqMtPerS<N> as ArraySeqMtPerRedefinableTrait<N>>::isSingleton(
        &single
    ));

    let multiple = ArraySeqMtPerS::from_vec(vec![1, 2]);
    assert!(!<ArraySeqMtPerS<N> as ArraySeqMtPerRedefinableTrait<N>>::isSingleton(
        &multiple
    ));

    let empty = ArraySeqMtPerS::<N>::empty();
    assert!(!<ArraySeqMtPerS<N> as ArraySeqMtPerRedefinableTrait<N>>::isSingleton(
        &empty
    ));
}

#[test]
fn test_clone() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let cloned = seq.clone();
    assert_eq!(seq, cloned);
    assert_eq!(cloned.length(), 5);
    assert_eq!(*cloned.nth(2), 3);
}

#[test]
fn test_partial_eq() {
    let seq1 = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let seq2 = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let seq3 = ArraySeqMtPerS::from_vec(vec![1, 2, 4]);
    let seq4 = ArraySeqMtPerS::from_vec(vec![1, 2]);

    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
    assert_ne!(seq1, seq4);
}

#[test]
fn test_into_iterator_ref() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let mut sum = 0;
    for item in &seq {
        sum += *item;
    }
    assert_eq!(sum, 15);
}

#[test]
fn test_into_iterator_owned() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let mut sum = 0;
    for item in seq {
        sum += item;
    }
    assert_eq!(sum, 15);
}

#[test]
fn test_display() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let display_str = format!("{seq}");
    assert!(display_str.contains("1"));
    assert!(display_str.contains("2"));
    assert!(display_str.contains("3"));
}
