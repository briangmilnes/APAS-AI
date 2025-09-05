use apas_ai::Types::{N, B};
use apas_ai::ArraySeqEphemeral::{ArraySEphemeral, ArraySeqEphemeral};
use apas_ai::ArraySeqEphemeralChap18::ArraySeqEphemeralChap18;
use apas_ai::ArraySeqEphemeralChap19::ArraySeqEphemeralChap19;

#[test]
fn test_ephemeral_arrayseq_basic() {
    let mut s: ArraySEphemeral<N> = <ArraySEphemeral<N> as ArraySeqEphemeral<N>>::new(3, 1);
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 1);
    let _ = <ArraySEphemeral<N> as ArraySeqEphemeral<N>>::set(&mut s, 1, 9).unwrap();
    assert_eq!(*s.nth(1), 9);
}

#[test]
fn test_ephemeral_ch18_map_append_filter() {
    let a = <ArraySEphemeral<N> as ArraySeqEphemeralChap18>::tabulate(|i| i, 5);
    let m = <ArraySEphemeral<N> as ArraySeqEphemeralChap18>::map(&a, |x| x + 1);
    let c = <ArraySEphemeral<N> as ArraySeqEphemeralChap18>::append(&a, &m);
    assert_eq!(c.length(), 10);
    let evens = <ArraySEphemeral<N> as ArraySeqEphemeralChap18>::filter(&c, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert!(evens.length() > 0);
}




