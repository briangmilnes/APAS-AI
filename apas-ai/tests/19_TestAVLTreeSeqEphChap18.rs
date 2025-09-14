//! Tests for AVLTreeSeq Eph (Chap18-style operations built using base APIs).

pub mod TestAVLTreeSeqEphChap18 {
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqEph::ArraySeqEph::*;
use apas_ai::AVLTreeSeqEph::AVLTreeSeqEph::AVLTreeSeqEphS;
use apas_ai::ArraySeqEph; // macro import
use apas_ai::AVLTreeSeqEph; // macro import

#[test]
fn test_tabulate_inorder() {
    let t: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2, 3, 4];
    assert_eq!(t.to_arrayseq(), ArraySeqEph![0, 1, 2, 3, 4]);
}

#[test]
fn test_map_increment() {
    let base: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2, 3];
    let mapped_v: Vec<N> = base.iter().map(|x| *x + 1).collect();
    let mapped: AVLTreeSeqEphS<N> = AVLTreeSeqEphS::from_vec(mapped_v);
    assert_eq!(mapped.to_arrayseq(), ArraySeqEph![1, 2, 3, 4]);
}

#[test]
fn test_append_union() {
    let a: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2, 3];
    let b: AVLTreeSeqEphS<N> = AVLTreeSeqEph![2, 3, 4, 5];
    let mut vals: Vec<N> = a.iter().map(|x| *x).collect();
    for x in b.iter() { if !vals.contains(x) { vals.push(*x); } }
    let u = AVLTreeSeqEphS::from_vec(vals);
    assert_eq!(u.to_arrayseq(), ArraySeqEph![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_filter_even() {
    let base: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2, 3, 4, 5];
    let kept: Vec<N> = base.iter().filter(|x| **x % 2 == 0).map(|x| *x).collect();
    let evens: AVLTreeSeqEphS<N> = AVLTreeSeqEphS::from_vec(kept);
    assert_eq!(evens.to_arrayseq(), ArraySeqEph![0, 2, 4]);
}
}
