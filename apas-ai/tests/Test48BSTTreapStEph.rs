use apas_ai::BSTTreapStEph::BSTTreapStEph::*;
use apas_ai::*;

fn inorder<T: StT + Ord>(tree: &BSTreeTreap<T>) -> Vec<T> {
    tree.in_order().iter().cloned().collect()
}

#[test]
fn treap_insert_find_stays_balanced() {
    let mut bst = BSTreeTreap::new();
    for value in 0..128 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 128);
    let height = bst.height();
    assert!(height <= 24, "height {} too large", height);
    assert_eq!(bst.find(&42), Some(&42));
    assert_eq!(bst.find(&256), None);
    assert_eq!(bst.minimum().copied(), Some(0));
    assert_eq!(bst.maximum().copied(), Some(127));
    assert_eq!(inorder(&bst), (0..128).collect::<Vec<_>>());
}

#[test]
fn treap_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeTreap::new();
    bst.insert(5);
    bst.insert(5);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&5), Some(&5));
}
