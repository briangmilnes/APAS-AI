//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::ArraySeq::{ArrayS, ArraySeq};
use apas_ai::ArraySeqChap18::ArraySeqChap18;
use apas_ai::ArraySeqChap19::ArraySeqChap19;
use std::sync::Mutex;

#[test]
fn test_empty() {
    let e = <ArrayS<N> as ArraySeq<N>>::empty();
    assert_eq!(e.length(), 0);
}

#[test]
fn test_singleton() {
    let s: ArrayS<N> = <ArrayS<N> as ArraySeq<N>>::singleton(42);
    assert_eq!(s, apas_ai::arrayseq![42]);
}

#[test]
fn test_map() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 5);
    let b = <ArrayS<N> as ArraySeqChap19>::map(&a, |x| x * 2);
    assert_eq!(b, apas_ai::arrayseq![2, 4, 6, 8, 10]);
}

#[test]
fn test_append() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 3);
    let b = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 4, 2);
    let c = <ArrayS<N> as ArraySeqChap19>::append(&a, &b);
    assert_eq!(c, apas_ai::arrayseq![1, 2, 3, 4, 5]);
}

#[test]
fn test_append2() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 3);
    let b = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 4, 2);
    let c = <ArrayS<N> as ArraySeqChap19>::append2(&a, &b);
    assert_eq!(c, apas_ai::arrayseq![1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate_true() {
    let y = <ArrayS<N> as ArraySeqChap19>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(y, apas_ai::arrayseq![6]);
}

#[test]
fn test_deflate_false() {
    let y = <ArrayS<N> as ArraySeqChap19>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &5);
    assert_eq!(y.length(), 0);
}

#[test]
fn test_filter_even_numbers() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 10);
    let evens = <ArrayS<N> as ArraySeqChap19>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, apas_ai::arrayseq![2, 4, 6, 8, 10]);
}

#[test]
fn test_filter_none() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 5);
    let odds_only = <ArrayS<N> as ArraySeqChap19>::filter(&a, |x| if *x % 2 == 1 { B::True } else { B::False });
    assert_eq!(odds_only, apas_ai::arrayseq![1, 3]);
}

#[test]
fn test_update_in_bounds() {
    let mut a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 5);
    let _ = a.update((2, 99));
    assert_eq!(a, apas_ai::arrayseq![1, 2, 99, 4, 5]);
}

#[test]
fn test_update_out_of_bounds() {
    let mut a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 3);
    let _ = a.update((10, 7));
    assert_eq!(a, apas_ai::arrayseq![1, 2, 3]);
}

#[test]
fn test_isEmpty() {
    let e = <ArrayS<N> as ArraySeq<N>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    let s = <ArrayS<N> as ArraySeq<N>>::singleton(7);
    assert_eq!(s.isEmpty(), B::False);
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 2);
    assert_eq!(a.isEmpty(), B::False);
}

#[test]
fn test_isSingleton() {
    let e = <ArrayS<N> as ArraySeq<N>>::empty();
    assert_eq!(e.isSingleton(), B::False);
    let s = <ArrayS<N> as ArraySeq<N>>::singleton(7);
    assert_eq!(s.isSingleton(), B::True);
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 2);
    assert_eq!(a.isSingleton(), B::False);
}

#[test]
fn test_iterate_sum() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 5);
    let sum = <ArrayS<N> as ArraySeqChap19>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_iterate_concat() {
    let a = <ArrayS<&str> as ArraySeqChap19>::tabulate(
        |i| match i { 0 => "a", 1 => "b", _ => "c" },
        3,
    );
    let res = <ArrayS<&str> as ArraySeqChap19>::iterate(
        &a,
        |acc: &String, x: &&str| format!("{}{}", acc, x),
        String::new(),
    );
    assert_eq!(res, "abc");
}

#[test]
fn test_map_empty() {
    let e: ArrayS<N> = <ArrayS<N> as ArraySeqChap19>::tabulate(|_| 0, 0);
    let m = <ArrayS<N> as ArraySeqChap19>::map(&e, |x| x + 1);
    assert_eq!(m.length(), 0);
}

