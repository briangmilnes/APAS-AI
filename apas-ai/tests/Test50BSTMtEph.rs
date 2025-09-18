use apas_ai::BSTPlainMtEph::BSTPlainMtEph::*;
use apas_ai::BSTAVLMtEph::BSTAVLMtEph::*;
use apas_ai::BSTRBMtEph::BSTRBMtEph::*;
use apas_ai::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
use apas_ai::BSTTreapMtEph::BSTTreapMtEph::*;
use apas_ai::BSTSplayMtEph::BSTSplayMtEph::*;
use apas_ai::*;



#[test]
fn mt_plain_basic_ops() {
    let mut bst = BSTree::<i32>::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&3), Some(&3));
    assert_eq!(bst.contains(&9), B::False);
    assert_eq!(bst.minimum(), Some(&1));
    assert_eq!(bst.maximum(), Some(&7));
}

#[test]
fn mt_avl_basic_ops() {
    let mut bst = BSTreeAVL::<i32>::new();
    for value in [10, 5, 15, 2, 7, 12, 20] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&7), Some(&7));
    assert_eq!(bst.contains(&30), B::False);
}

#[test]
fn mt_rb_basic_ops() {
    let mut bst = BSTreeRB::<i32>::new();
    for value in 0..16 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 16);
    assert_eq!(bst.find(&8), Some(&8));
}

#[test]
fn mt_bbalpha_basic_ops() {
    let mut bst = BSTreeBBAlpha::<i32>::new();
    for value in 0..32 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 32);
    assert_eq!(bst.find(&12), Some(&12));
}

#[test]
fn mt_treap_basic_ops() {
    let mut bst = BSTreeTreap::<i32>::new();
    for value in 0..16 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 16);
    assert_eq!(bst.find(&10), Some(&10));
}

#[test]
fn mt_splay_basic_ops() {
    let mut bst = BSTreeSplay::<i32>::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&5), Some(&5));
}
