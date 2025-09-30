//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap41Codex ArraySetEnumStEph.

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::{ArraySetEnumStEph, ArraySetEnumStEphTrait};

#[test]
fn size_and_membership() {
    let mut set = ArraySetEnumStEph::empty(16);
    let _ = set.insert(3);
    let _ = set.insert(7);
    assert_eq!(set.size(), 2);
    assert!(set.find(3));
    assert!(!set.find(9));

    let _ = set.delete(3);
    assert_eq!(set.size(), 1);
    assert!(!set.find(3));
}

#[test]
fn union_and_difference() {
    let mut a = ArraySetEnumStEph::empty(16);
    let mut b = ArraySetEnumStEph::empty(16);
    for value in [2, 4, 6] {
        let _ = a.insert(value);
    }
    for value in [4, 6, 8] {
        let _ = b.insert(value);
    }

    let union = a.union(&b);
    let difference = union.difference(&b);

    assert_eq!(union.size(), 4);
    for expected in [2, 4, 6, 8] {
        assert!(union.find(expected));
    }

    assert_eq!(difference.size(), 1);
    assert!(difference.find(2));
}

#[test]
fn from_seq_removes_duplicates() {
    let seq = ArraySeqStEphS::from_vec(vec![1usize, 4, 1, 4, 9]);
    let set = ArraySetEnumStEph::from_seq(16, &seq);
    assert_eq!(set.size(), 3);
    for expected in [1, 4, 9] {
        assert!(set.find(expected));
    }
}
