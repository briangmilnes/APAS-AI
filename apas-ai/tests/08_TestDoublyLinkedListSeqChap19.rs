//! Tests for DoublyLinkedListSeq Chapter 19 algorithms.

use apas_ai::Types::{N, B, O};
use apas_ai::DoublyLinkedListSeq::{DoublyLinkedListS as ListS, DoublyLinkedListSeq as ListSeq};
use apas_ai::DoublyLinkedListSeqChap18::DoublyLinkedListSeqChap18 as ListSeqChap18;
use apas_ai::DoublyLinkedListSeqChap19::DoublyLinkedListSeqChap19 as ListSeqChap19;
use std::sync::Mutex;

#[test]
fn test_tabulate_map_append() {
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 3);
    let b = <ListS<N> as ListSeqChap19>::map(&a, |x| x * 2);
    assert_eq!(b.length(), 3);
    assert_eq!(*b.nth(0), 2);
    let c = <ListS<N> as ListSeqChap19>::append(&a, &b);
    assert_eq!(c.length(), 6);
}

#[test]
fn test_select_and_append2() {
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 2);
    let b = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 3, 2);
    assert_eq!(<ListS<N> as ListSeqChap19>::select(&a, &b, 0), Some(&1));
    assert_eq!(<ListS<N> as ListSeqChap19>::select(&a, &b, 3), Some(&4));
    assert_eq!(<ListS<N> as ListSeqChap19>::select(&a, &b, 4), None);
    let c = <ListS<N> as ListSeqChap19>::append2(&a, &b);
    assert_eq!(c.length(), 4);
}

#[test]
fn test_deflate_filter() {
    let keep6 = <ListS<N> as ListSeqChap19>::deflate(|&x| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(keep6.length(), 1);
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 6);
    let evens = <ListS<N> as ListSeqChap19>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens.length(), 3);
}

#[test]
fn test_iterate_reduce_scan() {
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ListS<N> as ListSeqChap19>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let (_p, t) = <ListS<N> as ListSeqChap19>::scan(&a, &sum_fn, 0);
    assert_eq!(t, 15);
}

#[test]
fn test_flatten_ch19_list() {
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 2);
    let b = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 3, 2);
    let mut outer = std::collections::LinkedList::new();
    outer.push_back(a);
    outer.push_back(b);
    let nested = ListS { data: outer };
    let flat = <ListS<N> as ListSeqChap19>::flatten(&nested);
    assert_eq!(flat.length(), 4);
}

#[test]
fn test_atomic_write_variants_list() {
    let len: N = 3;
    let base = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, len);
    let mut values_with_changenum: ListS<(N, N)> = <ListS<(N, N)> as ListSeqChap19>::tabulate(|i| (base.nth(i).clone(), len), len);
    let mut ch_ll = std::collections::LinkedList::new();
    ch_ll.push_back((1, 33));
    ch_ll.push_back((1, 44));
    let changes = ListS { data: ch_ll };
    <ListS<N> as ListSeqChap19>::atomicWrite(&mut values_with_changenum, &changes, 1);
    <ListS<N> as ListSeqChap19>::atomicWrite(&mut values_with_changenum, &changes, 0);
    let (v, c) = values_with_changenum.nth(1);
    assert_eq!((*v, *c), (33, 0));
}

#[test]
fn test_inject_and_parallel_equivalence() {
    let values = <ListS<N> as ListSeqChap19>::tabulate(|i| i, 6);
    let mut changes_ll = std::collections::LinkedList::new();
    changes_ll.push_back((2, 99));
    changes_ll.push_back((2, 7));
    let changes = ListS { data: changes_ll };
    let serial = <ListS<N> as ListSeqChap19>::inject(&values, &changes);
    let parallel = <ListS<N> as ListSeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(serial.length(), parallel.length());
}


