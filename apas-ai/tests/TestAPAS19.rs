//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Sequences::*;
use apas_ai::APAS19::APAS19;
use apas_ai::Sequences::Sequence;
use apas_ai::APAS18::APAS18;
use std::sync::Mutex;

#[test]
fn test_empty() {
    let e = <S<N> as Sequence<N>>::empty();
    assert_eq!(e.length(), 0);
}

#[test]
fn test_singleton() {
    let s: S<i32> = <S<i32> as Sequence<i32>>::singleton(42);
    assert_eq!(s, apas_ai::seq![42]);
}

#[test]
fn test_map() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 5);
    let b = <S<i32> as APAS19>::map(&a, |x| x * 2);
    assert_eq!(b, apas_ai::seq![2, 4, 6, 8, 10]);
}

#[test]
fn test_append() {
    let a = <S<N> as APAS19>::tabulate(|i| i + 1, 3);
    let b = <S<N> as APAS19>::tabulate(|i| i + 4, 2);
    let c = <S<N> as APAS19>::append(&a, &b);
    assert_eq!(c, apas_ai::seq![1, 2, 3, 4, 5]);
}

#[test]
fn test_append2() {
    let a = <S<N> as APAS19>::tabulate(|i| i + 1, 3);
    let b = <S<N> as APAS19>::tabulate(|i| i + 4, 2);
    let c = <S<N> as APAS19>::append2(&a, &b);
    assert_eq!(c, apas_ai::seq![1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate_true() {
    let y = <S<i32> as APAS19>::deflate(|&x: &i32| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(y, apas_ai::seq![6]);
}

#[test]
fn test_deflate_false() {
    let y = <S<i32> as APAS19>::deflate(|&x: &i32| if x % 2 == 0 { B::True } else { B::False }, &5);
    assert_eq!(y.length(), 0);
}

#[test]
fn test_filter_even_numbers() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 10);
    let evens = <S<i32> as APAS19>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, apas_ai::seq![2, 4, 6, 8, 10]);
}

#[test]
fn test_filter_none() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32, 5);
    let odds_only = <S<i32> as APAS19>::filter(&a, |x| if *x % 2 == 1 { B::True } else { B::False });
    assert_eq!(odds_only, apas_ai::seq![1, 3]);
}

#[test]
fn test_update_in_bounds() {
    let mut a = <S<N> as APAS19>::tabulate(|i| i + 1, 5);
    let _ = a.update((2, 99));
    assert_eq!(a, apas_ai::seq![1, 2, 99, 4, 5]);
}

#[test]
fn test_update_out_of_bounds() {
    let mut a = <S<N> as APAS19>::tabulate(|i| i + 1, 3);
    let _ = a.update((10, 7));
    assert_eq!(a, apas_ai::seq![1, 2, 3]);
}

#[test]
fn test_isEmpty() {
    let e = <S<N> as Sequence<N>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    let s = <S<N> as Sequence<N>>::singleton(7);
    assert_eq!(s.isEmpty(), B::False);
    let a = <S<N> as APAS19>::tabulate(|i| i, 2);
    assert_eq!(a.isEmpty(), B::False);
}

#[test]
fn test_isSingleton() {
    let e = <S<N> as Sequence<N>>::empty();
    assert_eq!(e.isSingleton(), B::False);
    let s = <S<N> as Sequence<N>>::singleton(7);
    assert_eq!(s.isSingleton(), B::True);
    let a = <S<N> as APAS19>::tabulate(|i| i, 2);
    assert_eq!(a.isSingleton(), B::False);
}

#[test]
fn test_iterate_sum() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 5);
    let sum = <S<i32> as APAS19>::iterate(&a, |acc: &i32, x: &i32| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_iterate_concat() {
    let a = <S<&str> as APAS19>::tabulate(|i| match i { 0 => "a", 1 => "b", _ => "c" }, 3);
    let res = <S<&str> as APAS19>::iterate(&a, |acc: &String, x: &&str| format!("{}{}", acc, x), String::new());
    assert_eq!(res, "abc");
}

#[test]
fn test_map_empty() {
    let e: S<i32> = <S<i32> as APAS19>::tabulate(|_| 0, 0);
    let m = <S<i32> as APAS19>::map(&e, |x| x + 1);
    assert_eq!(m.length(), 0);
}

#[test]
fn test_append_with_empty() {
    let e: S<i32> = <S<i32> as APAS19>::tabulate(|_| 0, 0);
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 3);
    let left = <S<i32> as APAS19>::append(&e, &a);
    assert_eq!(left, apas_ai::seq![1, 2, 3]);
    let right = <S<i32> as APAS19>::append(&a, &e);
    assert_eq!(right, apas_ai::seq![1, 2, 3]);
}

#[test]
fn test_append2_equivalence() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 3);
    let b = <S<i32> as APAS19>::tabulate(|i| i as i32 + 4, 2);
    let c1 = <S<i32> as APAS19>::append(&a, &b);
    let c2 = <S<i32> as APAS19>::append2(&a, &b);
    assert_eq!(c1, c2);
}

#[test]
fn test_filter_empty_sequence() {
    let e: S<i32> = <S<i32> as APAS19>::tabulate(|_| 0, 0);
    let f = <S<i32> as APAS19>::filter(&e, |_| B::True);
    assert_eq!(f.length(), 0);
}

