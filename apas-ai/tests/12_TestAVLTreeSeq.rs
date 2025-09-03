//! Tests for AVLTreeSeq sequence-backed AVL structure.

use apas_ai::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
use apas_ai::Types::{B, N};

#[test]
fn test_insert_contains_delete() {
    let mut t: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    assert_eq!(<AVLTreeS<N> as AVLTreeSeq<N>>::length(&t), 0);
    assert_eq!(t.contains_value(&1), B::False);
    t.insert_value(1);
    assert_eq!(<AVLTreeS<N> as AVLTreeSeq<N>>::length(&t), 1);
    assert_eq!(t.contains_value(&1), B::True);
    assert!(t.delete_value(&1));
    assert_eq!(<AVLTreeS<N> as AVLTreeSeq<N>>::length(&t), 0);
}

#[test]
fn test_iter_and_range() {
    let mut t: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    for i in 0..5 { t.insert_value(i); }
    let arr = t.to_arrayseq();
    assert_eq!(arr, apas_ai::arrayseq![0, 1, 2, 3, 4]);
}


