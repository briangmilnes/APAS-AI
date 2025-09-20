pub mod TestLinkedListStEphChap18 {
    use apas_ai::LinkedListStEph; // macro import
    use apas_ai::LinkedListStEph::LinkedListStEph::*;
    use apas_ai::LinkedListStEphChap18::LinkedListStEphChap18::*;
    use apas_ai::Types::Types::*;
    use apas_ai::LinkedListStEphSLit;

    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {
        let mut iter = list.iter();
        for &value in expected {
            assert_eq!(iter.next().copied(), Some(value));
        }
        assert_eq!(iter.next(), None);
    }

    // Eph Chap18 algorithms are not implemented; we reference expected outcomes via simple constructions.

    #[test]
    fn test_construct_eph_from_vec() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2, 3];
        assert_eq!(a.length(), 3);
    }

    #[test]
    fn test_eph_is_empty_and_singleton() {
        let e: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::empty();
        assert_eq!(e.isEmpty(), B::True);
        let s = LinkedListStEphSLit![1];
        assert_eq!(s.isSingleton(), B::True);
    }

    #[test]
    fn test_eph_set_and_subseq_copy() {
        let mut a: LinkedListStEphS<N> = LinkedListStEphSLit![0; 3];
        let _ = a.set(1, 2);
        assert_eq!(*a.nth(1), 2);
        let sub = a.subseq_copy(1, 2);
        assert_eq!(sub.length(), 2);
    }

    #[test]
    fn test_iter_inorder_collect_eph_ch18() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 3, 5];
        expect_list(&a, &[1, 3, 5]);
    }

    #[test]
    fn test_tabulate_and_map_ch18() {
        let a: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::tabulate(|i| i, 5);
        let b: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::map(&a, |x| x + 1);
        expect_list(&b, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_append_ch18() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2];
        let b: LinkedListStEphS<N> = LinkedListStEphSLit![2, 3];
        let c: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::append(&a, &b);
        expect_list(&c, &[1, 2, 2, 3]);
    }

    #[test]
    fn test_filter_ch18() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2, 3, 4];
        let b = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::filter(&a, |x| {
            if *x % 2 == 0 {
                B::True
            } else {
                B::False
            }
        });
        expect_list(&b, &[2, 4]);
    }

    #[test]
    fn test_update_ch18() {
        let mut a: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::new(3, 0);
        let _ = a.update(Pair(1, 7).into());
        expect_list(&a, &[0, 7, 0]);
    }

    #[test]
    fn test_inject_and_ninject_ch18() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![0, 0, 0, 0];
        let ups: LinkedListStEphS<Pair<N, N>> = LinkedListStEphSLit![Pair(1, 9), Pair(1, 5), Pair(3, 6)];
        let inj = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::inject(&a, &ups);
        let ninj = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::ninject(&a, &ups);
        expect_list(&inj, &[0, 9, 0, 6]);
        expect_list(&ninj, &[0, 5, 0, 6]);
    }

    #[test]
    fn test_iterate_reduce_scan_ch18() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2, 3];
        let sum = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::reduce(&a, &|x, y| x + y, 0);
        assert_eq!(sum, 6);
        let pref = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::iterate(&a, |acc, x| acc + x, 0);
        assert_eq!(pref, 6);
        let (prefixes, total) = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::scan(&a, &|x, y| x + y, 0);
        assert_eq!(total, 6);
        expect_list(&prefixes, &[0, 1, 3]);
    }

    #[test]
    fn test_flatten_and_collect_ch18() {
        let x: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2];
        let y: LinkedListStEphS<N> = LinkedListStEphSLit![3];
        let outer: LinkedListStEphS<LinkedListStEphS<N>> = LinkedListStEphSLit![x, y];
        let flat = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::flatten(&outer);
        expect_list(&flat, &[1, 2, 3]);

        let pairs: LinkedListStEphS<Pair<N, N>> = LinkedListStEphSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)];
        let grouped = <LinkedListStEphS<Pair<N, LinkedListStEphS<N>>> as LinkedListStEphChap18Trait<
            Pair<N, LinkedListStEphS<N>>,
        >>::collect(&pairs, |a, b| {
            if a == b {
                O::Equal
            } else if a < b {
                O::Less
            } else {
                O::Greater
            }
        });
        // Expect keys 1 and 2 with their grouped lists
        assert_eq!(<LinkedListStEphS<Pair<N, LinkedListStEphS<N>>> as LinkedListStEphTrait<Pair<N, LinkedListStEphS<N>>>>::length(&grouped), 2);
    }
}
