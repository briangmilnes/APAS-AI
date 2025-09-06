/// Ephemeral singly linked list tests (LinkedListEph).
use apas_ai::Types::{B, N};
use apas_ai::LinkedListEph::{LinkedListEphS, LinkedListEphTrait};
use apas_ai::LinkedListEph;

#[test]
fn test_empty_singleton_and_predicates() {
    let l: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::empty();
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&l), 0);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::isEmpty(&l), B::True);
    let one = <LinkedListEphS<N> as LinkedListEphTrait<N>>::singleton(7);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::isSingleton(&one), B::True);
}

#[test]
fn test_new_and_nth_set() {
    let mut l = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(3, 1);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&l, 0), 1);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&l, 2), 1);
    let _ = <LinkedListEphS<N> as LinkedListEphTrait<N>>::set(&mut l, 1, 9).unwrap();
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&l, 1), 9);
}

#[test]
fn test_subseq_copy() {
    let l = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(5, 2);
    let sub = <LinkedListEphS<N> as LinkedListEphTrait<N>>::subseq_copy(&l, 1, 3);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&sub), 3);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&sub, 0), 2);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&sub, 2), 2);
}

#[test]
fn test_linkedlisteph_basic() {
    let mut s = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(3, 1);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&s), 3);
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&s, 0), 1);
    let _ = <LinkedListEphS<N> as LinkedListEphTrait<N>>::set(&mut s, 1, 9).unwrap();
    assert_eq!(*<LinkedListEphS<N> as LinkedListEphTrait<N>>::nth(&s, 1), 9);
}

#[test]
fn test_debug_format_for_eph() {
    let l = LinkedListEph![1, 2, 3];
    assert_eq!(format!("{:?}", l), "[1, 2, 3]");
}

#[test]
fn test_display_format_for_eph() {
    let l = LinkedListEph![1, 2, 3];
    assert_eq!(format!("{}", l), "[1, 2, 3]");
}

#[test]
fn test_iter_inorder_collect_eph() {
    let l = LinkedListEph![5, 6, 7];
    let vals: Vec<N> = l.iter().copied().collect();
    assert_eq!(vals, vec![5, 6, 7]);
}

