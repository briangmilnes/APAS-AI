pub mod TestLinkedListPer {
    use apas_ai::LinkedListStPer;
    use apas_ai::LinkedListStPer::LinkedListStPer::*;
    use apas_ai::LinkedListStPerSLit;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_empty_singleton_and_predicates() {
        let l: LinkedListStPerS<N> = LinkedListStPerSLit![];
        assert_eq!(l.length(), 0);
        assert_eq!(l.isEmpty(), B::True);
        let one = LinkedListStPerSLit![7];
        assert_eq!(one.isSingleton(), B::True);
    }

    #[test]
    fn test_new_and_nth_set() {
        let l = LinkedListStPerSLit![1; 3];
        assert_eq!(*l.nth(0), 1);
        assert_eq!(*l.nth(2), 1);
        let l2 = l.set(1, 9).unwrap();
        // original remains unchanged (persistent semantics)
        assert_eq!(*l.nth(1), 1);
        // updated copy has the change
        assert_eq!(*l2.nth(1), 9);
    }

    #[test]
    fn test_subseq_copy() {
        let l = LinkedListStPerSLit![2; 5];
        let sub = l.subseq_copy(1, 3);
        assert_eq!(sub.length(), 3);
        assert_eq!(*sub.nth(0), 2);
        assert_eq!(*sub.nth(2), 2);
    }

    #[test]
    fn test_from_vec_and_debug_format() {
        let l = LinkedListStPerSLit![1, 2, 3];
        assert_eq!(l.length(), 3);
        assert_eq!(format!("{:?}", l), "[1, 2, 3]");
    }

    #[test]
    fn test_iter_inorder_collect() {
        let l = LinkedListStPerSLit![10, 20, 30];
        let vals: Vec<N> = l.iter().copied().collect();
        assert_eq!(vals, vec![10, 20, 30]);
    }

    #[test]
    #[should_panic]
    fn test_nth_out_of_bounds_panics() {
        let l = LinkedListStPerSLit![0; 2];
        let _ = l.nth(2);
    }

    #[test]
    fn test_display_impl() {
        let l = LinkedListStPerSLit![1, 2, 3];
        assert_eq!(format!("{}", l), "[1, 2, 3]");
    }
}
