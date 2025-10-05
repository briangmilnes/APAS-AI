//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtEph - Subset Sum (ephemeral, multithreaded).

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap49::SubsetSumMtEph::SubsetSumMtEph::*;

#[test]
fn test_new_empty() {
    let ss: SubsetSumMtEphS<i32> = SubsetSumMtEphTrait::new();
    assert_eq!(ss.multiset().length(), 0);
}

#[test]
fn test_from_multiset() {
    let multiset = ArraySeqMtEphS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumMtEphS::from_multiset(multiset);
    assert_eq!(ss.multiset().length(), 3);
}

#[test]
fn test_subset_sum_exists() {
    let multiset = ArraySeqMtEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumMtEphS::from_multiset(multiset);

    assert!(ss.subset_sum(8));  // 3 + 5 = 8
    assert!(ss.subset_sum(10)); // 3 + 7 = 10
    assert!(ss.subset_sum(12)); // 5 + 7 = 12
}

#[test]
fn test_subset_sum_not_exists() {
    let multiset = ArraySeqMtEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumMtEphS::from_multiset(multiset);

    assert!(!ss.subset_sum(2));
    assert!(!ss.subset_sum(4));
}

#[test]
fn test_subset_sum_zero() {
    let multiset = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let mut ss = SubsetSumMtEphS::from_multiset(multiset);

    assert!(ss.subset_sum(0)); // Empty subset
}

#[test]
fn test_multiset_accessor() {
    let multiset = ArraySeqMtEphS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumMtEphS::from_multiset(multiset);

    let retrieved = ss.multiset();
    assert_eq!(retrieved.length(), 3);
}

#[test]
fn test_clear_memo() {
    let multiset = ArraySeqMtEphS::from_vec(vec![3, 5, 7]);
    let mut ss = SubsetSumMtEphS::from_multiset(multiset);

    ss.subset_sum(8);
    let size_before = ss.memo_size();

    ss.clear_memo();
    assert_eq!(ss.memo_size(), 0);
    assert!(size_before > 0);
}
