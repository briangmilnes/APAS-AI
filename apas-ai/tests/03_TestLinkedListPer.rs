use apas_ai::Types::{N, B};
use apas_ai::LinkedListPer::{LinkedListPerS, LinkedListPerTrait};

#[test]
fn test_empty_singleton_and_predicates() {
    let l: LinkedListPerS<N> = <LinkedListPerS<N> as LinkedListPerTrait<N>>::empty();
    assert_eq!(<LinkedListPerS<N> as LinkedListPerTrait<N>>::length(&l), 0);
    assert_eq!(<LinkedListPerS<N> as LinkedListPerTrait<N>>::isEmpty(&l), B::True);
    let one = <LinkedListPerS<N> as LinkedListPerTrait<N>>::singleton(7);
    assert_eq!(<LinkedListPerS<N> as LinkedListPerTrait<N>>::isSingleton(&one), B::True);
}

#[test]
fn test_new_and_nth_set() {
    let l = <LinkedListPerS<N> as LinkedListPerTrait<N>>::new(3, 1);
    assert_eq!(*<LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&l, 0), 1);
    assert_eq!(*<LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&l, 2), 1);
    let l2 = <LinkedListPerS<N> as LinkedListPerTrait<N>>::set(&l, 1, 9).unwrap();
    // original remains unchanged (persistent semantics)
    assert_eq!(*<LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&l, 1), 1);
    // updated copy has the change
    assert_eq!(*<LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&l2, 1), 9);
}

#[test]
fn test_subseq_copy() {
    let l = <LinkedListPerS<N> as LinkedListPerTrait<N>>::new(5, 2);
    let sub = <LinkedListPerS<N> as LinkedListPerTrait<N>>::subseq_copy(&l, 1, 3);
    assert_eq!(<LinkedListPerS<N> as LinkedListPerTrait<N>>::length(&sub), 3);
    assert_eq!(*<LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&sub, 0), 2);
    assert_eq!(*<LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&sub, 2), 2);
}

#[test]
fn test_from_vec_and_debug_format() {
    let l = LinkedListPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(<LinkedListPerS<N> as LinkedListPerTrait<N>>::length(&l), 3);
    assert_eq!(format!("{:?}", l), "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_nth_out_of_bounds_panics() {
    let l = <LinkedListPerS<N> as LinkedListPerTrait<N>>::new(2, 0);
    let _ = <LinkedListPerS<N> as LinkedListPerTrait<N>>::nth(&l, 2);
}

#[test]
fn test_display_impl() {
    let l = LinkedListPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(format!("{}", l), "[1, 2, 3]");
}

