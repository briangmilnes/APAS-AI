//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStPerSLit};

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            | 0 => 0,
            | 1 => 1,
            | _ => fib(n - 1) + fib(n - 2),
        }
    }
    let a: ArraySeqStPerS<N> = ArraySeqStPerS::tabulate(fib, 10);
    let fib10_head = ArraySeqStPerSLit![
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
    assert_eq!(fib10_head, ArraySeqStPerSLit![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let b = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&a, &|x| x + 1);
    assert_eq!(b, ArraySeqStPerSLit![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = ArraySeqStPerSLit![10, 20, 30, 40, 50];
    let b = a.subseq(1, 3);
    assert_eq!(b, ArraySeqStPerSLit![20, 30, 40]);
    let c = a.subseq(2, 0);
    assert_eq!(c.length(), 0);
    let d = a.subseq(0, 1);
    assert_eq!(d, ArraySeqStPerSLit![10]);
}

#[test]
fn test_append() {
    let a = ArraySeqStPerSLit![1, 2, 3];
    let b = ArraySeqStPerSLit![4, 5, 6];
    let c = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&a, &b);
    assert_eq!(c, ArraySeqStPerSLit![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySeqStPerSLit![1, 2, 3];
    let b = ArraySeqStPerSLit![4, 5];
    let c = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&a, &b);
    assert_eq!(c, ArraySeqStPerSLit![1, 2, 3, 4, 5]);
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    let d = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::append(&ArraySeqStPerSLit![1, 2, 3], &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::append(&empty, &ArraySeqStPerSLit![1, 2, 3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens =
        <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&numbers, |&x| if x % 2 == 0 { true } else { false });
    assert_eq!(evens, ArraySeqStPerSLit![2, 4, 6, 8, 10]);
    let odds_only = ArraySeqStPerSLit![1, 3, 5, 7];
    let no_evens = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&a, &|&x| {
        if x % 2 == 0 { true } else { false }
    });
    assert_eq!(no_evens.length(), 0);
}

// TODO: Fix flatten type inference - ArraySeqStPerS<ArraySeqStPerS<N>> trait bound issues
#[test]
fn test_flatten() {
    let a: ArraySeqStPerS<N> = ArraySeqStPerSLit![1, 2];
    let b: ArraySeqStPerS<N> = ArraySeqStPerSLit![3, 4, 5];
    let c: ArraySeqStPerS<N> = ArraySeqStPerSLit![6];
    let sequences: ArraySeqStPerS<ArraySeqStPerS<N>> = ArraySeqStPerSLit![a, b, c];
    let flattened = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&sequences);
    assert_eq!(flattened, ArraySeqStPerSLit![1, 2, 3, 4, 5, 6]);
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    let mixed: ArraySeqStPerS<ArraySeqStPerS<N>> =
        ArraySeqStPerSLit![ArraySeqStPerSLit![1, 2], empty, ArraySeqStPerSLit![3]];
    let mixed_flat = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySeqStPerSLit![1, 2, 3]);
}

#[test]
fn test_update_sequence() {
    let a = ArraySeqStPerSLit!["hello", "world", "test"];
    let b = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::update(&a, Pair(1, "rust"));
    assert_eq!(b, ArraySeqStPerSLit!["hello", "rust", "test"]);
    let c = ArraySeqStPerSLit!["hello", "world", "test"];
    let d = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::update(&c, Pair(5, "out_of_bounds"));
    assert_eq!(d, ArraySeqStPerSLit!["hello", "world", "test"]);
}

#[test]
fn test_inject_and_ninject() {
    let a = ArraySeqStPerSLit!["the", "cat", "in", "the", "hat"];
    let updates = ArraySeqStPerSLit![Pair(0, "a"), Pair(2, "on"), Pair(4, "mat")];
    let injected = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, ArraySeqStPerSLit!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = ArraySeqStPerSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")];
    let result_first = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::inject(&a, &conflicting_updates);
    assert_eq!(result_first, ArraySeqStPerSLit!["first", "updated", "in", "the", "hat"]);

    let ninjected = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::ninject(
        &a,
        &ArraySeqStPerSLit![Pair(1, "dog"), Pair(3, "big"), Pair(6, "hog")],
    );
    assert_eq!(ninjected, ArraySeqStPerSLit!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::ninject(
        &a,
        &ArraySeqStPerSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")],
    );
    assert_eq!(result_last, ArraySeqStPerSLit!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    let empty_result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySeqStPerSLit![100];
    let single_result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::iterate(&numbers, sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArraySeqStPerSLit![1, 2, 3];
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::iteratePrefixes(&numbers, sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    // Use N, N pairs since collect requires same types
    let pairs = ArraySeqStPerSLit![
        Pair(1_usize, 10_usize),
        Pair(2_usize, 20_usize),
        Pair(1_usize, 30_usize)
    ];
    let grouped: ArraySeqStPerS<Pair<N, ArraySeqStPerS<N>>> =
        <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let pair0 = grouped.nth(0);
    assert_eq!(pair0.0, 1);
    assert_eq!(pair0.1, ArraySeqStPerSLit![10, 30]);
    let pair1 = grouped.nth(1);
    assert_eq!(pair1.0, 2);
    assert_eq!(pair1.1, ArraySeqStPerSLit![20]);
}
}
