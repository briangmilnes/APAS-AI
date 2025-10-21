//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtEph.

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap49::SubsetSumMtEph::SubsetSumMtEph::*;
use apas_ai::SubsetSumMtEphLit;

#[test]
fn test_subset_sum_mt_eph_basic() {
    let mut solver = SubsetSumMtEphLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));

    // Test ephemeral mutation
    solver.set(1, 8);
    assert!(solver.subset_sum(8));
}

#[test]
fn test_subset_sum_mt_eph_mutation() {
    let mut solver = SubsetSumMtEphLit![5, 5, 5];
    assert!(solver.subset_sum(15));
    assert!(solver.subset_sum(10));

    solver.set(0, 10);
    assert!(solver.subset_sum(20));
}

#[test]
fn test_subset_sum_mt_eph_empty() {
    let mut solver: SubsetSumMtEphS<i32> = SubsetSumMtEphLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}
