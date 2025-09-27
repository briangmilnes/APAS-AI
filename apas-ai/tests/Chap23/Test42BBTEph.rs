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

#[test]
fn bbtree_empty_leaf_operations() {
    let leaf = BBTree::<N>::leaf();
    assert_eq!(leaf.size(), 0);
    assert_eq!(leaf.height(), 0);
    assert_eq!(leaf.in_order().length(), 0);
    assert_eq!(leaf.pre_order().length(), 0);
}

#[test]
fn bbtree_single_node_operations() {
    let single = BBTree::node(BBTree::leaf(), 42, BBTree::leaf());
    assert_eq!(single.size(), 1);
    assert_eq!(single.height(), 1);
    assert_eq!(single.in_order(), ArraySeqStPerSLit![42]);
    assert_eq!(single.pre_order(), ArraySeqStPerSLit![42]);
}

#[test]
fn bbtree_complex_structure() {
    // Build a more complex tree: ((1,2,3),4,(5,6,7))
    let left_subtree = BBTree::node(
        BBTree::node(BBTree::leaf(), 1, BBTree::leaf()),
        2,
        BBTree::node(BBTree::leaf(), 3, BBTree::leaf()),
    );
    let right_subtree = BBTree::node(
        BBTree::node(BBTree::leaf(), 5, BBTree::leaf()),
        6,
        BBTree::node(BBTree::leaf(), 7, BBTree::leaf()),
    );
    let tree = BBTree::node(left_subtree, 4, right_subtree);
    
    assert_eq!(tree.size(), 7);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(tree.pre_order(), ArraySeqStPerSLit![4, 2, 1, 3, 6, 5, 7]);
}

#[test]
fn bstree_empty_operations() {
    let empty_bst = BSTree::<N>::new();
    assert_eq!(empty_bst.size(), 0);
    assert_eq!(empty_bst.height(), 0);
    assert_eq!(empty_bst.contains(&42), B::False);
    assert_eq!(empty_bst.find(&42), None);
    assert_eq!(empty_bst.minimum(), None);
    assert_eq!(empty_bst.maximum(), None);
    assert_eq!(empty_bst.in_order().length(), 0);
    assert_eq!(empty_bst.pre_order().length(), 0);
}

#[test]
fn bstree_single_element() {
    let mut bst = BSTree::new();
    bst.insert(42);
    
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.height(), 1);
    assert_eq!(bst.contains(&42), B::True);
    assert_eq!(bst.contains(&99), B::False);
    assert_eq!(bst.find(&42), Some(&42));
    assert_eq!(bst.find(&99), None);
    assert_eq!(bst.minimum().copied(), Some(42));
    assert_eq!(bst.maximum().copied(), Some(42));
    assert_eq!(bst.in_order(), ArraySeqStPerSLit![42]);
    assert_eq!(bst.pre_order(), ArraySeqStPerSLit![42]);
}

#[test]
fn bstree_duplicate_insertions() {
    let mut bst = BSTree::new();
    bst.insert(5);
    bst.insert(5); // Duplicate
    bst.insert(3);
    bst.insert(5); // Another duplicate
    bst.insert(7);
    
    // BST should handle duplicates appropriately
    assert_eq!(bst.contains(&5), B::True);
    assert_eq!(bst.contains(&3), B::True);
    assert_eq!(bst.contains(&7), B::True);
    
    let inorder = bst.in_order();
    // Check that inorder traversal is still sorted
    for i in 1..inorder.length() {
        assert!(*inorder.nth(i-1) <= *inorder.nth(i));
    }
}

#[test]
fn bstree_left_skewed_tree() {
    let mut bst = BSTree::new();
    // Insert in descending order to create left-skewed tree
    for i in (1..=5).rev() {
        bst.insert(i);
    }
    
    assert_eq!(bst.size(), 5);
    assert_eq!(bst.height(), 5); // Completely unbalanced
    assert_eq!(bst.in_order(), ArraySeqStPerSLit![1, 2, 3, 4, 5]);
    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(5));
}

#[test]
fn bstree_right_skewed_tree() {
    let mut bst = BSTree::new();
    // Insert in ascending order to create right-skewed tree
    for i in 1..=5 {
        bst.insert(i);
    }
    
    assert_eq!(bst.size(), 5);
    assert_eq!(bst.height(), 5); // Completely unbalanced
    assert_eq!(bst.in_order(), ArraySeqStPerSLit![1, 2, 3, 4, 5]);
    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(5));
}

