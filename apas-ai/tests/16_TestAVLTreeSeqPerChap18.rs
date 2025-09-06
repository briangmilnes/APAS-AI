//! Tests for AVLTreeSeq Chapter 18 algorithms.

use apas_ai::Types::{B, N};
use apas_ai::AVLTreeSeqPer::{AVLTreeSeqPerS, AVLTreeSeqPerTrait};
use apas_ai::AVLTreeSeqPerChap18Trait as AVLTreeSeqPerChap18;
use apas_ai::ArraySeqPer::{ArrayPerS, ArraySeqPerTrait};
use apas_ai::ArraySeqPer;

#[test]
fn test_tabulate_inorder() {
    let t: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::tabulate(|i| i, 5);
    assert_eq!(t.to_arrayseq(), ArraySeqPer![0, 1, 2, 3, 4]);
}

#[test]
fn test_map_increment() {
    let base: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::tabulate(|i| i, 4);
    let mapped: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::map(&base, |x| x + 1);
    assert_eq!(mapped.to_arrayseq(), ArraySeqPer![1, 2, 3, 4]);
}

#[test]
fn test_append_union() {
    let a: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::tabulate(|i| i, 4); // 0..3
    let b: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::tabulate(|i| i + 2, 4); // 2..5
    let u: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::append(&a, &b);
    assert_eq!(u.to_arrayseq(), ArraySeqPer![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_filter_even() {
    let base: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::tabulate(|i| i, 6);
    let evens: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18>::filter(&base, |x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens.to_arrayseq(), ArraySeqPer![0, 2, 4]);
}


