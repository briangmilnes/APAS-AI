pub mod TestLinkedListStEphChap19 {
    use apas_ai::LinkedListStEph; // macro import
    use apas_ai::LinkedListStEph::LinkedListStEph::*;
    use apas_ai::LinkedListStEphChap19::LinkedListStEphChap19::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_eph_set_and_nth() {
        let mut a: LinkedListStEphS<N> = LinkedListStEph![1; 3];
        let _ = a.set(1, 9);
        assert_eq!(*a.nth(1), 9);
    }

    #[test]
    fn test_eph_subseq_copy_and_display_debug() {
        let a: LinkedListStEphS<N> = LinkedListStEph![1; 3];
        let sub = a.subseq_copy(1, 2);
        assert_eq!(sub.length(), 2);
        let v = LinkedListStEph![1, 2, 3];
        assert_eq!(format!("{:?}", v), "[1, 2, 3]");
        assert_eq!(format!("{}", v), "[1, 2, 3]");
    }

    #[test]
    fn test_iter_inorder_collect_eph_ch19() {
        let a: LinkedListStEphS<N> = LinkedListStEph![2, 4, 6];
        let vals: Vec<N> = a.iter().copied().collect();
        assert_eq!(vals, vec![2, 4, 6]);
    }

    #[test]
    fn test_tabulate_map_select_append_ch19() {
        let a: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::tabulate(|i| i, 4);
        let b: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::map(&a, |x| x + 10);
        assert_eq!(b.iter().copied().collect::<Vec<N>>(), vec![10, 11, 12, 13]);
        let c = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::append(&a, &b);
        assert_eq!(c.iter().copied().collect::<Vec<N>>(), vec![0, 1, 2, 3, 10, 11, 12, 13]);
        assert_eq!(<LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::select(&a, &b, 6), Some(12));
    }

    #[test]
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {
        let one = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::deflate(
            |x| if *x == 1 { B::True } else { B::False },
            &1,
        );
        assert_eq!(one.iter().copied().collect::<Vec<N>>(), vec![1]);
        let a: LinkedListStEphS<N> = LinkedListStEph![1, 2, 3, 4];
        let even = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::filter(&a, |x| {
            if *x % 2 == 0 { B::True } else { B::False }
        });
        assert_eq!(even.iter().copied().collect::<Vec<N>>(), vec![2, 4]);
        let sum = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::reduce(&a, &|x, y| x + y, 0);
        assert_eq!(sum, 10);
        let (prefixes, total) = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::scan(&a, &|x, y| x + y, 0);
        assert_eq!(total, 10);
        assert_eq!(prefixes.iter().copied().collect::<Vec<N>>(), vec![0, 1, 3, 6]);
        let outer: LinkedListStEphS<LinkedListStEphS<N>> =
            LinkedListStEph![LinkedListStEph![1], LinkedListStEph![2, 3]];
        let flat = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::flatten(&outer);
        assert_eq!(flat.iter().copied().collect::<Vec<N>>(), vec![1, 2, 3]);
        let ups: LinkedListStEphS<Pair<N, N>> = LinkedListStEph![Pair(1, 9), Pair(2, 8)];
        let inj = <LinkedListStEphS<N> as LinkedListStEphChap19Trait<N>>::inject(&a, &ups);
        assert_eq!(inj.iter().copied().collect::<Vec<N>>(), vec![1, 9, 8, 4]);
    }
}
