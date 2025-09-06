use apas_ai::Types::{N, B};
use apas_ai::AVLTreeSeqEph::{AVLTreeSeqEphS};
use apas_ai::AVLTreeSeqEphTrait as AVLTreeSeqEph;

#[test]
fn test_avl_ephemeral_basic() {
    let mut t: AVLTreeSeqEphS<N> = <AVLTreeSeqEphS<N> as AVLTreeSeqEph<N>>::new();
    assert_eq!(<AVLTreeSeqEphS<N> as AVLTreeSeqEph<N>>::length(&t), 0);
    t.push_back(1); t.push_back(2); t.push_back(3);
    assert_eq!(<AVLTreeSeqEphS<N> as AVLTreeSeqEph<N>>::length(&t), 3);
    assert_eq!(*<AVLTreeSeqEphS<N> as AVLTreeSeqEph<N>>::nth(&t, 1), 2);
    let _ = <AVLTreeSeqEphS<N> as AVLTreeSeqEph<N>>::set(&mut t, 1, 9).unwrap();
    assert_eq!(*<AVLTreeSeqEphS<N> as AVLTreeSeqEph<N>>::nth(&t, 1), 9);
}


