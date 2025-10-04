//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStEphSLit};

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            | 0 => 0,
            | 1 => 1,
            | _ => fib(n - 1) + fib(n - 2),
        }
    }
    let a = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&fib, 10);
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
    let b = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::map(&a, &|x| x + 1);
    assert_eq!(b, ArraySeqStEphSLit![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = ArraySeqStEphSLit![10, 20, 30, 40, 50];
    let b = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq(&a, 1, 3);
    assert_eq!(b, ArraySeqStEphSLit![20, 30, 40]);
    let c = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq(&a, 2, 0);
    assert_eq!(c.length(), 0);
    let d = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq(&a, 0, 1);
    assert_eq!(d, ArraySeqStEphSLit![10]);
}

#[test]
fn test_append() {
    let a = ArraySeqStEphSLit![1, 2, 3];
    let b = ArraySeqStEphSLit![4, 5, 6];
    let c = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySeqStEphSLit![1, 2, 3];
    let b = ArraySeqStEphSLit![4, 5];
    let c = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5]);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::empty();
    let d = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&ArraySeqStEphSLit![1, 2, 3], &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&empty, &ArraySeqStEphSLit![1, 2, 3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySeqStEphSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens =
        <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::filter(&numbers, &|&x| if x % 2 == 0 { true } else { false });
    assert_eq!(evens, ArraySeqStEphSLit![2, 4, 6, 8, 10]);
    let odds_only = ArraySeqStEphSLit![1, 3, 5, 7];
    let no_evens = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::filter(&odds_only, &|&x| {
        if x % 2 == 0 { true } else { false }
    });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = ArraySeqStEphSLit![1, 2];
    let b = ArraySeqStEphSLit![3, 4, 5];
    let c = ArraySeqStEphSLit![6];
    let sequences = ArraySeqStEphSLit![a, b, c];
    let flattened = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::flatten(&sequences);
    assert_eq!(flattened, ArraySeqStEphSLit![1, 2, 3, 4, 5, 6]);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::empty();
    let mixed = ArraySeqStEphSLit![ArraySeqStEphSLit![1, 2], empty, ArraySeqStEphSLit![3]];
    let mixed_flat = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySeqStEphSLit![1, 2, 3]);
}

#[test]
fn test_update_sequence() {
    // Eph: update mutates a clone (via update helper)
    let a = ArraySeqStEphSLit!["hello", "world", "test"];
    let mut b = a.clone();
    let _ = b.update(Pair(1, "rust").into());
    assert_eq!(b, ArraySeqStEphSLit!["hello", "rust", "test"]);
    let mut c = ArraySeqStEphSLit!["hello", "world", "test"];
    let _ = c.update(Pair(5, "out_of_bounds").into());
    assert_eq!(c, ArraySeqStEphSLit!["hello", "world", "test"]);
}

#[test]
fn test_inject_conflicts_last_wins() {
    let mut a = ArraySeqStEphSLit!["the", "cat", "in", "the", "hat"];
    let updates = ArraySeqStEphSLit![Pair(0, "a"), Pair(2, "on"), Pair(4, "mat")];
    let injected = <ArraySeqStEphS<&str> as ArraySeqStEphTrait<&str>>::inject(&mut a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(*injected, ArraySeqStEphSLit!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = ArraySeqStEphSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")];
    let result_last = <ArraySeqStEphS<&str> as ArraySeqStEphTrait<&str>>::inject(&mut a, &conflicting_updates);
    assert_eq!(*result_last, ArraySeqStEphSLit!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_ninject_conflicts_last_wins() {
    let mut a = ArraySeqStEphSLit!["the", "cat", "in", "the", "hat"];
    let ninjected = <ArraySeqStEphS<&str> as ArraySeqStEphTrait<&str>>::inject(
        &mut a,
        &ArraySeqStEphSLit![Pair(1, "dog"), Pair(3, "big"), Pair(6, "hog")],
    );
    assert_eq!(*ninjected, ArraySeqStEphSLit!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArraySeqStEphS<&str> as ArraySeqStEphTrait<&str>>::inject(
        &mut a,
        &ArraySeqStEphSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")],
    );
    assert_eq!(*result_last, ArraySeqStEphSLit!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::empty();
    let empty_result = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySeqStEphSLit![100];
    let single_result = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::iterate(&numbers, &sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    // iteratePrefixes method not available in ArraySeqStEphTrait
    // Using alternative approach with iterate
    let numbers = ArraySeqStEphSLit![1, 2, 3];
    let sum_fn = |a: &N, x: &N| a + x;
    let total = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::iterate(&numbers, &sum_fn, 0);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    let pairs = ArraySeqStEphSLit![Pair("a", 1_usize), Pair("b", 2_usize), Pair("a", 3_usize)];
    let grouped = <ArraySeqStEphS<Pair<&str, ArraySeqStEphS<N>>> as ArraySeqStEphTrait<
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
