pub mod TestArraySeqEph {

    use apas_ai::ArraySeqStEph; // macro import
    use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::ArraySeqStEphSLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_ephemeral_arrayseq_basic() {
        let mut s: ArraySeqStEphS<N> = ArraySeqStEphSLit![1; 3];
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(0), 1);
        let _ = s.set(1, 9).unwrap();
        assert_eq!(*s.nth(1), 9);
    }

    #[test]
    fn test_ephemeral_ch18_map_append_filter() {
        let a = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(|i| i, 5);
        let m = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::map(&a, |x| x + 1);
        let c = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&a, &m);
        assert_eq!(c.length(), 10);
        let evens = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::filter(&c, |x| {
            if *x % 2 == 0 { B::True } else { B::False }
        });
        assert!(evens.length() > 0);
    }

    #[test]
    fn test_iterators_collect() {
        let s = ArraySeqStEphSLit![1, 2, 3];
        let collected: Vec<N> = s.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
