//! Tests for AVLTreeSeq Chapter 19 algorithms.

use apas_ai::Types::{B, N};
use apas_ai::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
use apas_ai::AVLTreeSeqChap19::AVLTreeSeqChap19;

#[test]
fn test_tabulate_and_map() {
    let t: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap19>::tabulate(|i| i, 5);
    let m: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap19>::map(&t, |x| x * 2);
    assert_eq!(m.values_in_order(), vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_select_and_append() {
    let a: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap19>::tabulate(|i| i, 3); // 0..2
    let b: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap19>::tabulate(|i| i + 3, 3); // 3..5
    assert_eq!(<AVLTreeS<N> as AVLTreeSeqChap19>::select(&a, &b, 0), Some(0));
    assert_eq!(<AVLTreeS<N> as AVLTreeSeqChap19>::select(&a, &b, 4), Some(4));
    assert_eq!(<AVLTreeS<N> as AVLTreeSeqChap19>::select(&a, &b, 6), None);
    let c = <AVLTreeS<N> as AVLTreeSeqChap19>::append(&a, &b);
    assert_eq!(c.values_in_order(), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate_and_filter() {
    let t: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap19>::tabulate(|i| i, 6);
    let d = <AVLTreeS<N> as AVLTreeSeqChap19>::deflate(|x| if *x % 2 == 0 { B::True } else { B::False }, &2);
    assert_eq!(d.values_in_order(), vec![2]);
    let f = <AVLTreeS<N> as AVLTreeSeqChap19>::filter(&t, |x| if *x < 3 { B::True } else { B::False });
    assert_eq!(f.values_in_order(), vec![0, 1, 2]);
}


