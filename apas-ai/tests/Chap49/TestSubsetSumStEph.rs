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

#[test]
fn test_multiset_mut() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    let ms_mut = ss.multiset_mut();
    ms_mut.set(0, 10);
    assert_eq!(*ss.multiset().nth(0), 10);
}

#[test]
fn test_memo_reuse() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3, 4]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    // First call builds memo
    assert!(ss.subset_sum(5));
    let size_after_first = ss.memo_size();
    assert!(size_after_first > 0);
    
    // Second call reuses memo
    assert!(ss.subset_sum(5));
    assert_eq!(ss.memo_size(), size_after_first);
}

#[test]
fn test_empty_multiset() {
    let multiset = ArraySeqStEphS::from_vec(vec![] as Vec<i32>);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    assert!(ss.subset_sum(0));
    assert!(!ss.subset_sum(1));
}

#[test]
fn test_duplicates() {
    let multiset = ArraySeqStEphS::from_vec(vec![2, 2, 2]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    assert!(ss.subset_sum(2));
    assert!(ss.subset_sum(4));
    assert!(ss.subset_sum(6));
    assert!(!ss.subset_sum(5));
}

#[test]
fn test_large_target() {
    let multiset = ArraySeqStEphS::from_vec(vec![10, 20, 30, 40]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    assert!(ss.subset_sum(100)); // All elements
    assert!(ss.subset_sum(70));  // 30 + 40
    assert!(!ss.subset_sum(101));
}

#[test]
fn test_mutation_after_solve() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    assert!(ss.subset_sum(5)); // 2 + 3 or 1 + 2 + 2? Actually 2 + 3 = 5
    
    // Mutate and clear memo
    ss.set(0, 10);
    ss.clear_memo();
    
    // Now we have [10, 2, 3]
    assert!(ss.subset_sum(12)); // 10 + 2
    assert!(ss.subset_sum(13)); // 10 + 3
    assert!(ss.subset_sum(15)); // 10 + 2 + 3
    assert!(ss.subset_sum(5));  // 2 + 3 still possible
}

#[test]
fn test_display_formatting() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumStEphS::from_multiset(multiset);
    
    let display = format!("{}", ss);
    assert!(display.contains("SubsetSumStEph"));
}

#[test]
fn test_debug_formatting() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumStEphS::from_multiset(multiset);
    
    let debug = format!("{:?}", ss);
    assert!(debug.contains("SubsetSumStEphS"));
}

#[test]
fn test_clone() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumStEphS::from_multiset(multiset);
    
    let cloned = ss.clone();
    assert_eq!(cloned.multiset().length(), 3);
}

#[test]
fn test_equality() {
    let multiset1 = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss1 = SubsetSumStEphS::from_multiset(multiset1);
    
    let multiset2 = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss2 = SubsetSumStEphS::from_multiset(multiset2);
    
    assert_eq!(ss1, ss2);
}

#[test]
fn test_single_large_element() {
    let multiset = ArraySeqStEphS::from_vec(vec![1000]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    assert!(ss.subset_sum(1000));
    assert!(!ss.subset_sum(999));
    assert!(!ss.subset_sum(1001));
}

#[test]
fn test_negative_target_direct() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    // Negative targets should return false
    assert!(!ss.subset_sum(-1));
    assert!(!ss.subset_sum(-100));
}

#[test]
fn test_into_iterator_owned() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumStEphS::from_multiset(multiset);
    
    let collected: Vec<_> = ss.into_iter().collect();
    assert_eq!(collected.len(), 3);
}

#[test]
fn test_into_iterator_ref() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let ss = SubsetSumStEphS::from_multiset(multiset);
    
    let collected: Vec<_> = (&ss).into_iter().collect();
    assert_eq!(collected.len(), 3);
    
    // ss should still be usable
    assert_eq!(ss.multiset().length(), 3);
}

#[test]
fn test_into_iterator_mut() {
    let multiset = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let mut ss = SubsetSumStEphS::from_multiset(multiset);
    
    let collected: Vec<_> = (&mut ss).into_iter().collect();
    assert_eq!(collected.len(), 3);
    
    // ss should still be usable
    assert_eq!(ss.multiset().length(), 3);
}
