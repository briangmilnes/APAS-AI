//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtPer - Subset Sum (persistent, multithreaded).

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Chap49::SubsetSumMtPer::SubsetSumMtPer::*;

#[test]
fn test_new_empty() {
    let ss: SubsetSumMtPerS<i32> = SubsetSumMtPerTrait::new();
    assert_eq!(ss.multiset().length(), 0);
}

#[test]
fn test_from_multiset() {
    let multiset = ArraySeqMtPerS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);
    assert_eq!(ss.multiset().length(), 3);
}

#[test]
fn test_subset_sum_exists() {
    let multiset = ArraySeqMtPerS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(8));
    assert!(ss.subset_sum(10));
}

#[test]
fn test_subset_sum_not_exists() {
    let multiset = ArraySeqMtPerS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(!ss.subset_sum(2));
}
