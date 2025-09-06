use apas_ai::Types::{N, B};
use apas_ai::LinkedListEph::{LinkedListEphS, LinkedListEphTrait};

#[test]
fn test_eph_set_and_nth() {
    let mut a: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(3, 1);
    let _ = <LinkedListEphS<N> as LinkedListEphTrait<N>>::set(&mut a, 1, 9);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&a, 1), 9);
}

#[test]
fn test_eph_subseq_copy_and_display_debug() {
    let a: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(3, 1);
    let sub = <LinkedListEphS<N> as LinkedListEphTrait<N>>::subseq_copy(&a, 1, 2);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&sub), 2);
    let v = LinkedListEphS::from_vec(vec![1, 2, 3]);
    assert_eq!(format!("{:?}", v), "[1, 2, 3]");
    assert_eq!(format!("{}", v), "[1, 2, 3]");
}

