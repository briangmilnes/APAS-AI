//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::{ArraySeqEphS, ArraySeqEphTrait, ArraySeqEphChap18Trait};

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }
    let a = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::tabulate(fib, 10);
    let fib10_head = ArraySeqEphS::from_vec(vec![
        *a.nth(0), *a.nth(1), *a.nth(2), *a.nth(3), *a.nth(4),
        *a.nth(5), *a.nth(6), *a.nth(7), *a.nth(8), *a.nth(9)
    ]);
    assert_eq!(fib10_head, ArraySeqEphS::from_vec(vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]));
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArraySeqEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let b = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::map(&a, |x| x + 1);
    assert_eq!(b, ArraySeqEphS::from_vec(vec![2, 3, 4, 5, 6]));
}

#[test]
fn test_subseq() {
    let a = ArraySeqEphS::from_vec(vec![10, 20, 30, 40, 50]);
    let b = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(b, ArraySeqEphS::from_vec(vec![20, 30, 40]));
    let c = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::subseq_copy(&a, 2, 0);
    assert_eq!(c.length(), 0);
    let d = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::subseq_copy(&a, 0, 1);
    assert_eq!(d, ArraySeqEphS::from_vec(vec![10]));
}

#[test]
fn test_append() {
    let a = ArraySeqEphS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqEphS::from_vec(vec![4, 5, 6]);
    let c = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::append(&a, &b);
    assert_eq!(c, ArraySeqEphS::from_vec(vec![1, 2, 3, 4, 5, 6]));
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySeqEphS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqEphS::from_vec(vec![4, 5]);
    let c = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::append(&a, &b);
    assert_eq!(c, ArraySeqEphS::from_vec(vec![1, 2, 3, 4, 5]));
    let empty: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::empty();
    let d = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::append(&ArraySeqEphS::from_vec(vec![1,2,3]), &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::append(&empty, &ArraySeqEphS::from_vec(vec![1,2,3]));
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySeqEphS::from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
    let evens = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::filter(&numbers, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArraySeqEphS::from_vec(vec![2, 4, 6, 8, 10]));
    let odds_only = ArraySeqEphS::from_vec(vec![1,3,5,7]);
    let no_evens = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::filter(&odds_only, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = ArraySeqEphS::from_vec(vec![1, 2]);
    let b = ArraySeqEphS::from_vec(vec![3, 4, 5]);
    let c = ArraySeqEphS::from_vec(vec![6]);
    let sequences = ArraySeqEphS::from_vec(vec![a, b, c]);
    let flattened = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::flatten(&sequences);
    assert_eq!(flattened, ArraySeqEphS::from_vec(vec![1, 2, 3, 4, 5, 6]));
    let empty: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::empty();
    let mixed = ArraySeqEphS::from_vec(vec![ArraySeqEphS::from_vec(vec![1, 2]), empty, ArraySeqEphS::from_vec(vec![3])]);
    let mixed_flat = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySeqEphS::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_update_sequence() {
    // Eph: update mutates a clone (via update helper)
    let a = ArraySeqEphS::from_vec(vec!["hello", "world", "test"]);
    let mut b = a.clone();
    let _ = <ArraySeqEphS<&str> as ArraySeqEphChap18Trait>::update(&mut b, (1, "rust"));
    assert_eq!(b, ArraySeqEphS::from_vec(vec!["hello", "rust", "test"]));
    let mut c = ArraySeqEphS::from_vec(vec!["hello", "world", "test"]);
    let _ = <ArraySeqEphS<&str> as ArraySeqEphChap18Trait>::update(&mut c, (5, "out_of_bounds"));
    assert_eq!(c, ArraySeqEphS::from_vec(vec!["hello", "world", "test"]));
}

#[test]
fn test_inject_conflicts_last_wins() {
    let a = ArraySeqEphS::from_vec(vec!["the", "cat", "in", "the", "hat"]);
    let updates = ArraySeqEphS::from_vec(vec![(0, "a"), (2, "on"), (4, "mat")]);
    let injected = <ArraySeqEphS<&str> as ArraySeqEphChap18Trait>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, ArraySeqEphS::from_vec(vec!["a", "cat", "on", "the", "mat"]));

    let conflicting_updates = ArraySeqEphS::from_vec(vec![(0, "first"), (0, "second"), (1, "updated")]);
    let result_last = <ArraySeqEphS<&str> as ArraySeqEphChap18Trait>::inject(&a, &conflicting_updates);
    assert_eq!(result_last, ArraySeqEphS::from_vec(vec!["second", "updated", "in", "the", "hat"]));
}

#[test]
fn test_ninject_conflicts_last_wins() {
    let a = ArraySeqEphS::from_vec(vec!["the", "cat", "in", "the", "hat"]);
    let ninjected = <ArraySeqEphS<&str> as ArraySeqEphChap18Trait>::ninject(
        &a,
        &ArraySeqEphS::from_vec(vec![(1, "dog"), (3, "big"), (6, "hog")]),
    );
    assert_eq!(ninjected, ArraySeqEphS::from_vec(vec!["the", "dog", "in", "big", "hat"]));
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArraySeqEphS<&str> as ArraySeqEphChap18Trait>::ninject(
        &a,
        &ArraySeqEphS::from_vec(vec![(0, "first"), (0, "second"), (1, "updated")]),
    );
    assert_eq!(result_last, ArraySeqEphS::from_vec(vec!["second", "updated", "in", "the", "hat"]));
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySeqEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::empty();
    let empty_result = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySeqEphS::from_vec(vec![100]);
    let single_result = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySeqEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::iterate(&numbers, sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArraySeqEphS::from_vec(vec![1, 2, 3]);
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::iteratePrefixes(&numbers, sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    let pairs = ArraySeqEphS::from_vec(vec![("a", 1_usize), ("b", 2_usize), ("a", 3_usize)]);
    let grouped = <ArraySeqEphS<(&str, N)> as ArraySeqEphChap18Trait>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let (k0, v0) = grouped.nth(0);
    assert_eq!(*k0, "a");
    assert_eq!(*v0, ArraySeqEphS::from_vec(vec![1, 3]));
    let (k1, v1) = grouped.nth(1);
    assert_eq!(*k1, "b");
    assert_eq!(*v1, ArraySeqEphS::from_vec(vec![2]));
}


