use apas_ai::{SLList, B, N};
use apas_ai::SinglyLinkedList::SinglyList;

#[test]
fn test_empty_singleton_and_predicates() {
    let l: SLList<N> = <SLList<N> as SinglyList<N>>::empty();
    assert_eq!(<SLList<N> as SinglyList<N>>::length(&l), 0);
    assert_eq!(<SLList<N> as SinglyList<N>>::isEmpty(&l), B::True);
    let one = <SLList<N> as SinglyList<N>>::singleton(7);
    assert_eq!(<SLList<N> as SinglyList<N>>::isSingleton(&one), B::True);
}

#[test]
fn test_new_and_nth_set() {
    let mut l = <SLList<N> as SinglyList<N>>::new(3, 1);
    assert_eq!(*<SLList<N> as SinglyList<N>>::nth(&l, 0), 1);
    assert_eq!(*<SLList<N> as SinglyList<N>>::nth(&l, 2), 1);
    let _ = <SLList<N> as SinglyList<N>>::set(&mut l, 1, 9).unwrap();
    assert_eq!(*<SLList<N> as SinglyList<N>>::nth(&l, 1), 9);
}

#[test]
fn test_subseq_copy() {
    let l = <SLList<N> as SinglyList<N>>::new(5, 2);
    let sub = <SLList<N> as SinglyList<N>>::subseq_copy(&l, 1, 3);
    assert_eq!(<SLList<N> as SinglyList<N>>::length(&sub), 3);
    assert_eq!(*<SLList<N> as SinglyList<N>>::nth(&sub, 0), 2);
    assert_eq!(*<SLList<N> as SinglyList<N>>::nth(&sub, 2), 2);
}


