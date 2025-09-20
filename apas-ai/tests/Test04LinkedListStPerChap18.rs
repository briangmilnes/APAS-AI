pub mod TestLinkedListStPerChap18 {
    use apas_ai::LinkedListStPer; // macro import
    use apas_ai::LinkedListStPerSLit;
    use apas_ai::LinkedListStPer::LinkedListStPer::*;
    use apas_ai::LinkedListStPerChap18::LinkedListStPerChap18::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_tabulate() {
        let a = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::tabulate(|i| i * 2, 5);
        assert_eq!(a.length(), 5);
        assert_eq!(a.nth(3), &6);
    }

    #[test]
    fn test_map() {
        let a = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::tabulate(|i| i * 2, 5);
        let b = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::map(&a, |x| x + 1);
        assert_eq!(b.nth(0), &1);
        assert_eq!(b.nth(4), &9);
    }

    #[test]
    fn test_filter() {
        let a = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::tabulate(|i| i + 1, 5);
        let c = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::filter(&a, |x| {
            if *x % 2 == 1 {
                B::True
            } else {
                B::False
            }
        });
        assert_eq!(c, LinkedListStPerSLit![1, 3, 5]);
    }

    #[test]
    fn test_append() {
        let a = LinkedListStPerSLit![0, 2, 4, 6, 8];
        let b = LinkedListStPerSLit![1, 3, 5, 7, 9];
        let d = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::append(&a, &b);
        assert_eq!(d, LinkedListStPerSLit![0, 2, 4, 6, 8, 1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_update() {
        let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
        let upd = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::update(&a, Pair(1, 9));
        assert_eq!(upd, LinkedListStPerSLit![1, 9, 3, 2, 1]);
    }

    #[test]
    fn test_inject() {
        let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
        let changes = LinkedListStPerSLit![Pair(0, 7), Pair(3, 5)];
        let inj = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::inject(&a, &changes);
        assert_eq!(inj, LinkedListStPerSLit![7, 2, 3, 5, 1]);
    }

    #[test]
    fn test_ninject() {
        let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
        let changes = LinkedListStPerSLit![Pair(0, 7), Pair(3, 5)];
        let ninj = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::ninject(&a, &changes);
        assert_eq!(ninj, LinkedListStPerSLit![7, 2, 3, 5, 1]);
    }

    #[test]
    fn test_iterate() {
        let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
        let sum = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::iterate(&a, |acc, x| acc + x, 0);
        assert_eq!(sum, 9);
    }

    #[test]
    fn test_reduce() {
        let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
        let red = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::reduce(&a, &|x, y| x + y, 0);
        assert_eq!(red, 9);
    }

    #[test]
    fn test_scan() {
        let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
        let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::scan(&a, &|x, y| x + y, 0);
        assert_eq!(prefixes, LinkedListStPerSLit![0, 1, 3, 6, 8]);
        assert_eq!(total, 9);
    }

    #[test]
    fn test_flatten() {
        let nested = LinkedListStPerSLit![LinkedListStPerSLit![1, 2], LinkedListStPerSLit![3], LinkedListStPerSLit![4, 5],];
        let flat = <LinkedListStPerS<N> as LinkedListStPerChap18Trait<N>>::flatten(&nested);
        assert_eq!(flat, LinkedListStPerSLit![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_collect() {
        let pairs = LinkedListStPerSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)];
        let grouped =
            <LinkedListStPerS<Pair<N, N>> as LinkedListStPerChap18Trait<Pair<N, N>>>::collect(&pairs, |a, b| {
                if a < b {
                    O::Less
                } else if a > b {
                    O::Greater
                } else {
                    O::Equal
                }
            });
        // Expect two groups with keys 1 and 2
        assert_eq!(grouped.length(), 2);
    }
}
