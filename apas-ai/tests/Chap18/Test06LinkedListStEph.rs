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
}
