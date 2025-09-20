/// Ephemeral singly linked list tests (LinkedListEph).
pub mod TestLinkedListEph {

    use apas_ai::LinkedListStEph;
    use apas_ai::LinkedListStEph::LinkedListStEph::*;
    use apas_ai::Types::Types::*; // macro import
    use apas_ai::LinkedListStEphSLit;

    #[test]
    fn test_empty_singleton_and_predicates() {
        let l: LinkedListStEphS<N> = LinkedListStEphSLit![];
        assert_eq!(l.length(), 0);
        assert_eq!(l.isEmpty(), B::True);
        let one = LinkedListStEphSLit![7];
        assert_eq!(one.isSingleton(), B::True);
    }

    #[test]
    fn test_new_and_nth_set() {
        let mut l = LinkedListStEphSLit![1; 3];
        assert_eq!(*l.nth(0), 1);
        assert_eq!(*l.nth(2), 1);
        let _ = l.set(1, 9).unwrap();
        assert_eq!(*l.nth(1), 9);
    }

    #[test]
    fn test_subseq_copy() {
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
        let _ = s.set(1, 9).unwrap();
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
        let mut it = l.iter();
        assert_eq!(it.next().copied(), Some(5));
        assert_eq!(it.next().copied(), Some(6));
        assert_eq!(it.next().copied(), Some(7));
        assert_eq!(it.next(), None);
    }
}
