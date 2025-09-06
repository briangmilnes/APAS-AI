use apas_ai::Types::N;
use apas_ai::{AVLTreeSeqPerS, AVLTreeSeqPerTrait, AVLTreeSeqPerChap18Trait};

#[test]
fn test_persistent_set_does_not_mutate() {
    let _t: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerTrait<N>>::singleton(7);
    let a: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18Trait>::tabulate(|i| i, 4);
    let b = <AVLTreeSeqPerS<N> as AVLTreeSeqPerTrait<N>>::set(&a, 1, 99).unwrap();
    assert_eq!(*<AVLTreeSeqPerS<N> as AVLTreeSeqPerTrait<N>>::nth(&a, 1), 1);
    assert_eq!(*<AVLTreeSeqPerS<N> as AVLTreeSeqPerTrait<N>>::nth(&b, 1), 99);
}

#[test]
fn test_iterator_inorder_values() {
    let a: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18Trait>::tabulate(|i| i + 1, 5);
    let vals: Vec<N> = a.iter().map(|x| *x).collect();
    assert_eq!(vals, vec![1, 2, 3, 4, 5]);
}

