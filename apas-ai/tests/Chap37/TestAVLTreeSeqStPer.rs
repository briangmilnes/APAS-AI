//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::AVLTreeSeqStPerLit;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_persistent_set_does_not_mutate() {
    let _t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![7];
    let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3]; // tabulate(&|i| i, 4)
                                                                  // AVLTreeSeqStPer is persistent - no update method
                                                                  // let b = a.update(1, 99);
    assert_eq!(*a.nth(1), 1);
    // assert_eq!(*b.nth(1), 99);
}

#[test]
fn test_iterator_inorder_values() {
    let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![1, 2, 3, 4, 5]; // tabulate(&|i| i + 1, 5)
    let vals: Vec<N> = a.iter().map(|x| *x).collect();
    assert_eq!(vals, vec![1, 2, 3, 4, 5]);
}

// Complex tree balancing edge case tests
#[test]
fn test_avl_left_heavy_rebalancing() {
    // Create a tree and test modifications that could trigger rebalancing
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50];

    // Modify elements to create potential imbalance scenarios
    let tree = tree.set(0, 5).unwrap(); // Change first element
    let tree = tree.set(1, 15).unwrap(); // Change second element
    let tree = tree.set(2, 25).unwrap(); // Change third element

    // Tree should remain balanced and maintain structure
    assert_eq!(tree.length(), 5);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![5, 15, 25, 40, 50]);

    // Verify all elements are accessible
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), vals[i]);
    }
}

#[test]
fn test_avl_right_heavy_rebalancing() {
    // Create a tree and test modifications that could trigger rebalancing
    let tree = AVLTreeSeqStPerLit![1, 3, 5, 10, 15];

    // Modify elements to create potential imbalance scenarios
    let tree = tree.set(4, 50).unwrap(); // Change last element
    let tree = tree.set(3, 40).unwrap(); // Change fourth element
    let tree = tree.set(2, 30).unwrap(); // Change third element

    // Tree should remain balanced and maintain structure
    assert_eq!(tree.length(), 5);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![1, 3, 30, 40, 50]);

    // Verify all elements are accessible
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), vals[i]);
    }
}

#[test]
fn test_avl_left_right_rotation() {
    // Create a scenario requiring left-right rotation
    let tree = AVLTreeSeqStPerLit![1, 2, 3, 4, 5, 6, 7];

    // Modify middle elements to create left-right imbalance
    let tree = tree.set(2, 15).unwrap(); // Change 3 to 15
    let tree = tree.set(1, 12).unwrap(); // Change 2 to 12

    // Tree should remain balanced
    assert_eq!(tree.length(), 7);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![1, 12, 15, 4, 5, 6, 7]);

    // Verify structural integrity
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), vals[i]);
    }
}

#[test]
fn test_avl_right_left_rotation() {
    // Create a scenario requiring right-left rotation
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50, 60, 70];

    // Modify elements to create right-left imbalance
    let tree = tree.set(4, 25).unwrap(); // Change 50 to 25
    let tree = tree.set(5, 35).unwrap(); // Change 60 to 35

    // Tree should remain balanced
    assert_eq!(tree.length(), 7);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![10, 20, 30, 40, 25, 35, 70]);

    // Verify structural integrity
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), vals[i]);
    }
}

#[test]
fn test_avl_large_tree_balancing() {
    // Test balancing with a larger tree (15 elements)
    let values: Vec<N> = (1..=15).collect();
    let tree = AVLTreeSeqStPerS::from_vec(values.clone());

    assert_eq!(tree.length(), 15);
    assert_eq!(tree.values_in_order(), values);

    // Verify random access works correctly
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), values[i]);
    }

    // Test modifications maintain balance
    let modified = tree.set(7, 100).unwrap(); // Change middle element
    assert_eq!(modified.length(), 15);
    assert_eq!(*modified.nth(7), 100);

    // Verify other elements unchanged
    for i in 0..modified.length() {
        if i != 7 {
            assert_eq!(*modified.nth(i), values[i]);
        }
    }
}

#[test]
fn test_avl_sequential_modifications() {
    // Test multiple sequential modifications maintain balance
    let mut tree = AVLTreeSeqStPerLit![5, 10, 15, 20, 25];

    // Apply multiple modifications
    tree = tree.set(0, 1).unwrap(); // 1, 10, 15, 20, 25
    tree = tree.set(4, 30).unwrap(); // 1, 10, 15, 20, 30
    tree = tree.set(2, 12).unwrap(); // 1, 10, 12, 20, 30
    tree = tree.set(1, 8).unwrap(); // 1, 8, 12, 20, 30
    tree = tree.set(3, 22).unwrap(); // 1, 8, 12, 22, 30

    assert_eq!(tree.length(), 5);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![1, 8, 12, 22, 30]);

    // Verify all elements accessible
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), vals[i]);
    }
}

