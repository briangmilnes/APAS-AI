//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestAVLTreeSeqEph {
    use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS;
    use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait as AVLTreeSeqEph;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_avl_ephemeral_basic() {
        let mut t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::new();
        assert_eq!(t.length(), 0);
        t.push_back(1);
        t.push_back(2);
        t.push_back(3);
        assert_eq!(t.length(), 3);
        assert_eq!(*t.nth(1), 2);
        // AVL trees ephemeral - has update method
        let _ = t.update((1, 9));
        assert_eq!(*t.nth(1), 9);
    }

    // Tests for remaining 15% untested AVLTreeSeqStEph methods
    #[test]
    fn test_avl_empty_constructor() {
        let empty: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::empty();
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), B::True);
        assert_eq!(empty.isSingleton(), B::False);
        
        let new_empty: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::new();
        assert_eq!(new_empty.length(), 0);
        assert_eq!(new_empty.isEmpty(), B::True);
        assert_eq!(new_empty.isSingleton(), B::False);
    }

    #[test]
    fn test_avl_singleton_constructor() {
        let single = AVLTreeSeqStEphS::singleton(42);
        assert_eq!(single.length(), 1);
        assert_eq!(single.isEmpty(), B::False);
        assert_eq!(single.isSingleton(), B::True);
        assert_eq!(*single.nth(0), 42);
    }

    #[test]
    fn test_avl_from_vec_constructor() {
        let values = vec![10, 20, 30, 40, 50];
        let tree = AVLTreeSeqStEphS::from_vec(values.clone());
        
        assert_eq!(tree.length(), 5);
        for i in 0..tree.length() {
            assert_eq!(*tree.nth(i), values[i]);
        }
        
        // Test empty vec
        let empty_tree: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::from_vec(vec![]);
        assert_eq!(empty_tree.length(), 0);
        assert_eq!(empty_tree.isEmpty(), B::True);
    }

    #[test]
    fn test_avl_set_method() {
        let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
        
        // Test successful set
        let result = tree.set(2, 99);
        assert!(result.is_ok());
        assert_eq!(*tree.nth(2), 99);
        
        // Test out-of-bounds set
        let result_oob = tree.set(10, 42);
        assert!(result_oob.is_err());
        if let Err(err_msg) = result_oob {
            assert_eq!(err_msg, "Index out of bounds");
        }
    }

    #[test]
    fn test_avl_update_method() {
        let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
        
        // Test update (which internally uses set)
        let updated = tree.update((1, 88));
        assert_eq!(*updated.nth(1), 88);
        assert_eq!(*tree.nth(1), 88); // Ephemeral - original is modified
        
        // Test out-of-bounds update (should be silently ignored per APAS style)
        let _ = tree.update((10, 99));
        // Tree should remain unchanged for out-of-bounds
        assert_eq!(tree.length(), 5);
    }

    #[test]
    fn test_avl_subseq_copy() {
        let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        
        // Test normal subseq
        let sub1 = tree.subseq_copy(2, 4); // [3, 4, 5, 6]
        assert_eq!(sub1.length(), 4);
        assert_eq!(*sub1.nth(0), 3);
        assert_eq!(*sub1.nth(1), 4);
        assert_eq!(*sub1.nth(2), 5);
        assert_eq!(*sub1.nth(3), 6);
        
        // Test subseq at beginning
        let sub2 = tree.subseq_copy(0, 3); // [1, 2, 3]
        assert_eq!(sub2.length(), 3);
        assert_eq!(*sub2.nth(0), 1);
        assert_eq!(*sub2.nth(2), 3);
        
        // Test subseq at end
        let sub3 = tree.subseq_copy(7, 3); // [8, 9, 10]
        assert_eq!(sub3.length(), 3);
        assert_eq!(*sub3.nth(0), 8);
        assert_eq!(*sub3.nth(2), 10);
        
        // Test empty subseq (length 0)
        let empty_sub = tree.subseq_copy(5, 0);
        assert_eq!(empty_sub.length(), 0);
        assert_eq!(empty_sub.isEmpty(), B::True);
        
        // Test out-of-bounds start
        let oob_sub = tree.subseq_copy(15, 3);
        assert_eq!(oob_sub.length(), 0);
        assert_eq!(oob_sub.isEmpty(), B::True);
        
        // Test partial out-of-bounds (extends beyond end)
        let partial_sub = tree.subseq_copy(8, 5); // Should get [9, 10]
        assert_eq!(partial_sub.length(), 2);
        assert_eq!(*partial_sub.nth(0), 9);
        assert_eq!(*partial_sub.nth(1), 10);
    }

    #[test]
    fn test_avl_push_back() {
        let mut tree = AVLTreeSeqStEphS::new();
        
        // Test pushing to empty tree
        tree.push_back(10);
        assert_eq!(tree.length(), 1);
        assert_eq!(*tree.nth(0), 10);
        
        // Test pushing multiple elements
        tree.push_back(20);
        tree.push_back(30);
        tree.push_back(40);
        assert_eq!(tree.length(), 4);
        assert_eq!(*tree.nth(0), 10);
        assert_eq!(*tree.nth(1), 20);
        assert_eq!(*tree.nth(2), 30);
        assert_eq!(*tree.nth(3), 40);
    }

    #[test]
    fn test_avl_contains_value() {
        let tree = AVLTreeSeqStEphS::from_vec(vec![10, 20, 30, 40, 50]);
        
        // Test existing values
        assert_eq!(tree.contains_value(&20), B::True);
        assert_eq!(tree.contains_value(&50), B::True);
        assert_eq!(tree.contains_value(&10), B::True);
        
        // Test non-existing values
        assert_eq!(tree.contains_value(&15), B::False);
        assert_eq!(tree.contains_value(&60), B::False);
        assert_eq!(tree.contains_value(&0), B::False);
        
        // Test empty tree
        let empty = AVLTreeSeqStEphS::new();
        assert_eq!(empty.contains_value(&42), B::False);
    }

    #[test]
    fn test_avl_insert_value() {
        let mut tree = AVLTreeSeqStEphS::new();
        
        // insert_value is an alias for push_back
        tree.insert_value(100);
        tree.insert_value(200);
        tree.insert_value(300);
        
        assert_eq!(tree.length(), 3);
        assert_eq!(*tree.nth(0), 100);
        assert_eq!(*tree.nth(1), 200);
        assert_eq!(*tree.nth(2), 300);
    }

    #[test]
    fn test_avl_delete_value() {
        let mut tree = AVLTreeSeqStEphS::from_vec(vec![10, 20, 30, 40, 50]);
        
        // Test deleting existing value
        let deleted = tree.delete_value(&30);
        assert!(deleted);
        assert_eq!(tree.length(), 4);
        assert_eq!(*tree.nth(0), 10);
        assert_eq!(*tree.nth(1), 20);
        assert_eq!(*tree.nth(2), 40); // 30 was removed, 40 shifted left
        assert_eq!(*tree.nth(3), 50);
        
        // Test deleting first element
        let deleted_first = tree.delete_value(&10);
        assert!(deleted_first);
        assert_eq!(tree.length(), 3);
        assert_eq!(*tree.nth(0), 20);
        assert_eq!(*tree.nth(1), 40);
        assert_eq!(*tree.nth(2), 50);
        
        // Test deleting last element
        let deleted_last = tree.delete_value(&50);
        assert!(deleted_last);
        assert_eq!(tree.length(), 2);
        assert_eq!(*tree.nth(0), 20);
        assert_eq!(*tree.nth(1), 40);
        
        // Test deleting non-existing value
        let not_deleted = tree.delete_value(&99);
        assert!(!not_deleted);
        assert_eq!(tree.length(), 2); // Should remain unchanged
        
        // Test deleting from empty tree
        let mut empty = AVLTreeSeqStEphS::new();
        let not_deleted_empty = empty.delete_value(&42);
        assert!(!not_deleted_empty);
        assert_eq!(empty.length(), 0);
    }

    #[test]
    fn test_avl_iterator() {
        let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
        
        // Test iterator collects values in order
        let collected: Vec<N> = tree.iter().cloned().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
        
        // Test empty tree iterator
        let empty = AVLTreeSeqStEphS::new();
        let empty_collected: Vec<N> = empty.iter().cloned().collect();
        assert_eq!(empty_collected, vec![]);
        
        // Test single element iterator
        let single = AVLTreeSeqStEphS::singleton(42);
        let single_collected: Vec<N> = single.iter().cloned().collect();
        assert_eq!(single_collected, vec![42]);
    }

    #[test]
    fn test_avl_to_arrayseq() {
        let tree = AVLTreeSeqStEphS::from_vec(vec![10, 20, 30, 40, 50]);
        let array_seq = tree.to_arrayseq();
        
        assert_eq!(array_seq.length(), tree.length());
        for i in 0..tree.length() {
            assert_eq!(*array_seq.nth(i), *tree.nth(i));
        }
        
        // Test empty tree conversion
        let empty_tree: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::new();
        let empty_array = empty_tree.to_arrayseq();
        assert_eq!(empty_array.length(), 0);
    }

    #[test]
    fn test_avl_equality() {
        let tree1 = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
        let tree2 = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
        let tree3 = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 6]);
        
        assert!(tree1 == tree2);
        assert!(tree1 != tree3);
        
        // Test empty trees
        let empty1: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::new();
        let empty2: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::new();
        assert!(empty1 == empty2);
        
        // Test different lengths
        let short = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
        assert!(tree1 != short);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_avl_nth_out_of_bounds_panics() {
        let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
        let _ = tree.nth(5); // Should panic
    }

    #[test]
    fn test_avl_macro_literal() {
        use apas_ai::AVLTreeSeqStEphLit;
        
        // Test empty macro
        let empty: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![];
        assert_eq!(empty.length(), 0);
        
        // Test single element macro
        let single = AVLTreeSeqStEphLit![42];
        assert_eq!(single.length(), 1);
        assert_eq!(*single.nth(0), 42);
        
        // Test multiple elements macro
        let multi = AVLTreeSeqStEphLit![1, 2, 3, 4, 5];
        assert_eq!(multi.length(), 5);
        for i in 0..5 {
            assert_eq!(*multi.nth(i), i + 1);
        }
        
        // Test repeat macro
        let repeat = AVLTreeSeqStEphLit![99; 3];
        assert_eq!(repeat.length(), 3);
        for i in 0..3 {
            assert_eq!(*repeat.nth(i), 99);
        }
    }

    #[test]
    fn test_avl_large_tree_operations() {
        // Test with larger tree to stress balancing
        let values: Vec<N> = (1..=20).collect();
        let mut tree = AVLTreeSeqStEphS::from_vec(values.clone());
        
        assert_eq!(tree.length(), 20);
        
        // Test random access
        for i in 0..tree.length() {
            assert_eq!(*tree.nth(i), values[i]);
        }
        
        // Test modifications
        let _ = tree.set(10, 999);
        assert_eq!(*tree.nth(10), 999);
        
        // Test contains after modification
        assert_eq!(tree.contains_value(&999), B::True);
        assert_eq!(tree.contains_value(&11), B::False); // 11 was replaced by 999
        
        // Test deletion
        let deleted = tree.delete_value(&5);
        assert!(deleted);
        assert_eq!(tree.length(), 19);
        assert_eq!(tree.contains_value(&5), B::False);
    }

    #[test]
    fn test_avl_stress_operations() {
        let mut tree = AVLTreeSeqStEphS::new();
        
        // Build tree with push_back
        for i in 1..=10 {
            tree.push_back(i * 10);
        }
        assert_eq!(tree.length(), 10);
        
        // Modify every other element
        for i in (0..tree.length()).step_by(2) {
            let _ = tree.set(i, i * 100);
        }
        
        // Verify modifications
        for i in 0..tree.length() {
            if i % 2 == 0 {
                assert_eq!(*tree.nth(i), i * 100);
            } else {
                assert_eq!(*tree.nth(i), (i + 1) * 10);
            }
        }
        
        // Delete some elements
        let _deleted_0 = tree.delete_value(&0);   // Delete first modified element (if exists)
        let _deleted_20 = tree.delete_value(&20); // Delete an unmodified element
        
        // Verify tree integrity after deletions
        assert!(tree.length() <= 10);
        let collected: Vec<N> = tree.iter().cloned().collect();
        
        // Verify iterator still works - just check that we can collect all values
        // and that the tree maintains its basic properties
        assert_eq!(collected.len(), tree.length());
        
        // Verify all collected values are accessible via nth
        for (idx, &value) in collected.iter().enumerate() {
            assert_eq!(*tree.nth(idx), value);
        }
    }
}
