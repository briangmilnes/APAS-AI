pub mod TestLinkedListStEphChap19 {
    use apas_ai::LinkedListStEph; // macro import
    use apas_ai::LinkedListStEph::LinkedListStEph::*;
    use apas_ai::LinkedListStEphChap19::LinkedListStEphChap19::*;
    use apas_ai::LinkedListStEphSLit;
    use apas_ai::Types::Types::*;

    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {
        let mut iter = list.iter();
        for &value in expected {
            assert_eq!(iter.next().copied(), Some(value));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_eph_set_and_nth() {
        let mut a: LinkedListStEphS<N> = LinkedListStEphSLit![1; 3];
        let _ = a.set(1, 9);
        assert_eq!(*a.nth(1), 9);
    }

    #[test]
    fn test_eph_subseq_copy_and_display_debug() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1; 3];
        let sub = a.subseq_copy(1, 2);
        assert_eq!(sub.length(), 2);
        let v = LinkedListStEphSLit![1, 2, 3];
        assert_eq!(format!("{:?}", v), "[1, 2, 3]");
        assert_eq!(format!("{}", v), "[1, 2, 3]");
    }

    #[test]
    fn test_iter_inorder_collect_eph_ch19() {
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![2, 4, 6];
        expect_list(&a, &[2, 4, 6]);
    }

    #[test]
    fn test_tabulate_map_select_append_ch19() {
        let a: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::tabulate(|i| i, 4);
        let b: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::map(&a, |x| x + 10);
        expect_list(&b, &[10, 11, 12, 13]);
        let c = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::append(&a, &b);
        expect_list(&c, &[0, 1, 2, 3, 10, 11, 12, 13]);
        assert_eq!(
            <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::select(&a, &b, 6),
            Some(12)
        );
    }

    #[test]
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {
        let one = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::deflate(
            |x| if *x == 1 { B::True } else { B::False },
            &1,
        );
        expect_list(&one, &[1]);
        let a: LinkedListStEphS<N> = LinkedListStEphSLit![1, 2, 3, 4];
        let even = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::filter(&a, |x| {
            if *x % 2 == 0 { B::True } else { B::False }
        });
        expect_list(&even, &[2, 4]);
        let sum = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::reduce(&a, &|x, y| x + y, 0);
        assert_eq!(sum, 10);
        let (prefixes, total) = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::scan(&a, &|x, y| x + y, 0);
        assert_eq!(total, 10);
        expect_list(&prefixes, &[0, 1, 3, 6]);
        let outer: LinkedListStEphS<LinkedListStEphS<N>> =
            LinkedListStEphSLit![LinkedListStEphSLit![1], LinkedListStEphSLit![2, 3]];
        let flat = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::flatten(&outer);
        expect_list(&flat, &[1, 2, 3]);
        let ups: LinkedListStEphS<Pair<N, N>> = LinkedListStEphSLit![Pair(1, 9), Pair(2, 8)];
        let inj = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::inject(&a, &ups);
        expect_list(&inj, &[1, 9, 8, 4]);
    }
}
