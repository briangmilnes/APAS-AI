use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41Codex::AVLTreeSetMtEph::AVLTreeSetMtEph::{
    AVLTreeSetMtEph, AVLTreeSetMtEphTrait,
};
use apas_ai::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{
    AVLTreeSetStEph, AVLTreeSetStEphTrait,
};
use apas_ai::Types::Types::*;

fn seq_values<T: StT + Clone>(seq: &ArraySeqStEphS<T>) -> Vec<T> {
    let mut out = Vec::with_capacity(seq.length());
    for i in 0..seq.length() {
        out.push(seq.nth(i).clone());
    }
    out
}

#[test]
fn avl_tree_set_st_ops() {
    let mut set = AVLTreeSetStEph::empty();
    set.insert(5);
    set.insert(2);
    set.insert(9);
    set.insert(5);
    assert_eq!(set.size(), 3);
    assert!(set.find(&2));
    assert!(!set.find(&7));

    let filtered = set.filter(|x| *x >= 5);
    assert_eq!(seq_values(&filtered.to_seq()), vec![5, 9]);

    let other = AVLTreeSetStEph::from_seq(&ArraySeqStEphS::from_vec(vec![1, 5, 9, 12]));
    let union = set.union(&other);
    assert_eq!(seq_values(&union.to_seq()), vec![1, 2, 5, 9, 12]);
    let intersection = set.intersection(&other);
    assert_eq!(seq_values(&intersection.to_seq()), vec![5, 9]);
    let difference = set.difference(&other);
    assert_eq!(seq_values(&difference.to_seq()), vec![2]);

    set.delete(&5);
    assert!(!set.find(&5));
}

#[test]
fn avl_tree_set_mt_ops() {
    let set = AVLTreeSetMtEph::from_seq(&ArraySeqMtEphS::from_vec(vec![10, 4, 7, 10, 3]));
    assert_eq!(set.size(), 4);
    assert!(set.find(&4));

    let filtered = set.filter(|x| *x % 2 == 0);
    assert_eq!(filtered.size(), 2);
    assert!(filtered.find(&4));
    assert!(filtered.find(&10));

    let other = AVLTreeSetMtEph::from_seq(&ArraySeqMtEphS::from_vec(vec![1, 4, 8, 12]));
    let union = set.union(&other);
    assert_eq!(union.size(), 7);
    let intersection = set.intersection(&other);
    assert_eq!(intersection.size(), 1);
    assert!(intersection.find(&4));
    let difference = set.difference(&other);
    assert_eq!(difference.size(), 3);
    assert!(difference.find(&3));
    assert!(difference.find(&7));
    assert!(difference.find(&10));

    set.delete(&4);
    assert!(!set.find(&4));
    set.insert(4);
    assert!(set.find(&4));
}
