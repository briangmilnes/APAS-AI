use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41Codex::ArraySetEnumMtEph::ArraySetEnumMtEph::{ArraySetEnumMtEph, ArraySetEnumMtEphTrait};

#[test]
fn singleton_and_find() {
    let set = ArraySetEnumMtEph::singleton(32, 5);
    assert_eq!(set.size(), 1);
    assert!(set.find(5));
    assert!(!set.find(7));
}

#[test]
fn parallel_filter_marks_expected() {
    let seq = ArraySeqStEphS::from_vec((0usize..32).collect());
    let set = ArraySetEnumMtEph::from_seq(32, &seq);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);
    let filtered = set.filter(&move |index| {
        counter_clone.fetch_add(1, Ordering::Relaxed);
        index % 3 == 0
    });
    assert!(filtered.find(0));
    assert!(filtered.find(30));
    assert!(!filtered.find(31));
    assert!(counter.load(Ordering::Relaxed) >= set.size());
}

#[test]
fn union_and_intersection() {
    let mut left = ArraySetEnumMtEph::empty(32);
    let mut right = ArraySetEnumMtEph::empty(32);
    for value in [1, 3, 5, 7] {
        let _ = left.insert(value);
    }
    for value in [5, 7, 9] {
        let _ = right.insert(value);
    }
    let union = left.union(&right);
    let intersection = left.intersection(&right);

    assert_eq!(union.size(), 5);
    for expected in [1, 3, 5, 7, 9] {
        assert!(union.find(expected));
    }

    assert_eq!(intersection.size(), 2);
    assert!(intersection.find(5));
    assert!(intersection.find(7));
}
