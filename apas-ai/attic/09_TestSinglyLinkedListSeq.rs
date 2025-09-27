//! Tests for SinglyLinkedListSeq core APIs.

use apas_ai::Types::{N, B};
use apas_ai::SinglyLinkedListSeq::{SinglyLinkedListS as ListS, SinglyLinkedListSeq as ListSeq};

#[test]
fn test_new_and_update() {
    let mut s = <ListS<N> as ListSeq<N>>::new(3, 7);
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 7);
    assert_eq!(*s.nth(1), 7);
    assert_eq!(*s.nth(2), 7);
    let _ = s.update((1, 9)).update((0, 2));
    assert_eq!(*s.nth(0), 2);
    assert_eq!(*s.nth(1), 9);
}

#[test]
fn test_empty_singleton_and_predicates() {
    let e: ListS<N> = <ListS<N> as ListSeq<N>>::empty();
    assert_eq!(e.length(), 0);
    assert_eq!(e.isEmpty(), true);
    assert_eq!(e.isSingleton(), false);

    let s = <ListS<N> as ListSeq<N>>::singleton(42);
    assert_eq!(s.length(), 1);
    assert_eq!(s.isEmpty(), false);
    assert_eq!(s.isSingleton(), true);
}

#[test]
fn test_set_bounds() {
    let mut s = <ListS<N> as ListSeq<N>>::new(3, 0);
    let _ = s.set(2, 1).unwrap();
    assert_eq!(*s.nth(2), 1);
}

#[test]
fn test_set_out_of_bounds_err() {
    let mut s = <ListS<N> as ListSeq<N>>::new(2, 0);
    let r = s.set(5, 1);
    assert!(r.is_err());
}

#[test]
#[should_panic]
fn test_nth_out_of_bounds_panics() {
    let s = <ListS<N> as ListSeq<N>>::new(2, 0);
    s.nth(2);
}

#[test]
fn test_subseq_copy() {
    let mut s = <ListS<N> as ListSeq<N>>::new(5, 0);
    let _ = s.set(0, 10).unwrap();
    let _ = s.set(1, 20).unwrap();
    let _ = s.set(2, 30).unwrap();
    let _ = s.set(3, 40).unwrap();
    let _ = s.set(4, 50).unwrap();
    let t = s.subseq_copy(1, 3);
    assert_eq!(t.length(), 3);
    assert_eq!(*t.nth(0), 20);
    assert_eq!(*t.nth(1), 30);
    assert_eq!(*t.nth(2), 40);
}


