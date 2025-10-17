//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumStPer - Subset Sum (persistent, single-threaded).

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap49::SubsetSumStPer::SubsetSumStPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_new_empty() {
    let ss: SubsetSumStPerS<i32> = SubsetSumStPerTrait::new();
    assert_eq!(ss.multiset().length(), 0);
}

#[test]
fn test_from_multiset() {
    let multiset = ArraySeqStPerS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumStPerS::from_multiset(multiset);
    assert_eq!(ss.multiset().length(), 3);
}

#[test]
fn test_subset_sum_exists() {
    let multiset = ArraySeqStPerS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumStPerS::from_multiset(multiset);

    assert!(ss.subset_sum(8));
    assert!(ss.subset_sum(10));
}

#[test]
fn test_subset_sum_not_exists() {
    let multiset = ArraySeqStPerS::from_vec(vec![3, 5, 7]);
    let ss = SubsetSumStPerS::from_multiset(multiset);

    assert!(!ss.subset_sum(2));
}
