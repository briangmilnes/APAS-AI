//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Subset Sum.

use apas_ai::{
    ArraySeqMtEphSLit, ArraySeqStEphS, ArraySeqStPerS,
    Chap18::ArraySeqMtEph::ArraySeqMtEph::*,
    Chap49::SubsetSumMtEph::SubsetSumMtEph::*,
    Chap49::SubsetSumMtPer::SubsetSumMtPer::*, Chap49::SubsetSumStEph::SubsetSumStEph::*,
    Chap49::SubsetSumStPer::SubsetSumStPer::*, SubsetSumMtEphLit, SubsetSumMtPerLit, SubsetSumStEphLit,
    SubsetSumStPerLit, Types::Types::*
};


// Test cases from APAS textbook
#[test]
fn test_subset_sum_st_per_basic() {
    let solver = SubsetSumStPerLit![1, 4, 2, 9];

    // Example 49.1: SS({1, 4, 2, 9}, 8) returns false
    assert!(!solver.subset_sum(8));

    // Example 49.1: SS({1, 4, 2, 9}, 12) returns true (1 + 2 + 9 = 12)
    assert!(solver.subset_sum(12));

    // Additional test cases
    assert!(solver.subset_sum(0)); // Empty subset
    assert!(solver.subset_sum(1)); // Single element
    assert!(solver.subset_sum(5)); // 1 + 4 = 5
    assert!(solver.subset_sum(6)); // 2 + 4 = 6
    assert!(solver.subset_sum(16)); // 1 + 4 + 2 + 9 = 16
    assert!(!solver.subset_sum(17)); // No subset sums to 17
}

#[test]
fn test_subset_sum_st_per_example_49_2() {
    // Example 49.2: SS({1, 1, 1}, 3) returns true
    let solver = SubsetSumStPerLit![1, 1, 1];
    assert!(solver.subset_sum(3)); // 1 + 1 + 1 = 3
    assert!(solver.subset_sum(2)); // 1 + 1 = 2
    assert!(solver.subset_sum(1)); // 1 = 1
    assert!(solver.subset_sum(0)); // Empty subset
    assert!(!solver.subset_sum(4)); // No subset sums to 4
}

#[test]
fn test_subset_sum_st_eph_basic() {
    let mut solver = SubsetSumStEphLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));

    // Test ephemeral mutation
    solver.set(0, 3); // Change first element from 1 to 3
    assert!(!solver.subset_sum(8)); // 3 + 4 + 2 + 9 = 18, no subset = 8
    assert!(solver.subset_sum(7)); // 3 + 4 = 7
}

#[test]
fn test_subset_sum_mt_per_basic() {
    let solver = SubsetSumMtPerLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(16));
}

#[test]
fn test_subset_sum_mt_eph_basic() {
    let mut solver = SubsetSumMtEphLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));

    // Test ephemeral mutation
    solver.set(1, 5); // Change second element from 4 to 5
    assert!(solver.subset_sum(8)); // 1 + 2 + 5 = 8
}

#[test]
fn test_subset_sum_edge_cases() {
    // Empty multiset
    let empty_solver: SubsetSumStPerS<i32> = SubsetSumStPerLit![];
    assert!(empty_solver.subset_sum(0));
    assert!(!empty_solver.subset_sum(1));

    // Single element
    let single_solver = SubsetSumStPerLit![5];
    assert!(single_solver.subset_sum(0));
    assert!(single_solver.subset_sum(5));
    assert!(!single_solver.subset_sum(3));

    // Negative target (defensive programming)
    let solver = SubsetSumStPerLit![1, 2, 3];
    assert!(!solver.subset_sum(-1));
}

#[test]
fn test_subset_sum_large_target() {
    let solver = SubsetSumStPerLit![1, 2, 3, 4, 5];
    assert!(solver.subset_sum(15)); // All elements: 1+2+3+4+5=15
    assert!(!solver.subset_sum(16)); // Impossible
    assert!(solver.subset_sum(14)); // 2+3+4+5=14
}

#[test]
fn test_subset_sum_display() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    let display_str = format!("{solver}");
    assert!(display_str.contains("SubsetSumStPer"));
    assert!(display_str.contains("multiset"));
    assert!(display_str.contains("memo_entries"));
}

#[test]
fn test_subset_sum_iterator() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    let elements: Vec<i32> = solver.into_iter().collect();
    assert_eq!(elements, vec![1, 2, 3]);
}

#[test]
fn test_subset_sum_accessors() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    assert_eq!(solver.multiset().length(), 3);
    assert_eq!(solver.memo_size(), 0); // No computation done yet
}

#[test]
fn test_subset_sum_eph_mutation() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];

    // Initial state
    assert!(solver.subset_sum(6)); // 1+2+3=6

    // Mutate and test
    solver.set(2, 5); // Change 3 to 5
    assert!(solver.subset_sum(6)); // 1+5=6
    assert!(solver.subset_sum(8)); // 1+2+5=8
    assert!(!solver.subset_sum(4)); // No subset of [1,2,5] sums to 4

    // Clear memo and test
    solver.clear_memo();
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_subset_sum_mt_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let solver = Arc::new(SubsetSumMtPerLit![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    // Spawn multiple threads testing the same solver
    for target in [6, 7, 8, 9, 10] {
        let solver_clone = Arc::clone(&solver);
        let handle = thread::spawn(move || solver_clone.subset_sum(target));
        handles.push(handle);
    }

    // Collect results
    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify expected results
    assert_eq!(results, vec![true, true, true, true, true]); // All should be achievable
}

#[test]
fn test_subset_sum_memoization() {
    let solver = SubsetSumStPerLit![1, 2, 3, 4];

    // First call should populate memo
    let result1 = solver.subset_sum(5);
    assert!(result1);

    // Second call should use memoized results (same solver instance)
    let result2 = solver.subset_sum(5);
    assert!(result2);
}

#[test]
fn test_subset_sum_equality() {
    let solver1 = SubsetSumStPerLit![1, 2, 3];
    let solver2 = SubsetSumStPerLit![1, 2, 3];
    let solver3 = SubsetSumStPerLit![1, 2, 4];

    assert_eq!(solver1, solver2);
    assert_ne!(solver1, solver3);
}
