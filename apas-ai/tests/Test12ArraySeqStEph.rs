pub mod TestArraySeqEph {

    use apas_ai::ArraySeqStEph; // macro import
    use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::ArraySeqStEphChap18::ArraySeqStEphChap18::*;
    use apas_ai::ArraySeqStEphChap19::ArraySeqStEphChap19::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_ephemeral_arrayseq_basic() {
        let mut s: ArraySeqStEphS<N> = ArraySeqStEph![1; 3];
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(0), 1);
        let _ = s.set(1, 9).unwrap();
        assert_eq!(*s.nth(1), 9);
    }

    #[test]
    fn test_ephemeral_ch18_map_append_filter() {
        let a = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::tabulate(|i| i, 5);
        let m = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::map(&a, |x| x + 1);
        let c = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::append(&a, &m);
        assert_eq!(c.length(), 10);
        let evens = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::filter(&c, |x| {
            if *x % 2 == 0 {
                B::True
            } else {
                B::False
            }
        });
        assert!(evens.length() > 0);
    }

    #[test]
    fn test_iterators_collect() {
        let s = ArraySeqStEph![1, 2, 3];
        let collected: Vec<N> = s.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
