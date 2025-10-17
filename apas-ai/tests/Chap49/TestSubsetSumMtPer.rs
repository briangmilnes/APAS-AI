//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtPer - Subset Sum (persistent, multithreaded).

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Chap49::SubsetSumMtPer::SubsetSumMtPer::*;
use apas_ai::Types::Types::*;

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

#[test]
fn test_subset_sum_single_element() {
    let multiset = ArraySeqMtPerS::from_vec(vec![5]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(5));
    assert!(!ss.subset_sum(3));
}

#[test]
fn test_subset_sum_zero() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(0)); // Empty subset
}

#[test]
fn test_empty_multiset() {
    let multiset = ArraySeqMtPerS::from_vec(vec![] as Vec<i32>);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(0));
    assert!(!ss.subset_sum(1));
}

#[test]
fn test_duplicates() {
    let multiset = ArraySeqMtPerS::from_vec(vec![2, 2, 2]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(2));
    assert!(ss.subset_sum(4));
    assert!(ss.subset_sum(6));
    assert!(!ss.subset_sum(5));
}

#[test]
fn test_large_multiset() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(15));
    assert!(ss.subset_sum(55)); // All elements
}

#[test]
fn test_large_target() {
    let multiset = ArraySeqMtPerS::from_vec(vec![10, 20, 30, 40]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(100)); // All elements
    assert!(ss.subset_sum(70)); // 30 + 40
    assert!(!ss.subset_sum(101));
}

#[test]
fn test_persistence() {
    let multiset1 = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss1 = SubsetSumMtPerS::from_multiset(multiset1);

    // Persistent - check that original is unchanged
    let multiset2 = ss1.multiset().clone();
    assert_eq!(multiset2.length(), 3);
}

#[test]
fn test_display_formatting() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    let display = format!("{}", ss);
    assert!(display.contains("SubsetSumMtPer"));
}

#[test]
fn test_debug_formatting() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    let debug = format!("{:?}", ss);
    assert!(debug.contains("SubsetSumMtPerS"));
}

#[test]
fn test_clone() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    let cloned = ss.clone();
    assert_eq!(cloned.multiset().length(), 3);
}

#[test]
fn test_equality() {
    let multiset1 = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss1 = SubsetSumMtPerS::from_multiset(multiset1);

    let multiset2 = ArraySeqMtPerS::from_vec(vec![1, 2, 3]);
    let ss2 = SubsetSumMtPerS::from_multiset(multiset2);

    assert_eq!(ss1, ss2);
}

#[test]
fn test_single_large_element() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1000]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(1000));
    assert!(!ss.subset_sum(999));
}

#[test]
fn test_all_elements_sum() {
    let multiset = ArraySeqMtPerS::from_vec(vec![2, 3, 5]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(10)); // 2 + 3 + 5
}

#[test]
fn test_multiple_queries() {
    let multiset = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4]);
    let ss = SubsetSumMtPerS::from_multiset(multiset);

    assert!(ss.subset_sum(5));
    assert!(ss.subset_sum(6));
    assert!(ss.subset_sum(7));
    assert!(!ss.subset_sum(11));
}
