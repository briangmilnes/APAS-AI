//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTTreapMtEph.

use apas_ai::BSTTreapMtEphLit;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
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

#[test]
fn test_insert_and_find() {
    let tree = BSTTreapMtEph::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());

    tree.insert(5);
    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
    assert_eq!(tree.find(&5), Some(5));

    tree.insert(3);
    tree.insert(7);
    assert_eq!(tree.size(), 3);
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
}

#[test]
fn test_clone() {
    let tree1 = BSTTreapMtEphLit![1, 2, 3, 4, 5];
    let tree2 = tree1.clone();

    assert_eq!(tree1.size(), tree2.size());
    for i in 1..=5 {
        assert_eq!(tree1.contains(&i), tree2.contains(&i));
    }
}

#[test]
fn test_pre_order() {
    let tree = BSTTreapMtEphLit![5, 3, 7, 1, 4, 6, 9];
    let pre = tree.pre_order();
    assert_eq!(pre.length(), 7);

    // Pre-order should visit root first (though exact order depends on treap priorities)
    // Just verify all elements are present
    let mut found = vec![false; 10];
    for i in 0..pre.length() {
        let val = *pre.nth(i) as usize;
        if val < 10 {
            found[val] = true;
        }
    }
    assert!(found[1] && found[3] && found[4] && found[5] && found[6] && found[7] && found[9]);
}

#[test]
fn test_in_order_sorted() {
    let tree = BSTTreapMtEphLit![9, 3, 5, 1, 7, 2, 8, 4, 6];
    let in_order = tree.in_order();
    assert_eq!(in_order.length(), 9);

    // In-order traversal should be sorted
    for i in 0..8 {
        assert!(*in_order.nth(i) < *in_order.nth(i + 1));
    }
}

#[test]
fn test_find_empty_tree() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(tree.find(&42), None);
}

#[test]
fn test_find_all_elements() {
    let tree = BSTTreapMtEphLit![10, 20, 30, 40, 50];
    for val in [10, 20, 30, 40, 50] {
        assert_eq!(tree.find(&val), Some(val));
    }
}

#[test]
fn test_minimum_maximum_empty() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);
}

#[test]
fn test_minimum_maximum_single() {
    let tree = BSTTreapMtEphLit![42];
    assert_eq!(tree.minimum(), Some(42));
    assert_eq!(tree.maximum(), Some(42));
}

#[test]
fn test_minimum_maximum_large() {
    let tree = BSTTreapMtEphLit![50, 25, 75, 10, 30, 60, 80, 5, 15, 27, 35];
    assert_eq!(tree.minimum(), Some(5));
    assert_eq!(tree.maximum(), Some(80));
}

#[test]
fn test_height() {
    let empty: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(empty.height(), 0);

    let single = BSTTreapMtEphLit![42];
    assert_eq!(single.height(), 1);

    let tree = BSTTreapMtEphLit![1, 2, 3, 4, 5, 6, 7];
    let h = tree.height();
    assert!(h >= 3 && h <= 7); // Treap balancing is probabilistic
}

#[test]
fn test_large_tree() {
    let tree = BSTTreapMtEph::new();
    for i in 0..100 {
        tree.insert(i);
    }

    assert_eq!(tree.size(), 100);
    for i in 0..100 {
        assert!(tree.contains(&i));
    }

    assert_eq!(tree.minimum(), Some(0));
    assert_eq!(tree.maximum(), Some(99));
}

#[test]
fn test_insert_duplicate() {
    let tree = BSTTreapMtEph::new();
    tree.insert(42);
    tree.insert(42);
    tree.insert(42);

    // Treap should handle duplicates (either ignore or replace)
    assert!(tree.contains(&42));
    assert!(tree.size() >= 1 && tree.size() <= 3);
}

#[test]
fn test_mixed_operations() {
    let tree = BSTTreapMtEphLit![5, 3, 7, 1, 9];

    // Insert
    tree.insert(6);
    assert_eq!(tree.size(), 6);
    assert!(tree.contains(&6));

    // Find
    assert_eq!(tree.find(&7), Some(7));
    assert_eq!(tree.find(&99), None);
}

#[test]
fn test_string_tree() {
    let tree = BSTTreapMtEph::new();
    tree.insert("dog".to_string());
    tree.insert("cat".to_string());
    tree.insert("elephant".to_string());
    tree.insert("ant".to_string());

    assert_eq!(tree.size(), 4);
    assert!(tree.contains(&"cat".to_string()));
    assert!(!tree.contains(&"zebra".to_string()));

    assert_eq!(tree.minimum(), Some("ant".to_string()));
    assert_eq!(tree.maximum(), Some("elephant".to_string()));
}

#[test]
fn test_in_order_empty() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    let in_order = tree.in_order();
    assert_eq!(in_order.length(), 0);
}

#[test]
fn test_pre_order_empty() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    let pre_order = tree.pre_order();
    assert_eq!(pre_order.length(), 0);
}

#[test]
fn test_debug_trait() {
    let tree = BSTTreapMtEphLit![10, 20, 30];
    let debug_str = format!("{:?}", tree);
    // Just verify it doesn't panic
    assert!(!debug_str.is_empty());
}
