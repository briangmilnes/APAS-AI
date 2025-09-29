use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{AVLTreeSetStEph, AVLTreeSetStEphTrait};

#[test]
fn insert_delete_and_membership() {
    let mut set = AVLTreeSetStEph::empty();
    for value in [2, 7, 8, 11] {
        set.insert(value);
    }
    assert_eq!(set.size(), 4);
    assert!(set.find(&7));
    assert!(!set.find(&3));

    set.delete(&7);
    assert_eq!(set.size(), 3);
    assert!(!set.find(&7));
}

#[test]
fn from_seq_and_difference() {
    let seq = ArraySeqStEphS::from_vec(vec![2, 7, 2, 11, 4]);
    let set = AVLTreeSetStEph::from_seq(&seq);
    assert_eq!(set.size(), 4);

    let mut other = AVLTreeSetStEph::empty();
    other.insert(2);
    other.insert(7);
    let diff = set.difference(&other);
    assert_eq!(diff.size(), 2);
    assert!(diff.find(&4));
    assert!(diff.find(&11));
}

#[test]
fn intersection_and_union() {
    let mut left = AVLTreeSetStEph::empty();
    let mut right = AVLTreeSetStEph::empty();
    for value in [1, 3, 5, 7] {
        left.insert(value);
    }
    for value in [5, 7, 9] {
        right.insert(value);
    }

    let intersection = left.intersection(&right);
    let union = left.union(&right);

    assert_eq!(intersection.size(), 2);
    assert!(intersection.find(&5));
    assert!(intersection.find(&7));

    assert_eq!(union.size(), 5);
    for expected in [1, 3, 5, 7, 9] {
        assert!(union.find(&expected));
    }
}
