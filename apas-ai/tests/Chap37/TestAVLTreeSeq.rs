//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeq base trait functionality.

#[cfg(test)]
mod TestAVLTreeSeq {
    use apas_ai::Chap37::AVLTreeSeq::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
    use apas_ai::Types::Types::*;

    #[test]
    fn test_avltreeseq_empty_constructor() {
        let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), true);
        assert_eq!(empty.isSingleton(), false);
    }

    #[test]
    fn test_avltreeseq_new_constructor() {
        let new_tree: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::new();
        assert_eq!(new_tree.length(), 0);
        assert_eq!(new_tree.isEmpty(), true);
        assert_eq!(new_tree.isSingleton(), false);
    }

    #[test]
    fn test_avltreeseq_singleton_constructor() {
        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        assert_eq!(single.length(), 1);
        assert_eq!(single.isEmpty(), false);
        assert_eq!(single.isSingleton(), true);
        assert_eq!(*single.nth(0), 42);
    }

    #[test]
    fn test_avltreeseq_length_method() {
        let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        assert_eq!(empty.length(), 0);

        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(10);
        assert_eq!(single.length(), 1);
    }

    #[test]
    fn test_avltreeseq_nth_method() {
        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(99);
        assert_eq!(*single.nth(0), 99);
    }

    #[test]
    #[should_panic]
    fn test_avltreeseq_nth_panic_outofbounds() {
        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        let _ = single.nth(1); // Index 1 is out of bounds for single element
    }

    #[test]
    #[should_panic]
    fn test_avltreeseq_nth_panic_empty() {
        let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        let _ = empty.nth(0); // Any index on empty tree should panic
    }

    #[test]
    fn test_avltreeseq_set_method() {
        let mut single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        let result = single.set(0, 99);
        assert!(result.is_ok());
        assert_eq!(*single.nth(0), 99);
    }

    #[test]
    fn test_avltreeseq_set_out_of_bounds_error() {
        let mut single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        let result = single.set(1, 99);
        assert!(result.is_err());
        if let Err(err_msg) = result {
            assert_eq!(err_msg, "Index out of bounds");
        }
    }

    #[test]
    fn test_avltreeseq_predicates() {
        // Test isEmpty
        let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        assert_eq!(empty.isEmpty(), true);
        assert_eq!(empty.isSingleton(), false);

        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        assert_eq!(single.isEmpty(), false);
        assert_eq!(single.isSingleton(), true);
    }

    #[test]
    fn test_avltreeseq_subseq_copy() {
        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);

        // Full subseq
        let full_subseq = single.subseq_copy(0, 1);
        assert_eq!(full_subseq.length(), 1);
        assert_eq!(*full_subseq.nth(0), 42);

        // Empty subseq
        let empty_subseq = single.subseq_copy(0, 0);
        assert_eq!(empty_subseq.length(), 0);
        assert_eq!(empty_subseq.isEmpty(), true);

        // Out of bounds subseq
        let oob_subseq = single.subseq_copy(1, 1);
        assert_eq!(oob_subseq.length(), 0);
        assert_eq!(oob_subseq.isEmpty(), true);
    }

    #[test]
    fn test_avltreeseq_empty_operations_comprehensive() {
        let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();

        // Basic properties
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), true);
        assert_eq!(empty.isSingleton(), false);

        // Subseq operations on empty tree
        let empty_subseq = empty.subseq_copy(0, 0);
        assert_eq!(empty_subseq.length(), 0);
        assert_eq!(empty_subseq.isEmpty(), true);

        let empty_subseq2 = empty.subseq_copy(0, 10);
        assert_eq!(empty_subseq2.length(), 0);
        assert_eq!(empty_subseq2.isEmpty(), true);
    }

    #[test]
    fn test_avltreeseq_single_element_boundary() {
        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);

        // Basic properties
        assert_eq!(single.length(), 1);
        assert_eq!(single.isEmpty(), false);
        assert_eq!(single.isSingleton(), true);

        // Access operations
        assert_eq!(*single.nth(0), 42);

        // Subseq operations
        let full_subseq = single.subseq_copy(0, 1);
        assert_eq!(full_subseq.length(), 1);
        assert_eq!(*full_subseq.nth(0), 42);

        let empty_subseq = single.subseq_copy(1, 1);
        assert_eq!(empty_subseq.length(), 0);

        let zero_length_subseq = single.subseq_copy(0, 0);
        assert_eq!(zero_length_subseq.length(), 0);

        // Set operations
        let mut single_mut = single;
        let result = single_mut.set(0, 99);
        assert!(result.is_ok());
        assert_eq!(*single_mut.nth(0), 99);

        // Out of bounds set should return error
        let result_oob = single_mut.set(1, 100);
        assert!(result_oob.is_err());
    }

    #[test]
    fn test_avltreeseq_zero_length_operations() {
        // Test zero-length subseq operations
        let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);

        // Zero-length subseq at start
        let zero_start = single.subseq_copy(0, 0);
        assert_eq!(zero_start.length(), 0);
        assert_eq!(zero_start.isEmpty(), true);

        // Zero-length subseq at end
        let zero_end = single.subseq_copy(1, 0);
        assert_eq!(zero_end.length(), 0);
        assert_eq!(zero_end.isEmpty(), true);

        // Zero-length subseq beyond end should still return empty
        let zero_beyond = single.subseq_copy(10, 0);
        assert_eq!(zero_beyond.length(), 0);
        assert_eq!(zero_beyond.isEmpty(), true);

        // Test with empty tree
        let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        let zero_empty = empty.subseq_copy(0, 0);
        assert_eq!(zero_empty.length(), 0);
        assert_eq!(zero_empty.isEmpty(), true);

        // All zero-length subsequences should be equivalent to empty
        assert_eq!(zero_start.length(), empty.length());
        assert_eq!(zero_end.length(), empty.length());
        assert_eq!(zero_beyond.length(), empty.length());
        assert_eq!(zero_empty.length(), empty.length());
    }

    #[test]
    fn test_avltreeseq_equality_comparison() {
        let tree1: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        let tree2: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
        let tree3: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(43);

        // Note: AVLTreeS may not implement PartialEq, so we compare properties
        assert_eq!(tree1.length(), tree2.length());
        assert_eq!(*tree1.nth(0), *tree2.nth(0));

        assert_eq!(tree1.length(), tree3.length());
        assert_ne!(*tree1.nth(0), *tree3.nth(0));

        let empty1: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        let empty2: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
        assert_eq!(empty1.length(), empty2.length());
        assert_eq!(empty1.isEmpty(), empty2.isEmpty());
    }
}
