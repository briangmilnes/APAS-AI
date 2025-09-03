//! Tests for SinglyLinkedListSeq Chapter 18 algorithms.

use apas_ai::Types::{N, B, O};
use apas_ai::SinglyLinkedListSeq::{SinglyLinkedListS as ListS, SinglyLinkedListSeq as ListSeq};
use apas_ai::SinglyLinkedListSeqChap18::SinglyLinkedListSeqChap18 as ListSeqChap18;

#[test]
fn test_tabulate_map_append_filter() {
    let a = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 5);
    let b = <ListS<N> as ListSeqChap18>::map(&a, |x| x + 1);
    let c = <ListS<N> as ListSeqChap18>::append(&a, &b);
    assert_eq!(c.length(), 10);
    let evens = <ListS<N> as ListSeqChap18>::filter(&c, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert!(evens.length() > 0);
}

#[test]
fn test_update_inject_ninject_iterate() {
    let mut a = <ListS<N> as ListSeqChap18>::tabulate(|i| i, 5);
    let _ = <ListS<N> as ListSeqChap18>::update(&mut a, (2, 99));
    let updates = <ListS<(N, N)> as ListSeqChap18>::tabulate(|i| if i == 0 { (2, 7) } else { (10, 1) }, 2);
    let first = <ListS<N> as ListSeqChap18>::inject(&a, &updates);
    assert_eq!(*first.nth(2), 7);
    let updates2 = <ListS<(N, N)> as ListSeqChap18>::tabulate(|i| if i == 0 { (2, 5) } else { (2, 8) }, 2);
    let last = <ListS<N> as ListSeqChap18>::ninject(&a, &updates2);
    assert_eq!(*last.nth(2), 8);
    let sum = <ListS<N> as ListSeqChap18>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
    assert!(sum > 0);
}

#[test]
fn test_scan_flatten_collect() {
    let a = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 5);
    let (_p, t) = <ListS<N> as ListSeqChap18>::scan(&a, &|x: &N, y: &N| x + y, 0);
    assert_eq!(t, 15);
    let b = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 2);
    let mut ll = std::collections::LinkedList::new(); ll.push_back(a); ll.push_back(b);
    let nested = ListS { data: ll };
    let flat = <ListS<N> as ListSeqChap18>::flatten(&nested);
    assert_eq!(flat.length(), 7);
    let mut pairs = std::collections::LinkedList::new(); pairs.push_back(("a", 1_usize)); pairs.push_back(("a", 2_usize));
    let grouped = <ListS<(&str, N)> as ListSeqChap18>::collect(&ListS { data: pairs }, |k1, k2| k1.cmp(k2));
    assert!(grouped.length() >= 1);
}


