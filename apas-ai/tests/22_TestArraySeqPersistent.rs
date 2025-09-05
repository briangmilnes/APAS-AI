use apas_ai::Types::N;
use apas_ai::{ArraySPersistent, ArraySeqPersistentTrait};

#[test]
fn test_new_set_persistent() {
    let a: ArraySPersistent<N> = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::new(3, 7);
    assert_eq!(a.length(), 3);
    let b = <ArraySPersistent<N> as ArraySeqPersistentTrait<N>>::set(&a, 1, 9).unwrap();
    assert_eq!(*a.nth(1), 7);
    assert_eq!(*b.nth(1), 9);
}

