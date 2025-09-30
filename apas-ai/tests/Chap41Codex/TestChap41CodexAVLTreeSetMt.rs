//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.\n//! Tests for Chap41Codex AVLTreeSetMtEph.\n\nuse std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_ai::Chap41Codex::AVLTreeSetMtEph::AVLTreeSetMtEph::{AVLTreeSetMtEph, AVLTreeSetMtEphTrait};

#[test]
fn construct_and_find() {
    let seq = ArraySeqMtEphS::from_vec(vec![2, 7, 11]);
    let set = AVLTreeSetMtEph::from_seq(&seq);
    assert!(set.find(&2));
    assert!(set.find(&11));
    assert!(!set.find(&5));
}

#[test]
fn parallel_filter() {
    let seq = ArraySeqMtEphS::from_vec((0i32..32).collect());
    let set = AVLTreeSetMtEph::from_seq(&seq);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);
    let filtered = set.filter(move |value| {
        counter_clone.fetch_add(1, Ordering::Relaxed);
        value % 4 == 0
    });
    assert!(filtered.find(&0));
    assert!(filtered.find(&28));
    assert!(!filtered.find(&30));
    assert!(counter.load(Ordering::Relaxed) >= 32);
}

#[test]
fn union_and_difference() {
    let seq_left = ArraySeqMtEphS::from_vec(vec![1, 3, 5, 7]);
    let seq_right = ArraySeqMtEphS::from_vec(vec![5, 7, 9]);
    let left = AVLTreeSetMtEph::from_seq(&seq_left);
    let right = AVLTreeSetMtEph::from_seq(&seq_right);

    let union = left.union(&right);
    let difference = union.difference(&right);

    assert!(union.find(&9));
    assert!(union.find(&1));
    assert!(difference.find(&1));
    assert!(!difference.find(&9));
}
