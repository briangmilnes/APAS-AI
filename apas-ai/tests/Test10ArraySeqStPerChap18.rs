pub mod TestArraySeqStPerChap18 {
    use apas_ai::ArraySeqStPer;
    use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
    use apas_ai::ArraySeqStPerChap18Trait;
    use apas_ai::Types::Types::Pair;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_tabulate_fibonacci() {
        fn fib(n: N) -> N {
            match n {
                0 => 0,
                1 => 1,
                _ => fib(n - 1) + fib(n - 2),
            }
        }
        let a: ArrayStPerS<N> = ArrayStPerS::tabulate(fib, 10);
        let fib10_head = ArraySeqStPer![
            *a.nth(0),
            *a.nth(1),
            *a.nth(2),
            *a.nth(3),
            *a.nth(4),
            *a.nth(5),
            *a.nth(6),
            *a.nth(7),
            *a.nth(8),
            *a.nth(9)
        ];
        assert_eq!(fib10_head, ArraySeqStPer![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(a.length(), 10);
    }

    #[test]
    fn test_map_increment() {
        let a = ArraySeqStPer![1, 2, 3, 4, 5];
        let b = <ArrayStPerS<i32> as ArraySeqStPerChap18Trait<i32>>::map(&a, |x| x + 1);
        assert_eq!(b, ArraySeqStPer![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_subseq() {
        let a = ArraySeqStPer![10, 20, 30, 40, 50];
        let b = a.subseq_copy(1, 3);
        assert_eq!(b, ArraySeqStPer![20, 30, 40]);
        let c = a.subseq_copy(2, 0);
        assert_eq!(c.length(), 0);
        let d = a.subseq_copy(0, 1);
        assert_eq!(d, ArraySeqStPer![10]);
    }

    #[test]
    fn test_append() {
        let a = ArraySeqStPer![1, 2, 3];
        let b = ArraySeqStPer![4, 5, 6];
        let c = <ArrayStPerS<i32> as ArraySeqStPerChap18Trait<i32>>::append(&a, &b);
        assert_eq!(c, ArraySeqStPer![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sequence_literals_and_append() {
        let a = ArraySeqStPer![1, 2, 3];
        let b = ArraySeqStPer![4, 5];
        let c = <ArrayStPerS<i32> as ArraySeqStPerChap18Trait<i32>>::append(&a, &b);
        assert_eq!(c, ArraySeqStPer![1, 2, 3, 4, 5]);
        let empty: ArrayStPerS<N> = ArrayStPerS::empty();
        let d = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::append(&ArraySeqStPer![1, 2, 3], &empty);
        assert_eq!(d.length(), 3);
        let e = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::append(&empty, &ArraySeqStPer![1, 2, 3]);
        assert_eq!(e.length(), 3);
    }

    #[test]
    fn test_filter_even() {
        let numbers = ArraySeqStPer![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let evens = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::filter(&numbers, |&x| {
            if x % 2 == 0 { B::True } else { B::False }
        });
        assert_eq!(evens, ArraySeqStPer![2, 4, 6, 8, 10]);
        let odds_only = ArraySeqStPer![1, 3, 5, 7];
        let no_evens = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::filter(&odds_only, |&x| {
            if x % 2 == 0 { B::True } else { B::False }
        });
        assert_eq!(no_evens.length(), 0);
    }

    // TODO: Fix flatten type inference - ArrayStPerS<ArrayStPerS<N>> trait bound issues
    #[test]
    fn test_flatten() {
        let a: ArrayStPerS<N> = ArraySeqStPer![1, 2];
        let b: ArrayStPerS<N> = ArraySeqStPer![3, 4, 5];
        let c: ArrayStPerS<N> = ArraySeqStPer![6];
        let sequences: ArrayStPerS<ArrayStPerS<N>> = ArraySeqStPer![a, b, c];
        let flattened = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::flatten(&sequences);
        assert_eq!(flattened, ArraySeqStPer![1, 2, 3, 4, 5, 6]);
        let empty: ArrayStPerS<N> = ArrayStPerS::empty();
        let mixed: ArrayStPerS<ArrayStPerS<N>> = ArraySeqStPer![ArraySeqStPer![1, 2], empty, ArraySeqStPer![3]];
        let mixed_flat = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::flatten(&mixed);
        assert_eq!(mixed_flat, ArraySeqStPer![1, 2, 3]);
    }

    #[test]
    fn test_update_sequence() {
        let a = ArraySeqStPer!["hello", "world", "test"];
        let b = <ArrayStPerS<&str> as ArraySeqStPerChap18Trait<&str>>::update(&a, Pair(1, "rust"));
        assert_eq!(b, ArraySeqStPer!["hello", "rust", "test"]);
        let c = ArraySeqStPer!["hello", "world", "test"];
        let d = <ArrayStPerS<&str> as ArraySeqStPerChap18Trait<&str>>::update(&c, Pair(5, "out_of_bounds"));
        assert_eq!(d, ArraySeqStPer!["hello", "world", "test"]);
    }

    #[test]
    fn test_inject_and_ninject() {
        let a = ArraySeqStPer!["the", "cat", "in", "the", "hat"];
        let updates = ArraySeqStPer![Pair(0, "a"), Pair(2, "on"), Pair(4, "mat")];
        let injected = <ArrayStPerS<&str> as ArraySeqStPerChap18Trait<&str>>::inject(&a, &updates);
        assert_eq!(injected.length(), 5);
        assert_eq!(injected, ArraySeqStPer!["a", "cat", "on", "the", "mat"]);

        let conflicting_updates = ArraySeqStPer![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")];
        let result_first = <ArrayStPerS<&str> as ArraySeqStPerChap18Trait<&str>>::inject(&a, &conflicting_updates);
        assert_eq!(result_first, ArraySeqStPer!["first", "updated", "in", "the", "hat"]);

        let ninjected = <ArrayStPerS<&str> as ArraySeqStPerChap18Trait<&str>>::ninject(
            &a,
            &ArraySeqStPer![Pair(1, "dog"), Pair(3, "big"), Pair(6, "hog")],
        );
        assert_eq!(ninjected, ArraySeqStPer!["the", "dog", "in", "big", "hat"]);
        assert_eq!(ninjected.length(), 5);
        let result_last = <ArrayStPerS<&str> as ArraySeqStPerChap18Trait<&str>>::ninject(
            &a,
            &ArraySeqStPer![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")],
        );
        assert_eq!(result_last, ArraySeqStPer!["second", "updated", "in", "the", "hat"]);
    }

    #[test]
    fn test_iterate_and_prefixes_and_reduce_and_scan() {
        let numbers = ArraySeqStPer![1, 2, 3, 4, 5];
        let sum_fn = |a: &N, b: &N| a + b;
        let result = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::reduce(&numbers, &sum_fn, 0);
        assert_eq!(result, 15);
        let empty: ArrayStPerS<N> = ArrayStPerS::empty();
        let empty_result = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::reduce(&empty, &sum_fn, 42);
        assert_eq!(empty_result, 42);
        let single = ArraySeqStPer![100];
        let single_result = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::reduce(&single, &sum_fn, 0);
        assert_eq!(single_result, 100);
        let (prefixes, final_result) = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::scan(&numbers, &sum_fn, 0);
        assert_eq!(prefixes.length(), 5);
        assert_eq!(*prefixes.nth(0), 0);
        assert_eq!(*prefixes.nth(4), 10);
        assert_eq!(final_result, 15);
    }

    #[test]
    fn test_iterate_sum_basic() {
        let numbers = ArraySeqStPer![1, 2, 3, 4, 5];
        let sum_fn = |a: &N, x: &N| a + x;
        let r = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::iterate(&numbers, sum_fn, 0);
        assert_eq!(r, 15);
    }

    #[test]
    fn test_iterate_prefixes_sum() {
        let numbers = ArraySeqStPer![1, 2, 3];
        let sum_fn = |a: &N, x: &N| a + x;
        let (prefixes, total) = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::iteratePrefixes(&numbers, sum_fn, 0);
        assert_eq!(prefixes.length(), 3);
        assert_eq!(*prefixes.nth(0), 0);
        assert_eq!(*prefixes.nth(1), 1);
        assert_eq!(*prefixes.nth(2), 3);
        assert_eq!(total, 6);
    }

    #[test]
    fn test_collect_groups_by_key() {
        // Use N, N pairs since collect requires same types
        let pairs = ArraySeqStPer![
            Pair(1_usize, 10_usize),
            Pair(2_usize, 20_usize),
            Pair(1_usize, 30_usize)
        ];
        let grouped: ArrayStPerS<Pair<N, ArrayStPerS<N>>> =
            <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::collect(&pairs, |k1, k2| k1.cmp(k2));
        assert_eq!(grouped.length(), 2);
        let pair0 = grouped.nth(0);
        assert_eq!(pair0.0, 1);
        assert_eq!(pair0.1, ArraySeqStPer![10, 30]);
        let pair1 = grouped.nth(1);
        assert_eq!(pair1.0, 2);
        assert_eq!(pair1.1, ArraySeqStPer![20]);
    }
}
