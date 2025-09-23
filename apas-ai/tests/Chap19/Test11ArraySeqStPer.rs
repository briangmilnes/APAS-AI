pub mod TestArraySeqPer {
    use apas_ai::ArraySeqStPer; // macro import
    use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerTrait;
    use apas_ai::ArraySeqStPerTrait;
    use apas_ai::ArrayStPerSLit;
    use apas_ai::Types::Types::*;
    use std::sync::Mutex;

    #[test]
    fn test_map_and_select_and_append() {
        let a = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i + 1, 3);
        let b = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::map(&a, |x| x * 2);
        assert_eq!(b, ArrayStPerSLit![2, 4, 6]);
        assert_eq!(
            <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::select(&a, &b, 0),
            Some(&1)
        );
        let c = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::append(&a, &b);
        assert_eq!(c, ArrayStPerSLit![1, 2, 3, 2, 4, 6]);
    }

    #[test]
    fn test_deflate_and_filter() {
        let y = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::deflate(
            |&x: &N| if x % 2 == 0 { B::True } else { B::False },
            &6,
        );
        assert_eq!(y, ArrayStPerSLit![6]);
        let a = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i + 1, 10);
        let evens =
            <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::filter(
                &a,
                |x| if *x % 2 == 0 { B::True } else { B::False },
            );
        assert_eq!(evens, ArrayStPerSLit![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_iterate_reduce_scan_flatten() {
        let a = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i + 1, 5);
        let sum = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
        assert_eq!(sum, 15);
        let sum_fn = |x: &N, y: &N| x + y;
        let r = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&a, &sum_fn, 0);
        assert_eq!(r, 15);
        let (prefixes, total) = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::scan(&a, &sum_fn, 0);
        assert_eq!(total, 15);
        assert_eq!(*prefixes.nth(4), 10);
        let nested: ArrayStPerS<ArrayStPerS<N>> = ArrayStPerSLit![
            <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i + 1, 2),
            <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i + 3, 2),
        ];
        let flat = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&nested);
        assert_eq!(flat, ArrayStPerSLit![1, 2, 3, 4]);
    }

    #[test]
    fn test_inject_and_parallel() {
        let values = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(|i| i, 6);
        let changes: ArrayStPerS<Pair<N, N>> = ArrayStPerSLit![Pair(2, 99), Pair(2, 7), Pair(4, 20)];
        let result = <ArrayStPerS<N> as ArraySeqStPerTrait<N>>::inject(&values, &changes);
        assert_eq!(result.nth(0), &0);
        assert_eq!(result.nth(1), &1);
        assert_eq!(result.nth(2), &99); // First update wins: Pair(2, 99) over Pair(2, 7)
        assert_eq!(result.nth(3), &3);
        assert_eq!(result.nth(4), &20);
    }
}
