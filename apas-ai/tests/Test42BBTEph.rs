//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap23::BBTEph::BBTEph::*;
use apas_ai::Chap37::BSTPlainStEph::BSTPlainStEph::*;
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Types::Types::*;

#[test]
fn inorder_and_preorder_traversals_match_definitions() {
    let tree = BBTree::node(
        BBTree::node(BBTree::leaf(), 2, BBTree::leaf()),
        4,
        BBTree::node(BBTree::leaf(), 6, BBTree::leaf()),
    );
    let inorder = tree.in_order();
    let preorder = tree.pre_order();
    assert_eq!(inorder, ArraySeqStPerSLit![2, 4, 6]);
    assert_eq!(preorder, ArraySeqStPerSLit![4, 2, 6]);
    assert_eq!(tree.size(), 3);
    assert_eq!(tree.height(), 2);
}

#[test]
fn bst_insert_and_search_behavior() {
    let mut bst = BSTree::new();
    let keys = ArraySeqStPerSLit![4, 2, 6, 1, 3, 5, 7];
    for index in 0..keys.length() {
        bst.insert(keys.nth(index).clone());
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.height(), 3);
    assert_eq!(bst.contains(&4), B::True);
    assert_eq!(bst.contains(&5), B::True);
    assert_eq!(bst.contains(&8), B::False);
    assert_eq!(bst.find(&4), Some(&4));
    assert_eq!(bst.find(&6), Some(&6));
    assert_eq!(bst.find(&9), None);
    let inorder = bst.in_order();
    assert_eq!(inorder, ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7]);
    let preorder = bst.pre_order();
    assert_eq!(preorder, ArraySeqStPerSLit![4, 2, 1, 3, 6, 5, 7]);
    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(7));
}
