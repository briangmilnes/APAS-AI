//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
use apas_ai::Chap37::BSTPlainStEph::BSTPlainStEph::*;
use apas_ai::Types::Types::*;

// Note: BalBinNodeLit macro cannot be tested externally because BalBinNode::new() is private
// and the struct fields (left, value, right) are pub(crate) only.
// The macro is meant for internal use within the BalBinTreeStEph module.

#[test]
fn inorder_and_preorder_traversals_match_definitions() {
    let tree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 2, BalBinTree::leaf()),
        4,
        BalBinTree::node(BalBinTree::leaf(), 6, BalBinTree::leaf()),
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
    assert_eq!(bst.contains(&4), true);
    assert_eq!(bst.contains(&5), true);
    assert_eq!(bst.contains(&8), false);
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
fn balbintree_empty_leaf_operations() {
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.size(), 0);
    assert_eq!(leaf.height(), 0);
    assert_eq!(leaf.in_order().length(), 0);
    assert_eq!(leaf.pre_order().length(), 0);
}

#[test]
fn balbintree_single_node_operations() {
    let single = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    assert_eq!(single.size(), 1);
    assert_eq!(single.height(), 1);
    assert_eq!(single.in_order(), ArraySeqStPerSLit![42]);
    assert_eq!(single.pre_order(), ArraySeqStPerSLit![42]);
}

#[test]
fn balbintree_complex_structure() {
    // Build a more complex tree: ((1,2,3),4,(5,6,7))
    let left_subtree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    let right_subtree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
        6,
        BalBinTree::node(BalBinTree::leaf(), 7, BalBinTree::leaf()),
    );
    let tree = BalBinTree::node(left_subtree, 4, right_subtree);

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
    assert_eq!(empty_bst.contains(&42), false);
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
    assert_eq!(bst.contains(&42), true);
    assert_eq!(bst.contains(&99), false);
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
    assert_eq!(bst.contains(&5), true);
    assert_eq!(bst.contains(&3), true);
    assert_eq!(bst.contains(&7), true);

    let inorder = bst.in_order();
    // Check that inorder traversal is still sorted
    for i in 1..inorder.length() {
        assert!(*inorder.nth(i - 1) <= *inorder.nth(i));
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
        assert_eq!(bst.contains(keys.nth(i)), true);
        assert_eq!(bst.find(keys.nth(i)), Some(keys.nth(i)));
    }

    // Verify inorder traversal is sorted
    let inorder = bst.in_order();
    for i in 1..inorder.length() {
        assert!(*inorder.nth(i - 1) <= *inorder.nth(i));
    }

    assert_eq!(bst.minimum().copied(), Some(6));
    assert_eq!(bst.maximum().copied(), Some(27));
}

#[test]
fn balbintree_height_calculation() {
    // Test height calculation for various tree structures
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.height(), 0);

    let single = BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf());
    assert_eq!(single.height(), 1);

    let left_heavy = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::leaf(),
    );
    assert_eq!(left_heavy.height(), 2);

    let right_heavy = BalBinTree::node(
        BalBinTree::leaf(),
        1,
        BalBinTree::node(BalBinTree::leaf(), 2, BalBinTree::leaf()),
    );
    assert_eq!(right_heavy.height(), 2);
}

#[test]
fn balbintree_size_calculation() {
    // Test size calculation for various tree structures
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.size(), 0);

    let single = BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf());
    assert_eq!(single.size(), 1);

    let three_nodes = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
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
    assert_eq!(bst.contains(&1), false); // Less than minimum
    assert_eq!(bst.contains(&20), false); // Greater than maximum
    assert_eq!(bst.contains(&6), false); // Between existing keys
    assert_eq!(bst.contains(&14), false); // Between existing keys

    assert_eq!(bst.find(&1), None);
    assert_eq!(bst.find(&20), None);
    assert_eq!(bst.find(&6), None);
    assert_eq!(bst.find(&14), None);
}

#[test]
fn balbintree_traversal_consistency() {
    // Build a tree and verify traversal consistency
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::leaf(),
        ),
        3,
        BalBinTree::node(
            BalBinTree::leaf(),
            4,
            BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
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

#[test]
fn balbintree_is_leaf_check() {
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.is_leaf(), true);

    let single = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    assert_eq!(single.is_leaf(), false);

    let complex = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    assert_eq!(complex.is_leaf(), false);
}

