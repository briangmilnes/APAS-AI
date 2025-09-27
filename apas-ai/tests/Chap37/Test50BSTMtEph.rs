//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;
use apas_ai::Types::Types::*;
use apas_ai::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
use apas_ai::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
use apas_ai::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
use apas_ai::Chap37::BSTRBMtEph::BSTRBMtEph::*;
use apas_ai::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
use apas_ai::Types::*;
use apas_ai::*;

#[test]
fn mt_plain_basic_ops() {
    let bst = BSTree::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&3), Some(3));
    assert_eq!(bst.contains(&9), B::False);
    assert_eq!(bst.minimum(), Some(1));
    assert_eq!(bst.maximum(), Some(7));
}

#[test]
fn mt_avl_basic_ops() {
    let bst = BSTreeAVL::new();
    for value in [10, 5, 15, 2, 7, 12, 20] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&7), Some(7));
    assert_eq!(bst.contains(&30), B::False);
}

#[test]
fn mt_rb_basic_ops() {
    let bst = BSTreeRB::new();
    for value in 0..16 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 16);
    assert_eq!(bst.find(&8), Some(8));
}

#[test]
fn mt_bbalpha_basic_ops() {
    let bst = BSTreeBBAlpha::new();
    for value in 0..32 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 32);
    assert_eq!(bst.find(&12), Some(12));
}

#[test]
fn mt_treap_basic_ops() {
    let bst = BSTreeTreap::new();
    for value in 0..16 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 16);
    assert_eq!(bst.find(&10), Some(10));
}

#[test]
fn mt_splay_basic_ops() {
    let bst = BSTreeSplay::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&5), Some(5));
}