#[test]
fn test_append_with_empty() {
    let e: ArrayS<N> = <ArrayS<N> as ArraySeqChap19>::tabulate(|_| 0, 0);
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 3);
    let left = <ArrayS<N> as ArraySeqChap19>::append(&e, &a);
    assert_eq!(left, apas_ai::arrayseq![1, 2, 3]);
    let right = <ArrayS<N> as ArraySeqChap19>::append(&a, &e);
    assert_eq!(right, apas_ai::arrayseq![1, 2, 3]);
}

#[test]
fn test_append2_equivalence() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 3);
    let b = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 4, 2);
    let c1 = <ArrayS<N> as ArraySeqChap19>::append(&a, &b);
    let c2 = <ArrayS<N> as ArraySeqChap19>::append2(&a, &b);
    assert_eq!(c1, c2);
}

#[test]
fn test_filter_empty_sequence() {
    let e: ArrayS<N> = <ArrayS<N> as ArraySeqChap19>::tabulate(|_| 0, 0);
    let f = <ArrayS<N> as ArraySeqChap19>::filter(&e, |_| B::True);
    assert_eq!(f.length(), 0);
}

#[test]
fn test_select_boundary() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 2);
    let b = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 3, 2);
    assert_eq!(<ArrayS<N> as ArraySeqChap19>::select(&a, &b, 0), Some(&1));
    assert_eq!(<ArrayS<N> as ArraySeqChap19>::select(&a, &b, 1), Some(&2));
    assert_eq!(<ArrayS<N> as ArraySeqChap19>::select(&a, &b, 2), Some(&3));
    assert_eq!(<ArrayS<N> as ArraySeqChap19>::select(&a, &b, 3), Some(&4));
    assert_eq!(<ArrayS<N> as ArraySeqChap19>::select(&a, &b, 4), None);
}

#[test]
fn test_subseq_basic() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| (i + 1) * 10, 5);
    let s = a.subseq_copy(1, 3);
    assert_eq!(s, apas_ai::arrayseq![20, 30, 40]);
}

#[test]
fn test_reduce_sum_basic_ch19() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ArrayS<N> as ArraySeqChap19>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let e: ArrayS<N> = <ArrayS<N> as ArraySeqChap19>::tabulate(|_| 0, 0);
    let re = <ArrayS<N> as ArraySeqChap19>::reduce(&e, &sum_fn, 42);
    assert_eq!(re, 42);
    let s = <ArrayS<N> as ArraySeqChap19>::tabulate(|_| 7, 1);
    let rs = <ArrayS<N> as ArraySeqChap19>::reduce(&s, &sum_fn, 0);
    assert_eq!(rs, 7);
}

#[test]
fn test_scan_sum_basic_ch19() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let (prefixes, total) = <ArrayS<N> as ArraySeqChap19>::scan(&a, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(total, 15);
}

#[test]
fn test_flatten_ch19() {
    let s1 = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 2);
    let s2 = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 3, 2);
    let nested: ArrayS<ArrayS<N>> = apas_ai::arrayseq![s1, s2];
    let flat = <ArrayS<N> as ArraySeqChap19>::flatten(&nested);
    assert_eq!(flat, apas_ai::arrayseq![1, 2, 3, 4]);
}

#[test]
fn test_atomic_write_lowest_wins_serial() {
    let len: N = 3;
    let base = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, len);
    let mut values_with_changenum: ArrayS<(N, N)> = <ArrayS<(N, N)> as ArraySeqChap19>::tabulate(
        |i| (base.nth(i).clone(), len),
        len,
    );
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(1, 33), (1, 44)];
    <ArrayS<N> as ArraySeqChap19>::atomicWrite(&mut values_with_changenum, &changes, 1);
    <ArrayS<N> as ArraySeqChap19>::atomicWrite(&mut values_with_changenum, &changes, 0);
    let (v, c) = values_with_changenum.nth(1);
    assert_eq!((*v, *c), (33, 0));
}

#[test]
fn test_atomic_write_highest_wins_mutex() {
    let len: N = 3;
    let base = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, len);
    let values_with_changenum: ArrayS<Mutex<(N, N)>> = <ArrayS<Mutex<(N, N)>> as ArraySeqChap19>::tabulate(
        |i| Mutex::new((base.nth(i).clone(), 0)),
        len,
    );
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(1, 33), (1, 44)];
    <ArrayS<Mutex<(N, N)>> as ArraySeqChap19>::AtomicWriteHighestChangeNumberWins(
        &values_with_changenum,
        &changes,
        0,
    );
    <ArrayS<Mutex<(N, N)>> as ArraySeqChap19>::AtomicWriteHighestChangeNumberWins(
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
    let e: ArrayS<N> = <ArrayS<N> as ArraySeqChap19>::tabulate(|_| 0, 0);
    let sum = <ArrayS<N> as ArraySeqChap19>::iterate(&e, |acc: &N, x: &N| acc + x, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_inject_parallel2_equivalence_leftmost_wins() {
    let values = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 6);
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(2, 99), (2, 7), (4, 20)];
    let serial = <ArrayS<N> as ArraySeqChap19>::inject(&values, &changes);
    let parallel = <ArrayS<N> as ArraySeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);
}

