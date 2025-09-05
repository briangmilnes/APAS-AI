//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::{ArraySPersistent, ArraySeqPersistentTrait, ArraySeqPersistentChap18Trait, ArraySeqPersistentChap19Trait};
use std::sync::Mutex;

#[test]
fn test_empty() {
    let e = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    assert_eq!(e.length(), 0);
}

#[test]
fn test_singleton() {
    let s: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::singleton(42);
    assert_eq!(s, ArraySPersistent::from_vec(vec![42]));
}

#[test]
fn test_map() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 5);
    let b = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::map(&a, |x| x * 2);
    assert_eq!(b, ArraySPersistent::from_vec(vec![2, 4, 6, 8, 10]));
}

#[test]
fn test_append() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 4, 2);
    let c = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::append(&a, &b);
    assert_eq!(c, ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_append2() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 4, 2);
    let c = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::append2(&a, &b);
    assert_eq!(c, ArraySPersistent::from_vec(vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_deflate_true() {
    let y = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(y, ArraySPersistent::from_vec(vec![6]));
}

#[test]
fn test_deflate_false() {
    let y = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &5);
    assert_eq!(y.length(), 0);
}

#[test]
fn test_filter_even_numbers() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 10);
    let evens = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArraySPersistent::from_vec(vec![2, 4, 6, 8, 10]));
}

#[test]
fn test_filter_none() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 5);
    let odds_only = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::filter(&a, |x| if *x % 2 == 1 { B::True } else { B::False });
    assert_eq!(odds_only, ArraySPersistent::from_vec(vec![1, 3]));
}

#[test]
fn test_update_in_bounds() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 5);
    let a2 = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::update(&a, (2, 99));
    assert_eq!(a2, ArraySPersistent::from_vec(vec![1, 2, 99, 4, 5]));
}

#[test]
fn test_update_out_of_bounds() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 3);
    let a2 = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::update(&a, (10, 7));
    assert_eq!(a2, ArraySPersistent::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_isEmpty() {
    let e = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    let s = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::singleton(7);
    assert_eq!(s.isEmpty(), B::False);
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 2);
    assert_eq!(a.isEmpty(), B::False);
}

#[test]
fn test_isSingleton() {
    let e = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    assert_eq!(e.isSingleton(), B::False);
    let s = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::singleton(7);
    assert_eq!(s.isSingleton(), B::True);
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 2);
    assert_eq!(a.isSingleton(), B::False);
}

#[test]
fn test_iterate_sum() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 5);
    let sum = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_iterate_concat() {
    let a = <ArraySPersistent<&str> as ArraySeqPersistentChap19Trait>::tabulate(
        |i| match i { 0 => "a", 1 => "b", _ => "c" },
        3,
    );
    let res = <ArraySPersistent<&str> as ArraySeqPersistentChap19Trait>::iterate(
        &a,
        |acc: &String, x: &&str| format!("{}{}", acc, x),
        String::new(),
    );
    assert_eq!(res, "abc");
}

#[test]
fn test_map_empty() {
    let e: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|_| 0, 0);
    let m = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::map(&e, |x| x + 1);
    assert_eq!(m.length(), 0);
}

#[test]
fn test_append_with_empty() {
    let e: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|_| 0, 0);
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 3);
    let left = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::append(&e, &a);
    assert_eq!(left, ArraySPersistent::from_vec(vec![1, 2, 3]));
    let right = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::append(&a, &e);
    assert_eq!(right, ArraySPersistent::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_append2_equivalence() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 4, 2);
    let c1 = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::append(&a, &b);
    let c2 = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::append2(&a, &b);
    assert_eq!(c1, c2);
}

#[test]
fn test_filter_empty_sequence() {
    let e: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|_| 0, 0);
    let f = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::filter(&e, |_| B::True);
    assert_eq!(f.length(), 0);
}

#[test]
fn test_select_boundary() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 2);
    let b = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 3, 2);
    assert_eq!(<ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::select(&a, &b, 0), Some(&1));
    assert_eq!(<ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::select(&a, &b, 1), Some(&2));
    assert_eq!(<ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::select(&a, &b, 2), Some(&3));
    assert_eq!(<ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::select(&a, &b, 3), Some(&4));
    assert_eq!(<ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::select(&a, &b, 4), None);
}

#[test]
fn test_subseq_basic() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| (i + 1) * 10, 5);
    let s = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(s, ArraySPersistent::from_vec(vec![20, 30, 40]));
}

#[test]
fn test_reduce_sum_basic_ch19() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let e: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|_| 0, 0);
    let re = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::reduce(&e, &sum_fn, 42);
    assert_eq!(re, 42);
    let s = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|_| 7, 1);
    let rs = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::reduce(&s, &sum_fn, 0);
    assert_eq!(rs, 7);
}

#[test]
fn test_scan_sum_basic_ch19() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let (prefixes, total) = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::scan(&a, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(total, 15);
}

#[test]
fn test_flatten_ch19() {
    let s1 = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 2);
    let s2 = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 3, 2);
    let nested: ArraySPersistent<ArraySPersistent<N>> = ArraySPersistent::from_vec(vec![s1, s2]);
    let flat = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::flatten(&nested);
    assert_eq!(flat, ArraySPersistent::from_vec(vec![1, 2, 3, 4]));
}

#[test]
fn test_atomic_write_lowest_wins_serial() {
    let len: N = 3;
    let base = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, len);
    let mut values_with_changenum: ArraySPersistent<(N, N)> = <ArraySPersistent<(N, N)> as ArraySeqPersistentChap19Trait>::tabulate(
        |i| (base.nth(i).clone(), len),
        len,
    );
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(1, 33), (1, 44)]);
    <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::atomicWrite(&mut values_with_changenum, &changes, 1);
    <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::atomicWrite(&mut values_with_changenum, &changes, 0);
    let (v, c) = values_with_changenum.nth(1);
    assert_eq!((*v, *c), (33, 0));
}

