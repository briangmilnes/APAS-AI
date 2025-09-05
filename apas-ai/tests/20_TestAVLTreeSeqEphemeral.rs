use apas_ai::Types::{N, B};
use apas_ai::AVLTreeSeqEphemeral::{AVLTreeSEphemeral};
use apas_ai::AVLTreeSeqEphemeralTrait as AVLTreeSeqEphemeral;

#[test]
fn test_avl_ephemeral_basic() {
    let mut t: AVLTreeSEphemeral<N> = <AVLTreeSEphemeral<N> as AVLTreeSeqEphemeral<N>>::new();
    assert_eq!(<AVLTreeSEphemeral<N> as AVLTreeSeqEphemeral<N>>::length(&t), 0);
    t.push_back(1); t.push_back(2); t.push_back(3);
    assert_eq!(<AVLTreeSEphemeral<N> as AVLTreeSeqEphemeral<N>>::length(&t), 3);
    assert_eq!(*<AVLTreeSEphemeral<N> as AVLTreeSeqEphemeral<N>>::nth(&t, 1), 2);
    let _ = <AVLTreeSEphemeral<N> as AVLTreeSeqEphemeral<N>>::set(&mut t, 1, 9).unwrap();
    assert_eq!(*<AVLTreeSEphemeral<N> as AVLTreeSeqEphemeral<N>>::nth(&t, 1), 9);
}


