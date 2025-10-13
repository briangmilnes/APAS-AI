//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::BSTBBAlphaStEphLit;
use apas_ai::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::*;
use apas_ai::*;

#[test]
fn test_bstbbalphastephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn bbalpha_insert_find_balances() {
    let mut bst = BSTreeBBAlpha::new();
    for value in 0..64 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 64);
    let height = bst.height();
    assert!(height <= 12, "height {} too large", height);
    assert_eq!(bst.find(&10), Some(&10));
    assert_eq!(bst.find(&128), None);
    assert_eq!(bst.minimum().copied(), Some(0));
    assert_eq!(bst.maximum().copied(), Some(63));
    let inorder = bst.in_order();
    assert_eq!(inorder.length(), 64);
    for (expected, value) in (0..64).zip(inorder.iter()) {
        assert_eq!(*value, expected);
    }
}

#[test]
fn bbalpha_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}

#[test]
fn test_empty() {
    let bst: BSTreeBBAlpha<i32> = BSTreeBBAlpha::new();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
}

#[test]
fn test_singleton() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(42);
    assert_eq!(bst.size(), 1);
    assert!(bst.contains(&42));
}

#[test]
fn test_large_tree() {
    let mut bst = BSTreeBBAlpha::new();
    for i in 0..100 {
        bst.insert(i);
    }
    assert_eq!(bst.size(), 100);
}

#[test]
fn test_reverse_insert() {
    let mut bst = BSTreeBBAlpha::new();
    for i in (0..50).rev() {
        bst.insert(i);
    }
    assert_eq!(bst.size(), 50);
    assert_eq!(bst.minimum().copied(), Some(0));
}

#[test]
fn test_contains_empty() {
    let bst: BSTreeBBAlpha<i32> = BSTreeBBAlpha::new();
    assert!(!bst.contains(&5));
}

#[test]
fn test_negative_numbers() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(-5);
    bst.insert(-3);
    bst.insert(-7);
    assert!(bst.contains(&-5));
    assert_eq!(bst.minimum().copied(), Some(-7));
}

#[test]
fn test_find() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    assert_eq!(bst.find(&5), Some(&5));
    assert_eq!(bst.find(&10), None);
}

#[test]
fn test_maximum() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(9);
    assert_eq!(bst.maximum().copied(), Some(9));
}
