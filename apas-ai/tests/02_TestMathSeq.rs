//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::{mathseq, MathSeq::MathS};
use apas_ai::Types::{N, B};
use apas_ai::MathSeq::MathSeq;

#[test]
fn test_length_and_nth_basic() {
    let a: MathS<N> = mathseq![10, 20, 30, 40];
    assert_eq!(a.length(), 4);
    assert_eq!(*a.nth(0), 10);
    assert_eq!(*a.nth(3), 40);
}

#[test]
fn test_add_last_and_delete_last() {
    let mut s: MathS<N> = mathseq![];
    let _ = s.add_last(1).add_last(2).add_last(3);
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 1);
    assert_eq!(s.delete_last(), Some(3));
    assert_eq!(s.length(), 2);
    assert_eq!(s.delete_last(), Some(2));
    assert_eq!(s.delete_last(), Some(1));
    assert_eq!(s.delete_last(), None);
}

#[test]
fn test_new_empty_singleton_and_predicates() {
    let e: MathS<N> = <MathS<N> as MathSeq<N>>::empty();
    assert_eq!(e.length(), 0);
    assert_eq!(<MathS<N> as MathSeq<N>>::isEmpty(&e), B::True);
    assert_eq!(<MathS<N> as MathSeq<N>>::isSingleton(&e), B::False);

    let s = <MathS<N> as MathSeq<N>>::singleton(7);
    assert_eq!(s.length(), 1);
    assert_eq!(<MathS<N> as MathSeq<N>>::isEmpty(&s), B::False);
    assert_eq!(<MathS<N> as MathSeq<N>>::isSingleton(&s), B::True);

    let mut a = <MathS<N> as MathSeq<N>>::new(3, 0);
    assert_eq!(a.length(), 3);
    assert_eq!(*a.nth(1), 0);
    let _ = a.set(1, 42);
    assert_eq!(*a.nth(1), 42);
}

#[test]
fn test_set_in_bounds_and_out_of_bounds() {
    let mut s: MathS<&str> = mathseq!["a", "b", "c"];
    assert!(s.set(1, "x").is_ok());
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(1), "x");
    assert!(s.set(5, "oops").is_err());
    assert_eq!(*s.nth(0), "a");
    assert_eq!(*s.nth(2), "c");
}

#[test]
fn test_subseq_view_bounds() {
    let s: MathS<N> = mathseq![1, 2, 3, 4, 5];
    let v = s.subseq(1, 3);
    assert_eq!(v, &[2, 3, 4]);
    let empty = s.subseq(2, 0);
    assert_eq!(empty.len(), 0);
    let tail = s.subseq(3, 10);
    assert_eq!(tail, &[4, 5]);
    let beyond = s.subseq(10, 5);
    assert_eq!(beyond.len(), 0);
}

#[test]
fn test_subseq_copy_bounds() {
    let s: MathS<N> = mathseq![1, 2, 3, 4, 5];
    let c1 = <MathS<N> as MathSeq<N>>::subseq_copy(&s, 1, 3);
    assert_eq!(c1.length(), 3);
    assert_eq!(*c1.nth(0), 2);
    let c2 = <MathS<N> as MathSeq<N>>::subseq_copy(&s, 10, 3);
    assert_eq!(c2.length(), 0);
}

#[test]
fn test_domain() {
    let s: MathS<N> = mathseq![7, 8, 9];
    assert_eq!(s.domain(), vec![0, 1, 2]);
}

#[test]
fn test_range_deduplicates_preserving_order() {
    let s: MathS<&str> = mathseq!["a", "b", "a", "c", "b", "d", "d"];
    let r = s.range();
    assert_eq!(r, vec!["a", "b", "c", "d"]);
}

#[test]
fn test_multiset_range_counts_first_occurrence_order() {
    let s: MathS<N> = mathseq![1, 2, 1, 3, 2, 1];
    let m = s.multiset_range();
    assert_eq!(m, vec![(3, 1), (2, 2), (1, 3)]);
}

#[test]
#[should_panic]
fn test_nth_out_of_bounds_panics() {
    let s: MathS<N> = mathseq![1, 2, 3];
    s.nth(3);
}

#[test]
fn test_range_empty_returns_empty() {
    let s: MathS<N> = mathseq![];
    let r: Vec<N> = s.range();
    assert_eq!(r.len(), 0);
}

#[test]
fn test_multiset_range_empty_returns_empty() {
    let s: MathS<N> = mathseq![];
    let m: Vec<(N, N)> = s.multiset_range();
    assert_eq!(m.len(), 0);
}

#[test]
fn test_domain_empty_is_empty() {
    let s: MathS<N> = mathseq![];
    let d = s.domain();
    assert_eq!(d.len(), 0);
}

