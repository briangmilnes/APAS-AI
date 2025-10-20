//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtEphChap18 multithreaded ephemeral algorithms.

use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Types::Types::*;

fn identity(i: N) -> N {
    i
}
fn double(i: N) -> N {
    i * 2
}
fn add_one(i: N) -> N {
    i + 1
}
fn add_ten(i: N) -> N {
    i + 10
}
fn multiply_by_two(x: &N) -> N {
    x * 2
}
fn add_nums(x: &N, y: &N) -> N {
    x + y
}
fn is_even_bool(x: &N) -> B {
    x % 2 == 0
}

#[test]
fn test_new_and_set() {
    let mut a: ArraySeqMtEphS<N> = ArraySeqMtEphS::new(5, 0);
    assert_eq!(a.length(), 5);

    // Test set method
    let _ = a.set(0, 10);
    let _ = a.set(4, 40);
    // Note: set doesn't return Result, so no bounds checking test

    assert_eq!(a.nth_cloned(0), 10);
    assert_eq!(a.nth_cloned(4), 40);
}

#[test]
fn test_tabulate_basic() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<N> as ArraySeqMtEphRedefinableTrait<N>>::tabulate(&double, 5);
    assert_eq!(a.length(), 5);

    for i in 0..5 {
        assert_eq!(a.nth_cloned(i), i * 2);
    }
}

#[test]
fn test_map_parallel() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<N> as ArraySeqMtEphRedefinableTrait<N>>::tabulate(&identity, 2000); // Large enough for parallel
    let doubled: ArraySeqMtEphS<N> = <ArraySeqMtEphS<N> as ArraySeqMtEphRedefinableTrait<N>>::map(&a, &multiply_by_two);

    assert_eq!(doubled.length(), 2000);
    for i in 0..2000 {
        assert_eq!(doubled.nth_cloned(i), i * 2);
    }
}

#[test]
fn test_reduce_parallel() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&add_one, 2000); // [1, 2, 3, ..., 2000]
    let sum = <ArraySeqMtEphS<_> as ArraySeqMtEphBaseTrait<_>>::reduce(&a, add_nums, 0);

    // Sum of 1 to 2000 = 2000 * 2001 / 2 = 2001000
    assert_eq!(sum, 2001000);
}

#[test]
fn test_filter() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&identity, 10);
    let evens = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::filter(&a, &is_even_bool);

    assert_eq!(evens.length(), 5); // 0, 2, 4, 6, 8
    for i in 0..5 {
        assert_eq!(evens.nth_cloned(i), i * 2);
    }
}

#[test]
fn test_append() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&identity, 3);
    let b: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&add_ten, 2);
    let combined = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::append(&a, &b);

    assert_eq!(combined.length(), 5);
    assert_eq!(combined.nth_cloned(0), 0);
    assert_eq!(combined.nth_cloned(1), 1);
    assert_eq!(combined.nth_cloned(2), 2);
    assert_eq!(combined.nth_cloned(3), 10);
    assert_eq!(combined.nth_cloned(4), 11);
}

#[test]
fn test_flatten() {
    let inner1: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&|i| i, 2);
    let inner2: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&|i| i + 10, 3);
    let outer = ArraySeqMtEphSLit![inner1, inner2];

    let flattened = <ArraySeqMtEphS<_> as ArraySeqMtEphBaseTrait<_>>::flatten(&outer);
    assert_eq!(flattened.length(), 5);
    assert_eq!(flattened.nth_cloned(0), 0);
    assert_eq!(flattened.nth_cloned(1), 1);
    assert_eq!(flattened.nth_cloned(2), 10);
    assert_eq!(flattened.nth_cloned(3), 11);
    assert_eq!(flattened.nth_cloned(4), 12);
}

#[test]
fn test_scan() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&|i| i + 1, 5); // [1, 2, 3, 4, 5]
    let (prefixes, final_sum) = <ArraySeqMtEphS<_> as ArraySeqMtEphBaseTrait<_>>::scan(&a, &|x, y| x + y, 0);

    assert_eq!(prefixes.length(), 5);
    assert_eq!(prefixes.nth_cloned(0), 1); // 0 + 1
    assert_eq!(prefixes.nth_cloned(1), 3); // 1 + 2
    assert_eq!(prefixes.nth_cloned(2), 6); // 3 + 3
    assert_eq!(prefixes.nth_cloned(3), 10); // 6 + 4
    assert_eq!(prefixes.nth_cloned(4), 15); // 10 + 5
    assert_eq!(final_sum, 15);
}

#[test]
fn test_iterate() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&|i| i + 1, 5); // [1, 2, 3, 4, 5]
    let sum = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::iterate(&a, &|acc, x| acc + x, 0);
    assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
}

