//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::*;
use apas_ai::*;

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
