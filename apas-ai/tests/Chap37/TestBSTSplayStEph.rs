//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::BSTSplayStEphLit;
use apas_ai::Chap37::BSTSplayStEph::BSTSplayStEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsplaystephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSplayStEph<i32> = BSTSplayStEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTSplayStEph<i32> = BSTSplayStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn splay_basic_behaviour() {
    let mut bst = BSTreeSplay::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.contains(&3), true);
    assert_eq!(bst.find(&9), None);
    assert_eq!(bst.minimum(), Some(&1));
    assert_eq!(bst.maximum(), Some(&7));
    let inorder = bst.in_order();
    let expected = [1, 2, 3, 4, 5, 6, 7];
    assert_eq!(inorder.length(), expected.len());
    for (exp, value) in expected.iter().zip(inorder.iter()) {
        assert_eq!(*value, *exp);
    }
}

#[test]
fn splay_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeSplay::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}
