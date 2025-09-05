use apas_ai::Types::{N, B};
use apas_ai::SLEphemeral as SinglyLinkedListEphemeral;
use apas_ai::SinglyLinkedListEphemeral::SinglyListEphemeral;

#[test]
fn test_sll_ephemeral_basic() {
    let mut s = <SinglyLinkedListEphemeral<N> as SinglyListEphemeral<N>>::new(3, 1);
    assert_eq!(<SinglyLinkedListEphemeral<N> as SinglyListEphemeral<N>>::length(&s), 3);
    assert_eq!(*<SinglyLinkedListEphemeral<N> as SinglyListEphemeral<N>>::nth(&s, 0), 1);
    let _ = <SinglyLinkedListEphemeral<N> as SinglyListEphemeral<N>>::set(&mut s, 1, 9).unwrap();
    assert_eq!(*<SinglyLinkedListEphemeral<N> as SinglyListEphemeral<N>>::nth(&s, 1), 9);
}


