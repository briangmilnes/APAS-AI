use apas_ai::Types::{N, B};
use apas_ai::ArraySeqEph::{ArraySeqEphS, ArraySeqEphTrait};
use apas_ai::ArraySeqEph;
use apas_ai::ArraySeqEphChap18::ArraySeqEphChap18Trait;
use apas_ai::ArraySeqEphChap19::ArraySeqEphChap19Trait;

#[test]
fn test_ephemeral_arrayseq_basic() {
    let mut s: ArraySeqEphS<N> = ArraySeqEph![1; 3];
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 1);
    let _ = s.set(1, 9).unwrap();
    assert_eq!(*s.nth(1), 9);
}

#[test]
fn test_ephemeral_ch18_map_append_filter() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::tabulate(|i| i, 5);
    let m = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::map(&a, |x| x + 1);
    let c = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::append(&a, &m);
    assert_eq!(c.length(), 10);
    let evens = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::filter(&c, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert!(evens.length() > 0);
}

#[test]
fn test_iterators_collect() {
    let s = ArraySeqEph![1, 2, 3];
    let collected: Vec<N> = s.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