#[test]
fn test_atomic_write_highest_wins_mutex() {
    let len: N = 3;
    let base = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, len);
    let values_with_changenum: ArraySPersistent<Mutex<(N, N)>> = <ArraySPersistent<Mutex<(N, N)>> as ArraySeqPersistentChap19Trait>::tabulate(
        |i| Mutex::new((base.nth(i).clone(), 0)),
        len,
    );
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(1, 33), (1, 44)]);
    <ArraySPersistent<Mutex<(N, N)>> as ArraySeqPersistentChap19Trait>::AtomicWriteHighestChangeNumberWins(
        &values_with_changenum,
        &changes,
        0,
    );
    <ArraySPersistent<Mutex<(N, N)>> as ArraySeqPersistentChap19Trait>::AtomicWriteHighestChangeNumberWins(
        &values_with_changenum,
        &changes,
        1,
    );
    let guard = values_with_changenum.nth(1).lock().unwrap();
    assert_eq!(guard.0, 44);
    assert_eq!(guard.1, 1);
}

#[test]
fn test_iterate_empty_returns_acc() {
    let e: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|_| 0, 0);
    let sum = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::iterate(&e, |acc: &N, x: &N| acc + x, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_inject_parallel2_equivalence_leftmost_wins() {
    let values = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 6);
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(2, 99), (2, 7), (4, 20)]);
    let serial = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject(&values, &changes);
    let parallel = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);
}

#[test]
fn test_inject_parallel2_out_of_bounds_ignored() {
    let values = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, 3);
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(10, 42), (1, 99)]);
    let out = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject_parallel2(&values, &changes);
    assert_eq!(out, ArraySPersistent::from_vec(vec![1, 99, 3]));
}

#[test]
fn test_inject_parallel2_empty_values_and_empty_changes() {
    let values: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![]);
    let out = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject_parallel2(&values, &changes);
    assert_eq!(out.length(), 0);
}

#[test]
fn test_inject_parallel2_empty_values_nonempty_changes() {
    let values: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::empty();
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(0, 10), (1, 20)]);
    let out = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject_parallel2(&values, &changes);
    assert_eq!(out.length(), 0);
}

#[test]
fn test_inject_parallel2_nonempty_values_empty_changes() {
    let values = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 10, 4);
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![]);
    let out = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject_parallel2(&values, &changes);
    assert_eq!(out, values);
}

#[test]
fn test_actually_atomic_write_ordering() {
    let length: N = 3;
    let base: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 1, length);
    let values_with_changenumber: ArraySPersistent<Mutex<(N, N)>> = <ArraySPersistent<Mutex<(N, N)>> as ArraySeqPersistentChap19Trait>::tabulate(
        |i| Mutex::new((base.nth(i).clone(), length)),
        length,
    );
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(1, 33), (1, 44)]);
    <ArraySPersistent<Mutex<(N, N)>> as ArraySeqPersistentChap19Trait>::AtomicWriteLowestChangeNumberWins(
        &values_with_changenumber,
        &changes,
        1,
    );
    <ArraySPersistent<Mutex<(N, N)>> as ArraySeqPersistentChap19Trait>::AtomicWriteLowestChangeNumberWins(
        &values_with_changenumber,
        &changes,
        0,
    );
    {
        let guard = values_with_changenumber.nth(1).lock().unwrap();
        assert_eq!(guard.0, 33);
        assert_eq!(guard.1, 0);
    }
    {
        let g0 = values_with_changenumber.nth(0).lock().unwrap();
        let g2 = values_with_changenumber.nth(2).lock().unwrap();
        assert_eq!(g0.0, 1);
        assert_eq!(g0.1, length);
        assert_eq!(g2.0, 3);
        assert_eq!(g2.1, length);
    }
}

#[test]
fn test_inject_first_wins_duplicate_index() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 5);
    let updates: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(2, 99), (2, 7)]);
    let out = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject(&a, &updates);
    assert_eq!(out.length(), 5);
    assert_eq!(*out.nth(0), 0);
    assert_eq!(*out.nth(1), 1);
    assert_eq!(*out.nth(2), 99);
    assert_eq!(*out.nth(3), 3);
    assert_eq!(*out.nth(4), 4);
}

#[test]
fn test_inject_empty_updates_no_change() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i + 10, 4);
    let empty: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![]);
    let out = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject(&a, &empty);
    assert_eq!(out.length(), 4);
    for i in 0..4 { assert_eq!(*out.nth(i), *a.nth(i)); }
}

#[test]
fn test_inject_parallel_equivalence() {
    let a = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 6);
    let updates: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(1, 10), (4, 20), (1, 99)]);
    let seq1 = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject(&a, &updates);
    let seq2 = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::inject_parallel2(&a, &updates);
    assert_eq!(seq1.length(), seq2.length());
    for i in 0..seq1.length() { assert_eq!(seq1.nth(i), seq2.nth(i)); }
}

#[test]
fn test_ninject_parallel2_equivalence_last_wins() {
    let values = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::tabulate(|i| i, 5);
    let changes: ArraySPersistent<(N, N)> = ArraySPersistent::from_vec(vec![(2, 99), (2, 7)]);
    let serial = <ArraySPersistent<N> as ArraySeqPersistentChap18Trait>::ninject(&values, &changes);
    let parallel = <ArraySPersistent<N> as ArraySeqPersistentChap19Trait>::ninject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);
}


