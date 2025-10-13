//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTTreapMtEph.

use apas_ai::BSTTreapMtEphLit;
use apas_ai::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bsttreapmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTTreapMtEph<i32> = BSTTreapMtEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTTreapMtEph<i32> = BSTTreapMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

// Additional dedicated tests for BSTTreapMtEphLit macro

#[test]
fn test_bsttreapmtephlit_macro_empty_tree() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![];

    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);
    assert!(!tree.contains(&0));
    assert_eq!(tree.find(&0), None);
}

#[test]
fn test_bsttreapmtephlit_macro_single_element() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![42];

    assert_eq!(tree.size(), 1);
    assert!(!tree.is_empty());
    assert_eq!(tree.height(), 1);
    assert_eq!(tree.minimum(), Some(42));
    assert_eq!(tree.maximum(), Some(42));
    assert!(tree.contains(&42));
    assert_eq!(tree.find(&42), Some(42));
    assert!(!tree.contains(&99));
}

#[test]
fn test_bsttreapmtephlit_macro_multiple_elements() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![10, 5, 15, 3, 7, 12, 20];

    assert_eq!(tree.size(), 7);
    assert!(!tree.is_empty());

    // Check all elements are present
    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(tree.contains(&12));
    assert!(tree.contains(&20));

    // Check elements not in tree
    assert!(!tree.contains(&1));
    assert!(!tree.contains(&25));

    // Check min and max
    assert_eq!(tree.minimum(), Some(3));
    assert_eq!(tree.maximum(), Some(20));
}

#[test]
fn test_bsttreapmtephlit_macro_with_trailing_comma() {
    let tree1: BSTTreapMtEph<i32> = BSTTreapMtEphLit![1, 2, 3,];
    let tree2: BSTTreapMtEph<i32> = BSTTreapMtEphLit![1, 2, 3];

    assert_eq!(tree1.size(), tree2.size());
    assert_eq!(tree1.size(), 3);
}

#[test]
fn test_bsttreapmtephlit_macro_with_duplicates() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![5, 5, 3, 7, 3, 5];

    // Depending on BST implementation, duplicates might be ignored or handled
    // At minimum, the tree should contain the unique values
    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
}

#[test]
fn test_bsttreapmtephlit_macro_sorted_order() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![5, 4, 3, 2, 1];

    // Even with descending insertion, tree should work correctly
    assert_eq!(tree.size(), 5);
    assert_eq!(tree.minimum(), Some(1));
    assert_eq!(tree.maximum(), Some(5));

    // Check in-order traversal is sorted
    let in_order = tree.in_order();
    assert_eq!(in_order.length(), 5);
    for i in 0..4 {
        assert!(*in_order.nth(i) < *in_order.nth(i + 1));
    }
}

#[test]
fn test_bsttreapmtephlit_macro_negative_numbers() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![-5, -10, 0, 5, -3];

    assert_eq!(tree.size(), 5);
    assert_eq!(tree.minimum(), Some(-10));
    assert_eq!(tree.maximum(), Some(5));
    assert!(tree.contains(&-5));
    assert!(tree.contains(&0));
    assert!(tree.contains(&5));
}
