use apas_ai::Chap37::BSTSplayStEph::BSTSplayStEph::*;
use apas_ai::*;

#[test]
fn splay_basic_behaviour() {
    let mut bst = BSTreeSplay::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.contains(&3), B::True);
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
