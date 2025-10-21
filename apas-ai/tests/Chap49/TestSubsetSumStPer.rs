//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumStPer.

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap49::SubsetSumStPer::SubsetSumStPer::*;
use apas_ai::SubsetSumStPerLit;

#[test]
fn test_subset_sum_st_per_basic() {
    let solver = SubsetSumStPerLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(1));
    assert!(solver.subset_sum(5));
    assert!(solver.subset_sum(6));
    assert!(solver.subset_sum(16));
    assert!(!solver.subset_sum(17));
}

#[test]
fn test_subset_sum_st_per_example_49_2() {
    let solver = SubsetSumStPerLit![1, 1, 1];
    assert!(solver.subset_sum(3));
    assert!(solver.subset_sum(2));
    assert!(solver.subset_sum(1));
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(4));
}

#[test]
fn test_subset_sum_st_per_empty() {
    let solver: SubsetSumStPerS<i32> = SubsetSumStPerLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}
