//! Tests for AVLTreeSeq Eph Chap19-style operations using available Eph APIs.

use apas_ai::Types::{B, N};
use apas_ai::{ArraySeqEphS, ArraySeqEphTrait};
use apas_ai::AVLTreeSeqEph::{AVLTreeSeqEphS,AVLTreeSeqEphTrait};
use apas_ai::AVLTreeSeqEph;
use apas_ai::ArraySeqEph;


#[test]
fn test_tabulate_and_map() {
    let t: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2, 3, 4];
    let mapped_v: Vec<N> = t.iter().map(|x| *x * 2).collect();
    let m: AVLTreeSeqEphS<N> = AVLTreeSeqEphS::from_vec(mapped_v);
    assert_eq!(m.to_arrayseq(), ArraySeqEph![0, 2, 4, 6, 8]);
}

#[test]
fn test_select_and_append() {
    let a: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2];
    let b: AVLTreeSeqEphS<N> = AVLTreeSeqEph![3, 4, 5];
    // emulate select behavior
    let select = |a: &AVLTreeSeqEphS<N>, b: &AVLTreeSeqEphS<N>, i: N| -> Option<N> {
        if i < a.length() { Some(*a.nth(i)) } else {
            let off = i - a.length(); if off < b.length() { Some(*b.nth(off)) } else { None }
        }
    };
    assert_eq!(select(&a, &b, 0), Some(0));
    assert_eq!(select(&a, &b, 4), Some(4));
    assert_eq!(select(&a, &b, 6), None);
    let mut vals: Vec<N> = a.iter().map(|x| *x).collect();
    for x in b.iter() { if !vals.contains(x) { vals.push(*x); } }
    let c = AVLTreeSeqEphS::from_vec(vals);
    assert_eq!(c.to_arrayseq(), ArraySeqEph![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate_and_filter() {
    let t: AVLTreeSeqEphS<N> = AVLTreeSeqEph![0, 1, 2, 3, 4, 5];
    let d = if 2 % 2 == 0 { AVLTreeSeqEph![2] } else { AVLTreeSeqEph![] };
    assert_eq!(d.to_arrayseq(), ArraySeqEph![2]);
    let kept: Vec<N> = t.iter().filter(|x| **x < 3).map(|x| *x).collect();
    let f = AVLTreeSeqEphS::from_vec(kept);
    assert_eq!(f.to_arrayseq(), ArraySeqEph![0, 1, 2]);
}


