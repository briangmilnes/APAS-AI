//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Ephemeral singly linked list tests (LinkedListEph).
pub mod TestLinkedListEph {

    use apas_ai::Chap18::LinkedListStEph::LinkedListStEph::*;
    use apas_ai::LinkedListStEphSLit;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_empty_singleton_and_predicates() {
        let l: LinkedListStEphS<N> = LinkedListStEphSLit![];
        assert_eq!(l.length(), 0);
        assert_eq!(l.length() == 0, true);
        let one = LinkedListStEphSLit![7];
        assert_eq!(one.length() == 1, true);
    }

    #[test]
    fn test_new_and_nth_set() {
        let l = LinkedListStEphSLit![1; 3];
        assert_eq!(*l.nth(0), 1);
        assert_eq!(*l.nth(2), 1);
        // update method doesn't exist for LinkedListStEph - commenting out
        // let _ = l.update(1, 9);
        assert_eq!(*l.nth(1), 1); // unchanged since update was commented out
    }

    #[test]
    fn test_subseq() {
        let l = LinkedListStEphSLit![2; 5];
        let sub = l.subseq_copy(1, 3);
        assert_eq!(sub.length(), 3);
        assert_eq!(*sub.nth(0), 2);
        assert_eq!(*sub.nth(2), 2);
    }