#[test]
fn test_inject_parallel2_out_of_bounds_ignored() {
    let values = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, 3);
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(10, 42), (1, 99)];
    let out = <ArrayS<N> as ArraySeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(out, apas_ai::arrayseq![1, 99, 3]);
}

#[test]
fn test_inject_parallel2_empty_values_and_empty_changes() {
    let values: ArrayS<N> = <ArrayS<N> as ArraySeq<N>>::empty();
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![];
    let out = <ArrayS<N> as ArraySeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(out.length(), 0);
}

#[test]
fn test_inject_parallel2_empty_values_nonempty_changes() {
    let values: ArrayS<N> = <ArrayS<N> as ArraySeq<N>>::empty();
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(0, 10), (1, 20)];
    let out = <ArrayS<N> as ArraySeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(out.length(), 0);
}

#[test]
fn test_inject_parallel2_nonempty_values_empty_changes() {
    let values = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 10, 4);
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![];
    let out = <ArrayS<N> as ArraySeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(out, values);
}

#[test]
fn test_actually_atomic_write_ordering() {
    let length: N = 3;
    let base: ArrayS<N> = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 1, length);
    let values_with_changenumber: ArrayS<Mutex<(N, N)>> = <ArrayS<Mutex<(N, N)>> as ArraySeqChap19>::tabulate(
        |i| Mutex::new((base.nth(i).clone(), length)),
        length,
    );
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(1, 33), (1, 44)];
    <ArrayS<Mutex<(N, N)>> as ArraySeqChap19>::AtomicWriteLowestChangeNumberWins(
        &values_with_changenumber,
        &changes,
        1,
    );
    <ArrayS<Mutex<(N, N)>> as ArraySeqChap19>::AtomicWriteLowestChangeNumberWins(
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
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 5);
    let updates: ArrayS<(N, N)> = apas_ai::arrayseq![(2, 99), (2, 7)];
    let out = <ArrayS<N> as ArraySeqChap19>::inject(&a, &updates);
    assert_eq!(out.length(), 5);
    assert_eq!(*out.nth(0), 0);
    assert_eq!(*out.nth(1), 1);
    assert_eq!(*out.nth(2), 99);
    assert_eq!(*out.nth(3), 3);
    assert_eq!(*out.nth(4), 4);
}

#[test]
fn test_inject_empty_updates_no_change() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i + 10, 4);
    let empty: ArrayS<(N, N)> = apas_ai::arrayseq![];
    let out = <ArrayS<N> as ArraySeqChap19>::inject(&a, &empty);
    assert_eq!(out.length(), 4);
    for i in 0..4 { assert_eq!(*out.nth(i), *a.nth(i)); }
}

#[test]
fn test_inject_parallel_equivalence() {
    let a = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 6);
    let updates: ArrayS<(N, N)> = apas_ai::arrayseq![(1, 10), (4, 20), (1, 99)];
    let seq1 = <ArrayS<N> as ArraySeqChap19>::inject(&a, &updates);
    let seq2 = <ArrayS<N> as ArraySeqChap19>::inject_parallel2(&a, &updates);
    assert_eq!(seq1.length(), seq2.length());
    for i in 0..seq1.length() { assert_eq!(seq1.nth(i), seq2.nth(i)); }
}

#[test]
fn test_ninject_parallel2_equivalence_last_wins() {
    let values = <ArrayS<N> as ArraySeqChap19>::tabulate(|i| i, 5);
    let changes: ArrayS<(N, N)> = apas_ai::arrayseq![(2, 99), (2, 7)];
    let serial = <ArrayS<N> as ArraySeqChap18>::ninject(&values, &changes);
    let parallel = <ArrayS<N> as ArraySeqChap19>::ninject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);
}


