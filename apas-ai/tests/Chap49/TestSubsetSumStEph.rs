//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumStEph - Subset Sum (ephemeral, single-threaded).

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap49::SubsetSumStEph::SubsetSumStEph::*;

#[test]
fn test_new_empty() {
    let ss: SubsetSumStEphS<i32> = SubsetSumStEphTrait::new();
    assert_eq!(ss.multiset().length(), 0);
}

#[test]
fn test_from_multiset() {
    let multiset = ArraySeqStEphS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumStEphS::from_multiset(multiset);
    assert_eq!(ss.multiset().length(), 3);
}

#[test]
fn test_subset_sum_exists() {
    let multiset = ArraySeqStEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    assert!(ss.subset_sum(8)); // 3 + 5 = 8
    assert!(ss.subset_sum(10)); // 3 + 7 = 10
    assert!(ss.subset_sum(12)); // 5 + 7 = 12
}

#[test]
fn test_subset_sum_not_exists() {
    let multiset = ArraySeqStEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    assert!(!ss.subset_sum(2));
    assert!(!ss.subset_sum(4));
    assert!(!ss.subset_sum(6));
}

#[test]
fn test_subset_sum_single_element() {
    let multiset = ArraySeqStEphS::from_vec(vec![5]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    assert!(ss.subset_sum(5));
    assert!(!ss.subset_sum(3));
}

#[test]
fn test_subset_sum_all_elements() {
    let multiset = ArraySeqStEphS::from_vec(vec![2, 3, 5]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    assert!(ss.subset_sum(10)); // 2 + 3 + 5 = 10
}

#[test]
fn test_subset_sum_zero() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    assert!(ss.subset_sum(0)); // Empty subset
}

#[test]
fn test_multiset_accessor() {
    let multiset = ArraySeqStEphS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumStEphS::from_multiset(multiset);

    let retrieved = ss.multiset();
    assert_eq!(retrieved.length(), 3);
}

#[test]
fn test_set_element() {
    let multiset = ArraySeqStEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    ss.set(0, 10);
    assert_eq!(*ss.multiset().nth(0), 10);
}

#[test]
fn test_clear_memo() {
    let multiset = ArraySeqStEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    ss.subset_sum(8);
    let size_before = ss.memo_size();

    ss.clear_memo();
    assert_eq!(ss.memo_size(), 0);
    assert!(size_before > 0);
}

#[test]
fn test_large_multiset() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);

    assert!(ss.subset_sum(15));
    assert!(ss.subset_sum(55));
}
