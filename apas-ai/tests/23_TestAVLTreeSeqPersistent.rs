use apas_ai::Types::N;
use apas_ai::{AVLTreeSPersistent, AVLTreeSeqPersistentTrait, AVLTreeSeqPersistentChap18Trait};

#[test]
fn test_persistent_set_does_not_mutate() {
    let _t: AVLTreeSPersistent<N> = <AVLTreeSPersistent<N> as AVLTreeSeqPersistentTrait<N>>::singleton(7);
    let a: AVLTreeSPersistent<N> = <AVLTreeSPersistent<N> as AVLTreeSeqPersistentChap18Trait>::tabulate(|i| i, 4);
    let b = <AVLTreeSPersistent<N> as AVLTreeSeqPersistentTrait<N>>::set(&a, 1, 99).unwrap();
    assert_eq!(*<AVLTreeSPersistent<N> as AVLTreeSeqPersistentTrait<N>>::nth(&a, 1), 1);
    assert_eq!(*<AVLTreeSPersistent<N> as AVLTreeSeqPersistentTrait<N>>::nth(&b, 1), 99);
}

