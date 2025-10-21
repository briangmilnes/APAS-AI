//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumStEph.

use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap49::SubsetSumStEph::SubsetSumStEph::*;
use apas_ai::SubsetSumStEphLit;

#[test]
fn test_subset_sum_st_eph_basic() {
    let mut solver = SubsetSumStEphLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));

    // Test ephemeral mutation
    solver.set(0, 3);
    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(7));
}

#[test]
fn test_subset_sum_st_eph_mutation() {
    let mut solver = SubsetSumStEphLit![1, 1, 1];
    assert!(solver.subset_sum(3));

    solver.set(2, 5);
    assert!(!solver.subset_sum(3));
    assert!(solver.subset_sum(7));
}

#[test]
fn test_subset_sum_st_eph_empty() {
    let mut solver: SubsetSumStEphS<i32> = SubsetSumStEphLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}
