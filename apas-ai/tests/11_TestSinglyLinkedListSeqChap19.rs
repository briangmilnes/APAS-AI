//! Tests for SinglyLinkedListSeq Chapter 19 algorithms.

use apas_ai::Types::{N, B};
use apas_ai::SinglyLinkedListSeq::{SinglyLinkedListS as ListS, SinglyLinkedListSeq as ListSeq};
use apas_ai::SinglyLinkedListSeqChap18::SinglyLinkedListSeqChap18 as ListSeqChap18;
use apas_ai::SinglyLinkedListSeqChap19::SinglyLinkedListSeqChap19 as ListSeqChap19;
use std::sync::Mutex;

#[test]
fn test_map_append_select() {
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 3);
    let b = <ListS<N> as ListSeqChap19>::map(&a, |x| x * 2);
    let c = <ListS<N> as ListSeqChap19>::append(&a, &b);
    assert_eq!(c.length(), 6);
    assert_eq!(<ListS<N> as ListSeqChap19>::select(&a, &b, 0), Some(&1));
}

#[test]
fn test_reduce_scan_flatten() {
    let a = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 5);
    let r = <ListS<N> as ListSeqChap19>::reduce(&a, &|x: &N, y: &N| x + y, 0);
    assert_eq!(r, 15);
    let (_p, t) = <ListS<N> as ListSeqChap19>::scan(&a, &|x: &N, y: &N| x + y, 0);
    assert_eq!(t, 15);
    let b = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, 2);
    let mut ll = std::collections::LinkedList::new(); ll.push_back(a); ll.push_back(b);
    let nested = ListS { data: ll };
    let flat = <ListS<N> as ListSeqChap19>::flatten(&nested);
    assert_eq!(flat.length(), 7);
}

#[test]
fn test_atomic_write_and_inject_equivalence() {
    let len: N = 3;
    let base = <ListS<N> as ListSeqChap19>::tabulate(|i| i + 1, len);
    let mut values_with_changenum: ListS<(N, N)> = <ListS<(N, N)> as ListSeqChap19>::tabulate(|i| (base.nth(i).clone(), len), len);
    let mut ch_ll = std::collections::LinkedList::new(); ch_ll.push_back((1, 33)); ch_ll.push_back((1, 44));
    let changes = ListS { data: ch_ll };
    <ListS<N> as ListSeqChap19>::atomicWrite(&mut values_with_changenum, &changes, 1);
    <ListS<N> as ListSeqChap19>::atomicWrite(&mut values_with_changenum, &changes, 0);
    let (v, c) = values_with_changenum.nth(1); assert_eq!((*v, *c), (33, 0));

    let values = <ListS<N> as ListSeqChap19>::tabulate(|i| i, 6);
    let mut changes_ll = std::collections::LinkedList::new(); changes_ll.push_back((2, 99)); changes_ll.push_back((2, 7));
    let changes = ListS { data: changes_ll };
    let serial = <ListS<N> as ListSeqChap19>::inject(&values, &changes);
    let parallel = <ListS<N> as ListSeqChap19>::inject_parallel2(&values, &changes);
    assert_eq!(serial.length(), parallel.length());
}


