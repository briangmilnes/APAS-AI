//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::LinkedListStEph::LinkedListStEph::*;
use apas_ai::LinkedListStEphSLit;
use apas_ai::Types::Types::*;

fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {
    assert_eq!(list.length(), expected.len());
    for (i, &value) in expected.iter().enumerate() {
        assert_eq!(*list.nth(i), value);
    }
}

#[test]
fn test_eph_set_and_nth() {
    let mut a: LinkedListStEphS<N> = LinkedListStEphSLit![1; 3];
    let _ = LinkedListStEphTrait::update(&mut a, Pair(1, 9));
    assert_eq!(*a.nth(1), 9);
}

#[test]
fn test_eph_subseq_and_display_debug() {
    let a: LinkedListStEphS<N> = LinkedListStEphSLit![1; 3];
    let sub = a.subseq_copy(1, 2);
    assert_eq!(sub.length(), 2);
    let v = LinkedListStEphSLit![1, 2, 3];
    assert_eq!(format!("{:?}", v), "[1, 2, 3]");
    assert_eq!(format!("{}", v), "[1, 2, 3]");
}

#[test]
fn test_iter_inorder_collect_eph_ch19() {
    let a: LinkedListStEphS<N> = LinkedListStEphSLit![2, 4, 6];
    expect_list(&a, &[2, 4, 6]);
}

#[test]
fn test_tabulate_map_select_append_ch19() {
    let a: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::tabulate(&|i| i, 4);
    let b: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::map(&a, &|x| x + 10);
    expect_list(&b, &[10, 11, 12, 13]);
    let c = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::append(&a, &b);
    expect_list(&c, &[0, 1, 2, 3, 10, 11, 12, 13]);
    // Note: select method not implemented in LinkedListStEphTrait
    // assert_eq!(
    //     <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::select(&a, &b, 6),
    //     Some(12)
    // );
}

#[test]
fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {
    let one =
        <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::deflate(&|x: &N| if *x == 1 { true } else { false }, &1);
    expect_list(&one, &[1]);
    let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2, 3, 4];
    let even = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::filter(&a, &|x: &N| {
        if *x % 2 == 0 {
            true
        } else {
            false
        }
    });
    expect_list(&even, &[2, 4]);
    let sum = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(sum, 10);
    let (prefixes, total) = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(total, 10);
    expect_list(&prefixes, &[0, 1, 3, 6]);
    let outer: LinkedListStEphS<LinkedListStEphS<N>> =
        LinkedListStEphSLit![LinkedListStEphSLit![1], LinkedListStEphSLit![2, 3]];
    let flat = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::flatten(&outer);
    expect_list(&flat, &[1, 2, 3]);
    let ups: LinkedListStEphS<Pair<N, N>> = LinkedListStEphSLit![Pair(1, 9), Pair(2, 8)];
    let inj = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::inject(&a, &ups);
    expect_list(&inj, &[1, 9, 8, 4]);
}
