//! Tests for DoublyLinkedListSeq Chapter 18 algorithms.

use apas_ai::Types::{N, B, O};
use apas_ai::DoublyLinkedListSeq::{DoublyLinkedListS as ListS, DoublyLinkedListSeq as ListSeq};
use apas_ai::DoublyLinkedListSeqChap18::DoublyLinkedListSeqChap18 as ListSeqChap18;

#[test]
fn test_tabulate_and_map() {
    let a = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 5);
    assert_eq!(a.length(), 5);
    let b = <ListS<N> as ListSeqChap18>::map(&a, |x| x + 1);
    assert_eq!(b.length(), 5);
    assert_eq!(*b.nth(0), 2);
    assert_eq!(*b.nth(4), 6);
}

#[test]
fn test_append_and_filter() {
    let a = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 3);
    let b = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 4, 2);
    let c = <ListS<N> as ListSeqChap18>::append(&a, &b);
    assert_eq!(c.length(), 5);
    let evens = <ListS<N> as ListSeqChap18>::filter(&c, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens.length(), 2);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
}

#[test]
fn test_update_inject_ninject() {
    let mut a = <ListS<N> as ListSeqChap18>::tabulate(|i| i, 5);
    let _ = <ListS<N> as ListSeqChap18>::update(&mut a, (2, 99));
    assert_eq!(*a.nth(2), 99);

    let updates = <ListS<(N, N)> as ListSeqChap18>::tabulate(|i| if i == 0 { (2, 7) } else { (10, 1) }, 2);
    let first = <ListS<N> as ListSeqChap18>::inject(&a, &updates);
    assert_eq!(*first.nth(2), 7);

    let updates2 = <ListS<(N, N)> as ListSeqChap18>::tabulate(|i| if i == 0 { (2, 5) } else { (2, 8) }, 2);
    let last = <ListS<N> as ListSeqChap18>::ninject(&a, &updates2);
    assert_eq!(*last.nth(2), 8);
}

#[test]
fn test_iterate_prefixes_reduce_scan_flatten() {
    let s = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 5);
    let sum_fn = |a: &N, b: &N| a + b;
    let r = <ListS<N> as ListSeqChap18>::reduce(&s, &sum_fn, 0);
    assert_eq!(r, 15);
    let (p, total) = <ListS<N> as ListSeqChap18>::scan(&s, &sum_fn, 0);
    assert_eq!(p.length(), 5);
    assert_eq!(total, 15);

    let a = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 2);
    let b = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 3, 1);
    let mut ll = std::collections::LinkedList::new();
    ll.push_back(a);
    ll.push_back(b);
    let nested = ListS { data: ll };
    let flat = <ListS<N> as ListSeqChap18>::flatten(&nested);
    assert_eq!(flat.length(), 3);
}

#[test]
fn test_iterate_sum_basic_list() {
    let s = <ListS<N> as ListSeqChap18>::tabulate(|i| i + 1, 5);
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ListS<N> as ListSeqChap18>::iterate(&s, sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_collect_groups_by_key_list() {
    let mut ll = std::collections::LinkedList::new();
    ll.push_back(("a", 1_usize));
    ll.push_back(("b", 2_usize));
    ll.push_back(("a", 3_usize));
    let pairs = ListS { data: ll };
    let grouped = <ListS<(&str, N)> as ListSeqChap18>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
}