#[test]
fn test_collect() {
    // Now that collect supports Pair<K, V>, we can use different types
    let pairs: ArraySeqMtEphS<Pair<N, &str>> = ArraySeqMtEphS::from_vec(vec![
        Pair(1, "alice"),
        Pair(2, "bob"),
        Pair(1, "alex"),
        Pair(2, "betty"),
    ]);
    let grouped =
        <ArraySeqMtEphS<Pair<N, &str>> as ArraySeqMtEphBaseTrait<Pair<N, &str>>>::collect(&pairs, |a, b| a.cmp(b));

    assert_eq!(grouped.length(), 2);
    // First group should have key 1 with values ["alice", "alex"]
    let group0 = grouped.nth_cloned(0);
    assert_eq!(group0.0, 1);
    assert_eq!(group0.1.length(), 2);
    // Second group should have key 2 with values ["bob", "betty"]
    let group1 = grouped.nth_cloned(1);
    assert_eq!(group1.0, 2);
    assert_eq!(group1.1.length(), 2);
}

#[test]
fn test_update() {
    let mut a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&|i| i, 5);
    let updated = <ArraySeqMtEphS<_> as ArraySeqMtEphBaseTrait<_>>::update(&mut a, (2, 99));

    assert_eq!(updated.nth_cloned(2), 99);
    assert_eq!(updated.nth_cloned(0), 0); // Others unchanged
    assert_eq!(updated.nth_cloned(4), 4);
}

#[test]
fn test_inject() {
    let a: ArraySeqMtEphS<N> = <ArraySeqMtEphS<_> as ArraySeqMtEphRedefinableTrait<_>>::tabulate(&|i| i, 5);
    let updates = ArraySeqMtEphS::from_vec(vec![
        Pair(1, 100),
        Pair(3, 300),
        Pair(1, 111), // Duplicate index - first wins
    ]);

    let injected = <ArraySeqMtEphS<_> as ArraySeqMtEphBaseTrait<_>>::inject(&a, &updates);
    assert_eq!(injected.nth_cloned(0), 0); // Unchanged
    assert_eq!(injected.nth_cloned(1), 100); // Updated (first occurrence)
    assert_eq!(injected.nth_cloned(2), 2); // Unchanged
    assert_eq!(injected.nth_cloned(3), 300); // Updated
    assert_eq!(injected.nth_cloned(4), 4); // Unchanged
}

#[test]
fn test_empty_and_singleton() {
    let empty: ArraySeqMtEphS<N> = ArraySeqMtEphS::empty();
    assert_eq!(empty.length(), 0);
    assert!(empty.length() == 0);
    assert!(empty.length() != 1);

    let single: ArraySeqMtEphS<N> = ArraySeqMtEphS::singleton(42);
    assert_eq!(single.length(), 1);
    assert!(single.length() != 0);
    assert!(single.length() == 1);
    assert_eq!(single.nth_cloned(0), 42);
}

#[test]
fn test_subseq_copy() {
    let seq: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let sub = seq.subseq_copy(3, 4);
    assert_eq!(sub.length(), 4);
    assert_eq!(sub.nth_cloned(0), 3);
    assert_eq!(sub.nth_cloned(3), 6);
}

#[test]
fn test_ninject() {
    let seq: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let updates = ArraySeqMtEphS::from_vec(vec![Pair(1, 10), Pair(3, 30)]);
    let result = <ArraySeqMtEphS<_> as ArraySeqMtEphBaseTrait<_>>::ninject(&seq, &updates);
    assert_eq!(result.nth_cloned(1), 10);
    assert_eq!(result.nth_cloned(3), 30);
}

#[test]
fn test_collect_mt_eph() {
    let pairs: ArraySeqMtEphS<Pair<N, N>> = ArraySeqMtEphS::from_vec(vec![Pair(1, 10), Pair(2, 20), Pair(1, 11)]);
    let grouped = <ArraySeqMtEphS<Pair<N, N>> as ArraySeqMtEphBaseTrait<Pair<N, N>>>::collect(&pairs, |a, b| a.cmp(b));
    assert_eq!(grouped.length(), 2);
}

#[test]
fn test_clone_mt_eph() {
    let seq: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let cloned = seq.clone();
    assert_eq!(seq, cloned);
}

#[test]
fn test_partial_eq_mt_eph() {
    let seq1: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let seq2: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let seq3: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 4]);
    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
}

#[test]
fn test_display_mt_eph() {
    let seq: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let display_str = format!("{seq}");
    assert!(display_str.contains("1"));
    assert!(display_str.contains("2"));
    assert!(display_str.contains("3"));
}

#[test]
fn test_to_vec() {
    let seq: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let vec = seq.to_vec();
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_iter_cloned() {
    let seq: ArraySeqMtEphS<N> = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let vec = seq.iter_cloned();
    assert_eq!(vec.len(), 5);
    assert_eq!(vec[0], 1);
    assert_eq!(vec[4], 5);
}