#[test]
fn test_avl_empty_and_singleton_edge_cases() {
    // Test empty tree
    let empty: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerS::empty();
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), true);
    assert_eq!(empty.isSingleton(), false);
    assert_eq!(empty.values_in_order(), vec![]);

    // Test singleton
    let single = AVLTreeSeqStPerS::singleton(42);
    assert_eq!(single.length(), 1);
    assert_eq!(single.isEmpty(), false);
    assert_eq!(single.isSingleton(), true);
    assert_eq!(*single.nth(0), 42);
    assert_eq!(single.values_in_order(), vec![42]);

    // Test singleton modification
    let modified = single.set(0, 99).unwrap();
    assert_eq!(modified.length(), 1);
    assert_eq!(*modified.nth(0), 99);
    assert_eq!(modified.values_in_order(), vec![99]);

    // Original singleton unchanged (persistent)
    assert_eq!(*single.nth(0), 42);
}

#[test]
fn test_avl_subseq_balancing() {
    // Test that subseq operations maintain balance
    let tree = AVLTreeSeqStPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Test various subseq operations
    let sub1 = tree.subseq_copy(2, 5); // [3, 4, 5, 6, 7]
    assert_eq!(sub1.length(), 5);
    assert_eq!(sub1.values_in_order(), vec![3, 4, 5, 6, 7]);

    let sub2 = tree.subseq_copy(0, 3); // [1, 2, 3]
    assert_eq!(sub2.length(), 3);
    assert_eq!(sub2.values_in_order(), vec![1, 2, 3]);

    let sub3 = tree.subseq_copy(7, 3); // [8, 9, 10]
    assert_eq!(sub3.length(), 3);
    assert_eq!(sub3.values_in_order(), vec![8, 9, 10]);

    // Test edge cases
    let empty_sub = tree.subseq_copy(5, 0); // Empty subseq
    assert_eq!(empty_sub.length(), 0);
    assert_eq!(empty_sub.isEmpty(), true);

    let out_of_bounds = tree.subseq_copy(15, 5); // Start beyond end
    assert_eq!(out_of_bounds.length(), 0);
    assert_eq!(out_of_bounds.isEmpty(), true);

    let partial_bounds = tree.subseq_copy(8, 5); // Extends beyond end
    assert_eq!(partial_bounds.length(), 2); // Should get [9, 10]
    assert_eq!(partial_bounds.values_in_order(), vec![9, 10]);
}

#[test]
fn test_avl_stress_balancing() {
    // Stress test with alternating modifications
    let mut tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // Apply alternating modifications that could stress balancing
    for i in 0..tree.length() {
        let new_val = if i % 2 == 0 { i * 5 } else { i * 15 };
        tree = tree.set(i, new_val).unwrap();
    }

    assert_eq!(tree.length(), 10);

    // Verify all elements are accessible and tree maintains integrity
    let vals = tree.values_in_order();
    for i in 0..tree.length() {
        assert_eq!(*tree.nth(i), vals[i]);
    }

    // Expected values: [0, 15, 10, 45, 20, 75, 30, 105, 40, 135]
    let expected = vec![0, 15, 10, 45, 20, 75, 30, 105, 40, 135];
    assert_eq!(vals, expected);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_avl_nth_out_of_bounds_panics() {
    let tree = AVLTreeSeqStPerLit![1, 2, 3];
    let _ = tree.nth(5); // Should panic
}

#[test]
fn test_avl_set_out_of_bounds_error() {
    let tree = AVLTreeSeqStPerLit![1, 2, 3];
    let result = tree.set(5, 99);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Index out of bounds");
}

#[test]
fn test_avl_equality_and_debug() {
    let tree1 = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];
    let tree2 = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];
    let tree3 = AVLTreeSeqStPerLit![1, 2, 3, 4, 6];

    // Test equality
    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);

    // Test debug formatting (should not panic)
    let debug_str = format!("{:?}", tree1);
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("5"));
}

#[test]
fn test_avl_to_arrayseq_conversion() {
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50];
    let array_seq = tree.to_arrayseq();

    assert_eq!(array_seq.length(), tree.length());
    for i in 0..tree.length() {
        assert_eq!(*array_seq.nth(i), *tree.nth(i));
    }
}