#[test]
fn test_select_boundary() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 2);
    let b = <S<i32> as APAS19>::tabulate(|i| i as i32 + 3, 2);
    assert_eq!(<S<i32> as APAS19>::select(&a, &b, 0), Some(&1));
    assert_eq!(<S<i32> as APAS19>::select(&a, &b, 1), Some(&2));
    assert_eq!(<S<i32> as APAS19>::select(&a, &b, 2), Some(&3));
    assert_eq!(<S<i32> as APAS19>::select(&a, &b, 3), Some(&4));
    assert_eq!(<S<i32> as APAS19>::select(&a, &b, 4), None);
}

#[test]
fn test_subseq_basic() {
    let a = <S<i32> as APAS19>::tabulate(|i| (i + 1) as i32 * 10, 5);
    let s = <S<i32> as APAS19>::subseq(&a, 1, 4);
    assert_eq!(s, apas_ai::seq![20, 30, 40]);
}

#[test]
fn test_iterate_empty_returns_acc() {
    let e: S<i32> = <S<i32> as APAS19>::tabulate(|_| 0, 0);
    let sum = <S<i32> as APAS19>::iterate(&e, |acc: &i32, x: &i32| acc + x, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_inject_parallel2_equivalence_leftmost_wins() {
    let values = <S<i32> as APAS19>::tabulate(|i| i as i32, 6);
    let changes: S<(N, i32)> = apas_ai::seq![(2, 99), (2, 7), (4, 20)];
    let serial = <S<i32> as APAS19>::inject(&values, &changes);
    let parallel = <S<i32> as APAS19>::inject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);
}

#[test]
fn test_inject_parallel2_out_of_bounds_ignored() {
    let values = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, 3);
    let changes: S<(N, i32)> = apas_ai::seq![(10, 42), (1, 99)];
    let out = <S<i32> as APAS19>::inject_parallel2(&values, &changes);
    assert_eq!(out, apas_ai::seq![1, 99, 3]);
}

#[test]
fn test_inject_parallel2_empty_values_and_empty_changes() {
    let values: S<i32> = <S<i32> as Sequence<i32>>::empty();
    let changes: S<(N, i32)> = apas_ai::seq![];
    let out = <S<i32> as APAS19>::inject_parallel2(&values, &changes);
    assert_eq!(out.length(), 0);
}

#[test]
fn test_inject_parallel2_empty_values_nonempty_changes() {
    let values: S<i32> = <S<i32> as Sequence<i32>>::empty();
    let changes: S<(N, i32)> = apas_ai::seq![(0, 10), (1, 20)];
    let out = <S<i32> as APAS19>::inject_parallel2(&values, &changes);
    assert_eq!(out.length(), 0);
}

#[test]
fn test_inject_parallel2_nonempty_values_empty_changes() {
    let values = <S<i32> as APAS19>::tabulate(|i| i as i32 + 10, 4);
    let changes: S<(N, i32)> = apas_ai::seq![];
    let out = <S<i32> as APAS19>::inject_parallel2(&values, &changes);
    assert_eq!(out, values);
}

#[test]
fn test_actually_atomic_write_ordering() {
    let length: N = 3;
    let base: S<i32> = <S<i32> as APAS19>::tabulate(|i| i as i32 + 1, length);
    let values_with_changenumber: S<Mutex<(i32, N)>> = <S<Mutex<(i32, N)>> as APAS19>::tabulate(|i| Mutex::new((base.nth(i).clone(), length)), length);
    let changes: S<(N, i32)> = apas_ai::seq![(1, 33), (1, 44)];
    <S<Mutex<(i32, N)>> as APAS19>::AtomicWriteLowestChangeNumberWins(&values_with_changenumber, &changes, 1);
    <S<Mutex<(i32, N)>> as APAS19>::AtomicWriteLowestChangeNumberWins(&values_with_changenumber, &changes, 0);
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
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32, 5);
    let updates: S<(N, i32)> = apas_ai::seq![(2, 99), (2, 7)];
    let out = <S<i32> as APAS19>::inject(&a, &updates);
    assert_eq!(out.length(), 5);
    assert_eq!(*out.nth(0), 0);
    assert_eq!(*out.nth(1), 1);
    assert_eq!(*out.nth(2), 99);
    assert_eq!(*out.nth(3), 3);
    assert_eq!(*out.nth(4), 4);
}

#[test]
fn test_inject_empty_updates_no_change() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32 + 10, 4);
    let empty: S<(N, i32)> = apas_ai::seq![];
    let out = <S<i32> as APAS19>::inject(&a, &empty);
    assert_eq!(out.length(), 4);
    for i in 0..4 { assert_eq!(*out.nth(i), *a.nth(i)); }
}

#[test]
fn test_inject_parallel_equivalence() {
    let a = <S<i32> as APAS19>::tabulate(|i| i as i32, 6);
    let updates: S<(N, i32)> = apas_ai::seq![(1, 10), (4, 20), (1, 99)];
    let seq1 = <S<i32> as APAS19>::inject(&a, &updates);
    let seq2 = <S<i32> as APAS19>::inject_parallel2(&a, &updates);
    assert_eq!(seq1.length(), seq2.length());
    for i in 0..seq1.length() { assert_eq!(seq1.nth(i), seq2.nth(i)); }
}

#[test]
fn test_ninject_parallel2_equivalence_last_wins() {
    let values = <S<i32> as APAS19>::tabulate(|i| i as i32, 5);
    let changes: S<(N, i32)> = apas_ai::seq![(2, 99), (2, 7)];
    let serial = <S<i32> as APAS18>::ninject(&values, &changes);
    let parallel = <S<i32> as APAS19>::ninject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);
}


