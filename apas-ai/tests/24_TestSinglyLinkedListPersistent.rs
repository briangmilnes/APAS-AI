use apas_ai::Types::{N, B};
use apas_ai::{SLPersistent, SinglyListPersistentTrait};

#[test]
fn test_persistent_set_and_subseq() {
    let a: SLPersistent<N> = <SLPersistent<N> as SinglyListPersistentTrait<N>>::new(4, 7);
    assert_eq!(<SLPersistent<N> as SinglyListPersistentTrait<N>>::length(&a), 4);
    let b = <SLPersistent<N> as SinglyListPersistentTrait<N>>::set(&a, 2, 99).unwrap();
    assert_eq!(*<SLPersistent<N> as SinglyListPersistentTrait<N>>::nth(&a, 2), 7);
    assert_eq!(*<SLPersistent<N> as SinglyListPersistentTrait<N>>::nth(&b, 2), 99);
    let s = <SLPersistent<N> as SinglyListPersistentTrait<N>>::subseq_copy(&b, 1, 2);
    assert_eq!(<SLPersistent<N> as SinglyListPersistentTrait<N>>::length(&s), 2);
}