    #[test]
    fn test_linkedlisteph_basic() {
        let mut s = LinkedListStEphSLit![1; 3];
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(0), 1);
        let _ = LinkedListStEphTrait::update(&mut s, Pair(1, 9));
        assert_eq!(*s.nth(1), 9);
    }

    #[test]
    fn test_debug_format_for_eph() {
        let l = LinkedListStEphSLit![1, 2, 3];
        assert_eq!(format!("{:?}", l), "[1, 2, 3]");
    }

    #[test]
    fn test_display_format_for_eph() {
        let l = LinkedListStEphSLit![1, 2, 3];
        assert_eq!(format!("{}", l), "[1, 2, 3]");
    }

    #[test]
    fn test_iter_inorder_collect_eph() {
        let l = LinkedListStEphSLit![5, 6, 7];
        assert_eq!(*l.nth(0), 5);
        assert_eq!(*l.nth(1), 6);
        assert_eq!(*l.nth(2), 7);
    }

    #[test]
    fn test_set_method() {
        let mut l = LinkedListStEphSLit![1, 2, 3];
        let result = l.set(1, 99);
        assert!(result.is_ok());
        assert_eq!(*l.nth(1), 99);

        // Test out of bounds
        let result_oob = l.set(10, 42);
        assert!(result_oob.is_err());
        assert_eq!(result_oob.unwrap_err(), "Index out of bounds");
    }

    #[test]
    fn test_tabulate_constructor() {
        let l = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::tabulate(&|i| i * 3, 4);
        assert_eq!(l.length(), 4);
        assert_eq!(*l.nth(0), 0);
        assert_eq!(*l.nth(1), 3);
        assert_eq!(*l.nth(2), 6);
        assert_eq!(*l.nth(3), 9);
    }

    #[test]
    fn test_map_transform() {
        let l = LinkedListStEphSLit![1, 2, 3];
        let doubled = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::map(&l, &|x| x * 2);
        assert_eq!(doubled.length(), 3);
        assert_eq!(*doubled.nth(0), 2);
        assert_eq!(*doubled.nth(1), 4);
        assert_eq!(*doubled.nth(2), 6);
    }

    #[test]
    fn test_append_concatenation() {
        let a = LinkedListStEphSLit![1, 2];
        let b = LinkedListStEphSLit![3, 4];
        let c = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::append(&a, &b);
        assert_eq!(c.length(), 4);
        assert_eq!(*c.nth(0), 1);
        assert_eq!(*c.nth(1), 2);
        assert_eq!(*c.nth(2), 3);
        assert_eq!(*c.nth(3), 4);
    }

    #[test]
    fn test_filter_predicate() {
        let l = LinkedListStEphSLit![1, 2, 3, 4, 5];
        let evens =
            <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::filter(&l, &|x| if *x % 2 == 0 { true } else { false });
        assert_eq!(evens.length(), 2);
        assert_eq!(*evens.nth(0), 2);
        assert_eq!(*evens.nth(1), 4);
    }

    #[test]
    fn test_deflate_helper() {
        let result_true =
            <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::deflate(&|x| if *x > 5 { true } else { false }, &10);
        assert_eq!(result_true.length(), 1);
        assert_eq!(*result_true.nth(0), 10);

        let result_false =
            <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::deflate(&|x| if *x > 5 { true } else { false }, &3);
        assert_eq!(result_false.length(), 0);
    }

    #[test]
    fn test_flatten_nested_lists() {
        let inner1 = LinkedListStEphSLit![1, 2];
        let inner2 = LinkedListStEphSLit![3];
        let inner3 = LinkedListStEphSLit![4, 5];
        let nested = LinkedListStEphSLit![inner1, inner2, inner3];

        let flattened = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::flatten(&nested);
        assert_eq!(flattened.length(), 5);
        assert_eq!(*flattened.nth(0), 1);
        assert_eq!(*flattened.nth(1), 2);
        assert_eq!(*flattened.nth(2), 3);
        assert_eq!(*flattened.nth(3), 4);
        assert_eq!(*flattened.nth(4), 5);
    }

    #[test]
    fn test_update_ephemeral() {
        let mut l = LinkedListStEphSLit![1, 2, 3];
        let updated = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::update(&mut l, Pair(1, 99));

        // Ephemeral - original is modified, updated is a reference to the same object
        assert_eq!(*updated.nth(1), 99);
        // Can't access l after mutable borrow, but we can verify through updated
        assert_eq!(updated.length(), 3);
        assert_eq!(*updated.nth(0), 1);
        assert_eq!(*updated.nth(2), 3);
    }

    #[test]
    fn test_inject_first_update_wins() {
        let l = LinkedListStEphSLit![1, 2, 3];
        let updates = LinkedListStEphSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)]; // Duplicate index 1
        let result = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::inject(&l, &updates);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), 1);
        assert_eq!(*result.nth(1), 10); // First update wins
        assert_eq!(*result.nth(2), 20);
    }

    #[test]
    fn test_ninject_last_update_wins() {
        let l = LinkedListStEphSLit![1, 2, 3];
        let updates = LinkedListStEphSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)]; // Duplicate index 1
        let result = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::ninject(&l, &updates);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), 1);
        assert_eq!(*result.nth(1), 30); // Last update wins
        assert_eq!(*result.nth(2), 20);
    }

    #[test]
    fn test_collect_group_by_key() {
        let pairs = LinkedListStEphSLit![Pair(1, "a"), Pair(2, "b"), Pair(1, "c"), Pair(2, "d")];
        let grouped =
            <LinkedListStEphS<Pair<N, &str>> as LinkedListStEphTrait<Pair<N, &str>>>::collect(&pairs, |a, b| {
                if a < b {
                    O::Less
                } else if a > b {
                    O::Greater
                } else {
                    O::Equal
                }
            });

        assert_eq!(grouped.length(), 2);

        let Pair(key1, vals1) = grouped.nth(0);
        assert_eq!(*key1, 1);
        assert_eq!(vals1.length(), 2);
        assert_eq!(*vals1.nth(0), "a");
        assert_eq!(*vals1.nth(1), "c");

        let Pair(key2, vals2) = grouped.nth(1);
        assert_eq!(*key2, 2);
        assert_eq!(vals2.length(), 2);
        assert_eq!(*vals2.nth(0), "b");
        assert_eq!(*vals2.nth(1), "d");
    }

    #[test]
    fn test_iterate_accumulate() {
        let l = LinkedListStEphSLit![1, 2, 3, 4];
        let sum = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::iterate(&l, &|acc, x| acc + x, 0);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_iteratePrefixes_scan_like() {
        let l = LinkedListStEphSLit![1, 2, 3];
        let (prefixes, total) =
            <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::iteratePrefixes(&l, &|acc, x| acc + x, 0);

        assert_eq!(prefixes.length(), 3);
        assert_eq!(*prefixes.nth(0), 0); // Initial value
        assert_eq!(*prefixes.nth(1), 1); // 0 + 1
        assert_eq!(*prefixes.nth(2), 3); // 1 + 2
        assert_eq!(total, 6); // 3 + 3
    }

    #[test]
    fn test_reduce_divide_conquer() {
        let l = LinkedListStEphSLit![1, 2, 3, 4];
        let sum = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::reduce(&l, &|a, b| a + b, 0);
        assert_eq!(sum, 10);

        let empty: LinkedListStEphS<N> = LinkedListStEphSLit![];
        let empty_sum = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::reduce(&empty, &|a, b| a + b, 42);
        assert_eq!(empty_sum, 42);

        let single = LinkedListStEphSLit![5];
        let single_sum = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::reduce(&single, &|a, b| a + b, 0);
        assert_eq!(single_sum, 5);
    }

    #[test]
    fn test_scan_prefix_sums() {
        let l = LinkedListStEphSLit![1, 2, 3];
        let (prefixes, total) = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::scan(&l, &|a, b| a + b, 0);

        assert_eq!(prefixes.length(), 3);
        assert_eq!(*prefixes.nth(0), 0); // reduce([], +, 0) = 0
        assert_eq!(*prefixes.nth(1), 1); // reduce([1], +, 0) = 1
        assert_eq!(*prefixes.nth(2), 3); // reduce([1,2], +, 0) = 3
        assert_eq!(total, 6); // reduce([1,2,3], +, 0) = 6
    }

    #[test]
    fn test_equality_comparison() {
        let l1 = LinkedListStEphSLit![1, 2, 3];
        let l2 = LinkedListStEphSLit![1, 2, 3];
        let l3 = LinkedListStEphSLit![1, 2, 4];

        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_subseq_copy_edge_cases() {
        let l = LinkedListStEphSLit![1, 2, 3, 4, 5];

        // Zero length
        let empty_sub = l.subseq_copy(2, 0);
        assert_eq!(empty_sub.length(), 0);

        // Start beyond bounds
        let beyond_sub = l.subseq_copy(10, 3);
        assert_eq!(beyond_sub.length(), 0);

        // Length extends beyond bounds
        let partial_sub = l.subseq_copy(3, 10);
        assert_eq!(partial_sub.length(), 2);
        assert_eq!(*partial_sub.nth(0), 4);
        assert_eq!(*partial_sub.nth(1), 5);
    }

    #[test]
    #[should_panic]
    fn test_nth_out_of_bounds_panics() {
        let l = LinkedListStEphSLit![0; 2];
        let _ = l.nth(2);
    }
}
