//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphBaseTrait, ArraySeqStEphRedefinableTrait};
use apas_ai::Types::Types::*; // macro import

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            | 0 => 0,
            | 1 => 1,
            | _ => fib(n - 1) + fib(n - 2),
        }
    }
    let a = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::tabulate(&fib, 10);
    let fib10_head = ArraySeqStEphSLit![
        *a.nth(0),
        *a.nth(1),
        *a.nth(2),
        *a.nth(3),
        *a.nth(4),
        *a.nth(5),
        *a.nth(6),
        *a.nth(7),
        *a.nth(8),
        *a.nth(9)
    ];
    assert_eq!(fib10_head, ArraySeqStEphSLit![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let b = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::map(&a, &|x| x + 1);
    assert_eq!(b, ArraySeqStEphSLit![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = ArraySeqStEphSLit![10, 20, 30, 40, 50];
    let b = <ArraySeqStEphS<N> as ArraySeqStEphBaseTrait<N>>::subseq(&a, 1, 3);
    assert_eq!(b, ArraySeqStEphSLit![20, 30, 40]);
    let c = <ArraySeqStEphS<N> as ArraySeqStEphBaseTrait<N>>::subseq(&a, 2, 0);
    assert_eq!(c.length(), 0);
    let d = <ArraySeqStEphS<N> as ArraySeqStEphBaseTrait<N>>::subseq(&a, 0, 1);
    assert_eq!(d, ArraySeqStEphSLit![10]);
}

#[test]
fn test_append() {
    let a = ArraySeqStEphSLit![1, 2, 3];
    let b = ArraySeqStEphSLit![4, 5, 6];
    let c = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySeqStEphSLit![1, 2, 3];
    let b = ArraySeqStEphSLit![4, 5];
    let c = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5]);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::empty();
    let d = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&ArraySeqStEphSLit![1, 2, 3], &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&empty, &ArraySeqStEphSLit![1, 2, 3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySeqStEphSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::filter(&numbers, &|&x| x % 2 == 0);
    assert_eq!(evens, ArraySeqStEphSLit![2, 4, 6, 8, 10]);
    let odds_only = ArraySeqStEphSLit![1, 3, 5, 7];
    let no_evens = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::filter(&odds_only, &|&x| x % 2 == 0);
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = ArraySeqStEphSLit![1, 2];
    let b = ArraySeqStEphSLit![3, 4, 5];
    let c = ArraySeqStEphSLit![6];
    let sequences = ArraySeqStEphSLit![a, b, c];
    let flattened = <ArraySeqStEphS<N> as ArraySeqStEphBaseTrait<N>>::flatten(&sequences);
    assert_eq!(flattened, ArraySeqStEphSLit![1, 2, 3, 4, 5, 6]);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::empty();
    let mixed = ArraySeqStEphSLit![ArraySeqStEphSLit![1, 2], empty, ArraySeqStEphSLit![3]];
    let mixed_flat = <ArraySeqStEphS<N> as ArraySeqStEphBaseTrait<N>>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySeqStEphSLit![1, 2, 3]);
}

#[test]
fn test_update_sequence() {
    // Eph: update mutates a clone (via update helper)
    let a = ArraySeqStEphSLit!["hello", "world", "test"];
    let mut b = a.clone();
    let _ = b.update(Pair(1, "rust"));
    assert_eq!(b, ArraySeqStEphSLit!["hello", "rust", "test"]);
    let mut c = ArraySeqStEphSLit!["hello", "world", "test"];
    let _ = c.update(Pair(5, "out_of_bounds"));
    assert_eq!(c, ArraySeqStEphSLit!["hello", "world", "test"]);
}

#[test]
fn test_inject_conflicts_last_wins() {
    let a = ArraySeqStEphSLit!["the", "cat", "in", "the", "hat"];
    let updates = ArraySeqStEphSLit![Pair(0, "a"), Pair(2, "on"), Pair(4, "mat")];
    let mut a_mut = a.clone();
    let injected = <ArraySeqStEphS<&str> as ArraySeqStEphBaseTrait<&str>>::inject(&mut a_mut, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(*injected, ArraySeqStEphSLit!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = ArraySeqStEphSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")];
    let mut a_mut2 = a.clone();
    let result_last = <ArraySeqStEphS<&str> as ArraySeqStEphBaseTrait<&str>>::inject(&mut a_mut2, &conflicting_updates);
    assert_eq!(
        *result_last,
        ArraySeqStEphSLit!["second", "updated", "in", "the", "hat"]
    );
}

#[test]
fn test_ninject_conflicts_last_wins() {
    let a = ArraySeqStEphSLit!["the", "cat", "in", "the", "hat"];
    let mut a_mut3 = a.clone();
    let ninjected = <ArraySeqStEphS<&str> as ArraySeqStEphBaseTrait<&str>>::inject(
        &mut a_mut3,
        &ArraySeqStEphSLit![Pair(1, "dog"), Pair(3, "big"), Pair(6, "hog")],
    );
    assert_eq!(*ninjected, ArraySeqStEphSLit!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let mut a_mut4 = a.clone();
    let result_last = <ArraySeqStEphS<&str> as ArraySeqStEphBaseTrait<&str>>::inject(
        &mut a_mut4,
        &ArraySeqStEphSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")],
    );
    assert_eq!(
        *result_last,
        ArraySeqStEphSLit!["second", "updated", "in", "the", "hat"]
    );
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::empty();
    let empty_result = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySeqStEphSLit![100];
    let single_result = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::iterate(&numbers, &sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArraySeqStEphSLit![1, 2, 3];
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    let pairs = ArraySeqStEphSLit![Pair("a", 1_usize), Pair("b", 2_usize), Pair("a", 3_usize)];
    let grouped = <ArraySeqStEphS<Pair<&str, ArraySeqStEphS<N>>> as ArraySeqStEphBaseTrait<
        Pair<&str, ArraySeqStEphS<N>>,
    >>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let pair0 = grouped.nth(0);
    assert_eq!(pair0.0, "a");
    assert_eq!(pair0.1, ArraySeqStEphSLit![1, 3]);
    let pair1 = grouped.nth(1);
    assert_eq!(pair1.0, "b");
    assert_eq!(pair1.1, ArraySeqStEphSLit![2]);
}

#[test]
fn test_from_vec_empty() {
    let seq: ArraySeqStEphS<N> = ArraySeqStEphS::from_vec(vec![]);
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_new() {
    let seq = ArraySeqStEphS::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 42);
    }
}

#[test]
fn test_singleton() {
    let seq = ArraySeqStEphS::singleton(99);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 99);
}

#[test]
fn test_set_out_of_bounds() {
    let mut seq = ArraySeqStEphSLit![1, 2, 3];
    let result = seq.set(10, 99);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Index out of bounds");
}

#[test]
fn test_set_success() {
    let mut seq = ArraySeqStEphSLit![1, 2, 3];
    let result = seq.set(1, 42);
    assert!(result.is_ok());
    assert_eq!(*seq.nth(1), 42);
}

#[test]
fn test_iter_comprehensive() {
    let seq = ArraySeqStEphSLit![10, 20, 30, 40, 50];
    let collected: Vec<N> = seq.iter().copied().collect();
    assert_eq!(collected, vec![10, 20, 30, 40, 50]);

    let empty: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let empty_collected: Vec<N> = empty.iter().copied().collect();
    assert_eq!(empty_collected.len(), 0);
}

#[test]
fn test_subseq_edge_cases() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4, 5];

    // Start beyond end
    let sub1 = seq.subseq(10, 5);
    assert_eq!(sub1.length(), 0);

    // Length extends beyond end
    let sub2 = seq.subseq(3, 10);
    assert_eq!(sub2.length(), 2);
    assert_eq!(sub2, ArraySeqStEphSLit![4, 5]);

    // Zero length
    let sub3 = seq.subseq(2, 0);
    assert_eq!(sub3.length(), 0);

    // Full sequence
    let sub4 = seq.subseq(0, 5);
    assert_eq!(sub4, seq);
}

#[test]
fn test_equality() {
    let seq1 = ArraySeqStEphSLit![1, 2, 3];
    let seq2 = ArraySeqStEphSLit![1, 2, 3];
    let seq3 = ArraySeqStEphSLit![1, 2, 4];

    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
}

#[test]
fn test_debug_display() {
    let seq = ArraySeqStEphSLit![1, 2, 3];
    let debug_str = format!("{:?}", seq);
    assert!(debug_str.contains("1"));

    let display_str = format!("{}", seq);
    assert!(display_str.contains("1"));
}

#[test]
fn test_map_empty() {
    let empty: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let mapped = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::map(&empty, &|x| x + 1);
    assert_eq!(mapped.length(), 0);
}

#[test]
fn test_filter_empty() {
    let empty: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let filtered = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::filter(&empty, &|_| true);
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_flatten_empty() {
    let empty: ArraySeqStEphS<ArraySeqStEphS<N>> = ArraySeqStEphS::empty();
    let flattened = <ArraySeqStEphS<N> as ArraySeqStEphBaseTrait<N>>::flatten(&empty);
    assert_eq!(flattened.length(), 0);
}

#[test]
fn test_reduce_empty() {
    let empty: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let result = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::reduce(&empty, &|a, b| a + b, 100);
    assert_eq!(result, 100);
}

#[test]
fn test_scan_empty() {
    let empty: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
    let (prefixes, total) = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::scan(&empty, &|a, b| a + b, 0);
    // scan returns initial value as first prefix even for empty sequence
    assert_eq!(prefixes.length(), 1);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(total, 0);
}

#[test]
fn test_append_empty() {
    let seq = ArraySeqStEphSLit![1, 2, 3];
    let empty: ArraySeqStEphS<N> = ArraySeqStEphS::empty();

    let result1 = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&seq, &empty);
    assert_eq!(result1, seq);

    let result2 = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&empty, &seq);
    assert_eq!(result2, seq);

    let result3 = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::append(&empty, &empty);
    assert_eq!(result3.length(), 0);
}

#[test]
fn test_inject_empty_updates() {
    let mut seq = ArraySeqStEphSLit![1, 2, 3];
    let empty_updates: ArraySeqStEphS<Pair<N, N>> = ArraySeqStEphS::empty();
    seq.inject(&empty_updates);
    assert_eq!(seq, ArraySeqStEphSLit![1, 2, 3]);
}

#[test]
fn test_clone() {
    let seq1 = ArraySeqStEphSLit![1, 2, 3];
    let seq2 = seq1.clone();
    assert_eq!(seq1, seq2);
}

#[test]
fn test_tabulate_empty() {
    let seq = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::tabulate(&|i| i, 0);
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_large_sequence() {
    let seq = <ArraySeqStEphS<N> as ArraySeqStEphRedefinableTrait<N>>::tabulate(&|i| i, 1000);
    assert_eq!(seq.length(), 1000);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(500), 500);
    assert_eq!(*seq.nth(999), 999);
}