#[test]
fn balbintree_unbalanced_left() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::leaf(),
        ),
        3,
        BalBinTree::leaf(),
    );

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3]);
    assert_eq!(tree.pre_order(), ArraySeqStPerSLit![3, 2, 1]);
}

#[test]
fn balbintree_unbalanced_right() {
    let tree = BalBinTree::node(
        BalBinTree::leaf(),
        1,
        BalBinTree::node(
            BalBinTree::leaf(),
            2,
            BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
        ),
    );

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3]);
    assert_eq!(tree.pre_order(), ArraySeqStPerSLit![1, 2, 3]);
}

#[test]
fn bstree_large_dataset() {
    let mut bst = BSTree::new();
    let n = 100;

    // Insert 1 to 100
    for i in 1..=n {
        bst.insert(i);
    }

    assert_eq!(bst.size(), n);
    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(n));

    // Verify all elements are present
    for i in 1..=n {
        assert_eq!(bst.contains(&i), true);
        assert_eq!(bst.find(&i), Some(&i));
    }

    // Verify inorder is sorted
    let inorder = bst.in_order();
    assert_eq!(inorder.length(), n);
    for i in 0..(n - 1) {
        assert!(*inorder.nth(i) < *inorder.nth(i + 1));
    }
}

#[test]
fn bstree_alternating_insertions() {
    let mut bst = BSTree::new();
    // Insert in alternating pattern: 50, 25, 75, 12, 37, 62, 87...
    let keys = ArraySeqStPerSLit![50, 25, 75, 12, 37, 62, 87, 6, 18, 31, 43];

    for i in 0..keys.length() {
        bst.insert(*keys.nth(i));
    }

    assert_eq!(bst.size(), keys.length());

    // Verify inorder is sorted
    let inorder = bst.in_order();
    for i in 1..inorder.length() {
        assert!(*inorder.nth(i - 1) < *inorder.nth(i));
    }

    assert_eq!(bst.minimum().copied(), Some(6));
    assert_eq!(bst.maximum().copied(), Some(87));
}

#[test]
fn balbintree_large_balanced_tree() {
    // Build a perfectly balanced tree with 7 nodes
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
        ),
        4,
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
            6,
            BalBinTree::node(BalBinTree::leaf(), 7, BalBinTree::leaf()),
        ),
    );

    assert_eq!(tree.size(), 7);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(tree.pre_order(), ArraySeqStPerSLit![4, 2, 1, 3, 6, 5, 7]);
}

#[test]
fn bstree_negative_numbers() {
    let mut bst = BSTree::new();
    let keys = ArraySeqStPerSLit![-10, -5, -15, -3, -7, -20];

    for i in 0..keys.length() {
        bst.insert(*keys.nth(i));
    }

    assert_eq!(bst.size(), keys.length());
    assert_eq!(bst.minimum().copied(), Some(-20));
    assert_eq!(bst.maximum().copied(), Some(-3));

    let inorder = bst.in_order();
    assert_eq!(inorder, ArraySeqStPerSLit![-20, -15, -10, -7, -5, -3]);
}

#[test]
fn bstree_mixed_positive_negative() {
    let mut bst = BSTree::new();
    let keys = ArraySeqStPerSLit![0, -5, 5, -3, 3, -8, 8];

    for i in 0..keys.length() {
        bst.insert(*keys.nth(i));
    }

    assert_eq!(bst.minimum().copied(), Some(-8));
    assert_eq!(bst.maximum().copied(), Some(8));

    let inorder = bst.in_order();
    for i in 1..inorder.length() {
        assert!(*inorder.nth(i - 1) < *inorder.nth(i));
    }
}

#[test]
fn balbintree_only_left_children() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::leaf(),
            ),
            3,
            BalBinTree::leaf(),
        ),
        4,
        BalBinTree::leaf(),
    );

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3, 4]);
}

#[test]
fn balbintree_only_right_children() {
    let tree = BalBinTree::node(
        BalBinTree::leaf(),
        1,
        BalBinTree::node(
            BalBinTree::leaf(),
            2,
            BalBinTree::node(
                BalBinTree::leaf(),
                3,
                BalBinTree::node(BalBinTree::leaf(), 4, BalBinTree::leaf()),
            ),
        ),
    );

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3, 4]);
}
