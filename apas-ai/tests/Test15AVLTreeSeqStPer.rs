pub mod TestAVLTreeSeqPer {
use apas_ai::Types::Types::*;
use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::AVLTreeSeqStPerChap18::AVLTreeSeqStPerChap18::*;

#[test]
fn test_persistent_set_does_not_mutate() {
    let _t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::singleton(7);
    let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i, 4);
    let b = a.set(1, 99).unwrap();
    assert_eq!(*a.nth(1), 1);
    assert_eq!(*b.nth(1), 99);
}

#[test]
fn test_iterator_inorder_values() {
    let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i + 1, 5);
    let vals: Vec<N> = a.iter().map(|x| *x).collect();
    assert_eq!(vals, vec![1, 2, 3, 4, 5]);
}

}
