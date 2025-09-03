//! Tests for AVLTreeSeq Chapter 18 algorithms.

use apas_ai::Types::{B, N};
use apas_ai::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
use apas_ai::AVLTreeSeqChap18::AVLTreeSeqChap18;

#[test]
fn test_tabulate_inorder() {
    let t: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::tabulate(|i| i, 5);
    assert_eq!(t.values_in_order(), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_map_increment() {
    let base: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::tabulate(|i| i, 4);
    let mapped: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::map(&base, |x| x + 1);
    assert_eq!(mapped.values_in_order(), vec![1, 2, 3, 4]);
}

#[test]
fn test_append_union() {
    let a: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::tabulate(|i| i, 4); // 0..3
    let b: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::tabulate(|i| i + 2, 4); // 2..5
    let u: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::append(&a, &b);
    assert_eq!(u.values_in_order(), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_filter_even() {
    let base: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::tabulate(|i| i, 6);
    let evens: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap18>::filter(&base, |x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens.values_in_order(), vec![0, 2, 4]);
}


