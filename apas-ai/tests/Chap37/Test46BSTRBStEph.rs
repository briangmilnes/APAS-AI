use apas_ai::Chap37::BSTRBStEph::BSTRBStEph::*;
use apas_ai::*;

#[test]
fn rb_insert_find_and_bounds() {
    let mut bst = BSTreeRB::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert!(bst.height() <= 5);
    assert_eq!(bst.find(&3), Some(&3));
    assert_eq!(bst.find(&9), None);
    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(7));
    let inorder = bst.in_order();
    let expected = [1, 2, 3, 4, 5, 6, 7];
    assert_eq!(inorder.length(), expected.len());
    for (exp, value) in expected.iter().zip(inorder.iter()) {
        assert_eq!(*value, *exp);
    }
}

#[test]
fn rb_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeRB::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}
