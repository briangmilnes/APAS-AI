//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::{ArraySPersistent, ArraySeqPersistentTrait, ArraySeqPersistentChap18Trait};

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::tabulate(fib, 10);
    let fib10_head = ArraySPersistent::from_vec(vec![
        *a.nth(0), *a.nth(1), *a.nth(2), *a.nth(3), *a.nth(4),
        *a.nth(5), *a.nth(6), *a.nth(7), *a.nth(8), *a.nth(9)
    ]);
    assert_eq!(fib10_head, ArraySPersistent::from_vec(vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]));
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5]);
    let b = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::map(&a, |x| x + 1);
    assert_eq!(b, ArraySPersistent::from_vec(vec![2, 3, 4, 5, 6]));
}

#[test]
fn test_subseq() {
    let a = ArraySPersistent::from_vec(vec![10, 20, 30, 40, 50]);
    let b = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(b, ArraySPersistent::from_vec(vec![20, 30, 40]));
    let c = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::subseq_copy(&a, 2, 0);
    assert_eq!(c.length(), 0);
    let d = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::subseq_copy(&a, 0, 1);
    assert_eq!(d, ArraySPersistent::from_vec(vec![10]));
}

#[test]
fn test_append() {
    let a = ArraySPersistent::from_vec(vec![1, 2, 3]);
    let b = ArraySPersistent::from_vec(vec![4, 5, 6]);
    let c = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::append(&a, &b);
    assert_eq!(c, ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5, 6]));
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySPersistent::from_vec(vec![1, 2, 3]);
    let b = ArraySPersistent::from_vec(vec![4, 5]);
    let c = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::append(&a, &b);
    assert_eq!(c, ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5]));
    let empty: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    let d = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::append(&ArraySPersistent::from_vec(vec![1,2,3]), &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::append(&empty, &ArraySPersistent::from_vec(vec![1,2,3]));
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySPersistent::from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
    let evens = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::filter(&numbers, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArraySPersistent::from_vec(vec![2, 4, 6, 8, 10]));
    let odds_only = ArraySPersistent::from_vec(vec![1,3,5,7]);
    let no_evens = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::filter(&odds_only, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = ArraySPersistent::from_vec(vec![1, 2]);
    let b = ArraySPersistent::from_vec(vec![3, 4, 5]);
    let c = ArraySPersistent::from_vec(vec![6]);
    let sequences = ArraySPersistent::from_vec(vec![a, b, c]);
    let flattened = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::flatten(&sequences);
    assert_eq!(flattened, ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5, 6]));
    let empty: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    let mixed = ArraySPersistent::from_vec(vec![ArraySPersistent::from_vec(vec![1, 2]), empty, ArraySPersistent::from_vec(vec![3])]);
    let mixed_flat = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySPersistent::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_update_sequence() {
    // Persistent: update returns new sequence
    let a = ArraySPersistent::from_vec(vec!["hello", "world", "test"]);
    let b = <ArraySPersistent<&str> as ArraySeqPersistentChap18Trait>::update(&a, (1, "rust"));
    assert_eq!(b, ArraySPersistent::from_vec(vec!["hello", "rust", "test"]));
    let c = ArraySPersistent::from_vec(vec!["hello", "world", "test"]);
    let d = <ArraySPersistent<&str> as ArraySeqPersistentChap18Trait>::update(&c, (5, "out_of_bounds"));
    assert_eq!(d, ArraySPersistent::from_vec(vec!["hello", "world", "test"]));
}

#[test]
fn test_inject_and_ninject() {
    let a = ArraySPersistent::from_vec(vec!["the", "cat", "in", "the", "hat"]);
    let updates = ArraySPersistent::from_vec(vec![(0, "a"), (2, "on"), (4, "mat")]);
    let injected = <ArraySPersistent<&str> as ArraySeqPersistentChap18Trait>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, ArraySPersistent::from_vec(vec!["a", "cat", "on", "the", "mat"]));

    let conflicting_updates = ArraySPersistent::from_vec(vec![(0, "first"), (0, "second"), (1, "updated")]);
    let result_first = <ArraySPersistent<&str> as ArraySeqPersistentChap18Trait>::inject(&a, &conflicting_updates);
    assert_eq!(result_first, ArraySPersistent::from_vec(vec!["first", "updated", "in", "the", "hat"]));

    let ninjected = <ArraySPersistent<&str> as ArraySeqPersistentChap18Trait>::ninject(&a, &ArraySPersistent::from_vec(vec![(1, "dog"), (3, "big"), (6, "hog")]));
    assert_eq!(ninjected, ArraySPersistent::from_vec(vec!["the", "dog", "in", "big", "hat"]));
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArraySPersistent<&str> as ArraySeqPersistentChap18Trait>::ninject(&a, &ArraySPersistent::from_vec(vec![(0, "first"), (0, "second"), (1, "updated")]));
    assert_eq!(result_last, ArraySPersistent::from_vec(vec!["second", "updated", "in", "the", "hat"]));
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5]);
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    let empty_result = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySPersistent::from_vec(vec![100]);
    let single_result = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5]);
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::iterate(&numbers, sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArraySPersistent::from_vec(vec![1, 2, 3]);
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::iteratePrefixes(&numbers, sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    let pairs = ArraySPersistent::from_vec(vec![("a", 1_usize), ("b", 2_usize), ("a", 3_usize)]);
    let grouped = <ArraySPersistent<(&str, N)> as ArraySeqPersistentChap18Trait>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let (k0, v0) = grouped.nth(0);
    assert_eq!(*k0, "a");
    assert_eq!(*v0, ArraySPersistent::from_vec(vec![1, 3]));
    let (k1, v1) = grouped.nth(1);
    assert_eq!(*k1, "b");
    assert_eq!(*v1, ArraySPersistent::from_vec(vec![2]));
}


