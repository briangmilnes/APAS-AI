use apas_ai::Types::{N, B, O};
use apas_ai::LinkedListEph::{LinkedListEphS, LinkedListEphTrait};
use apas_ai::LinkedListEph;

// Eph Chap18 algorithms are not implemented; we reference expected outcomes via simple constructions.

#[test]
fn test_construct_eph_from_vec() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 2, 3];
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&a), 3);
}

#[test]
fn test_eph_is_empty_and_singleton() {
    let e: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::empty();
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::isEmpty(&e), B::True);
    let s = <LinkedListEphS<N> as LinkedListEphTrait<N>>::singleton(1);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::isSingleton(&s), B::True);
}

#[test]
fn test_eph_set_and_subseq_copy() {
    let mut a: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(3, 0);
    let _ = <LinkedListEphS<N> as LinkedListEphTrait<N>>::set(&mut a, 1, 2);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&a, 1), 2);
    let sub = <LinkedListEphS<N> as LinkedListEphTrait<N>>::subseq_copy(&a, 1, 2);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&sub), 2);
}

#[test]
fn test_iter_inorder_collect_eph_ch18() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 3, 5];
    let vals: Vec<N> = a.iter().copied().collect();
    assert_eq!(vals, vec![1, 3, 5]);
}