#[test]
fn bstree_random_insertions() {
    let mut bst = BSTree::new();
    let keys = ArraySeqStPerSLit![15, 10, 20, 8, 12, 25, 6, 11, 13, 22, 27];
    
    for i in 0..keys.length() {
        bst.insert(*keys.nth(i));
    }
    
    assert_eq!(bst.size(), keys.length());
    
    // Verify all keys are present
    for i in 0..keys.length() {
        assert_eq!(bst.contains(keys.nth(i)), B::True);
        assert_eq!(bst.find(keys.nth(i)), Some(keys.nth(i)));
    }
    
    // Verify inorder traversal is sorted
    let inorder = bst.in_order();
    for i in 1..inorder.length() {
        assert!(*inorder.nth(i-1) <= *inorder.nth(i));
    }
    
    assert_eq!(bst.minimum().copied(), Some(6));
    assert_eq!(bst.maximum().copied(), Some(27));
}

#[test]
fn bbtree_height_calculation() {
    // Test height calculation for various tree structures
    let leaf = BBTree::<N>::leaf();
    assert_eq!(leaf.height(), 0);
    
    let single = BBTree::node(BBTree::leaf(), 1, BBTree::leaf());
    assert_eq!(single.height(), 1);
    
    let left_heavy = BBTree::node(
        BBTree::node(BBTree::leaf(), 1, BBTree::leaf()),
        2,
        BBTree::leaf(),
    );
    assert_eq!(left_heavy.height(), 2);
    
    let right_heavy = BBTree::node(
        BBTree::leaf(),
        1,
        BBTree::node(BBTree::leaf(), 2, BBTree::leaf()),
    );
    assert_eq!(right_heavy.height(), 2);
}

#[test]
fn bbtree_size_calculation() {
    // Test size calculation for various tree structures
    let leaf = BBTree::<N>::leaf();
    assert_eq!(leaf.size(), 0);
    
    let single = BBTree::node(BBTree::leaf(), 1, BBTree::leaf());
    assert_eq!(single.size(), 1);
    
    let three_nodes = BBTree::node(
        BBTree::node(BBTree::leaf(), 1, BBTree::leaf()),
        2,
        BBTree::node(BBTree::leaf(), 3, BBTree::leaf()),
    );
    assert_eq!(three_nodes.size(), 3);
}

#[test]
fn bstree_edge_case_searches() {
    let mut bst = BSTree::new();
    let keys = ArraySeqStPerSLit![10, 5, 15, 3, 7, 12, 18];
    
    for i in 0..keys.length() {
        bst.insert(*keys.nth(i));
    }
    
    // Test searches for non-existent keys
    assert_eq!(bst.contains(&1), B::False);   // Less than minimum
    assert_eq!(bst.contains(&20), B::False);  // Greater than maximum
    assert_eq!(bst.contains(&6), B::False);   // Between existing keys
    assert_eq!(bst.contains(&14), B::False);  // Between existing keys
    
    assert_eq!(bst.find(&1), None);
    assert_eq!(bst.find(&20), None);
    assert_eq!(bst.find(&6), None);
    assert_eq!(bst.find(&14), None);
}

#[test]
fn bbtree_traversal_consistency() {
    // Build a tree and verify traversal consistency
    let tree = BBTree::node(
        BBTree::node(
            BBTree::node(BBTree::leaf(), 1, BBTree::leaf()),
            2,
            BBTree::leaf(),
        ),
        3,
        BBTree::node(
            BBTree::leaf(),
            4,
            BBTree::node(BBTree::leaf(), 5, BBTree::leaf()),
        ),
    );
    
    let inorder = tree.in_order();
    let preorder = tree.pre_order();
    
    // Inorder should be sorted
    assert_eq!(inorder, ArraySeqStPerSLit![1, 2, 3, 4, 5]);
    
    // Preorder should visit root first
    assert_eq!(*preorder.nth(0), 3); // Root value
    
    // Both should have same length
    assert_eq!(inorder.length(), preorder.length());
    assert_eq!(inorder.length(), tree.size());
}
