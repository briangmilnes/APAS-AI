//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtPer.

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Chap49::SubsetSumMtPer::SubsetSumMtPer::*;
use apas_ai::SubsetSumMtPerLit;

#[test]
fn test_subset_sum_mt_per_basic() {
    let solver = SubsetSumMtPerLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(16));
}

#[test]
fn test_subset_sum_mt_per_example() {
    let solver = SubsetSumMtPerLit![1, 1, 1];
    assert!(solver.subset_sum(3));
    assert!(solver.subset_sum(2));
    assert!(!solver.subset_sum(4));
}

#[test]
fn test_subset_sum_mt_per_empty() {
    let solver: SubsetSumMtPerS<i32> = SubsetSumMtPerLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}
