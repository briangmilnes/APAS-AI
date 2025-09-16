//! Tests for ArraySeqMtPerChap19 multithreaded algorithms.

pub mod TestArraySeqMtPerChap19 {
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::ArraySeqMtPerChap19::ArraySeqMtPerChap19::*;
use apas_ai::ArraySeqMtPer; // macro import
use std::sync::Mutex;

#[test]
fn test_inject_basic() {
    let values = ArraySeqMtPer![0, 1, 2, 3, 4, 5];
    let changes = ArraySeqMtPer![Pair(2, 99), Pair(4, 88)];
    let result = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::inject(&values, &changes);
    
    assert_eq!(result.length(), 6);
    // inject should apply first occurrence of each index
    assert_eq!(*result.nth(0), 0);
    assert_eq!(*result.nth(1), 1);
    assert_eq!(*result.nth(2), 99);
    assert_eq!(*result.nth(3), 3);
    assert_eq!(*result.nth(4), 88);
    assert_eq!(*result.nth(5), 5);
}

#[test]
fn test_inject_conflicting_updates() {
    let values = ArraySeqMtPer![0, 1, 2, 3, 4, 5];
    let changes = ArraySeqMtPer![Pair(2, 99), Pair(2, 77), Pair(4, 88)];
    let result = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::inject(&values, &changes);
    
    assert_eq!(result.length(), 6);
    // inject takes FIRST update for each index (99 for index 2)
    assert_eq!(*result.nth(0), 0);
    assert_eq!(*result.nth(1), 1);
    assert_eq!(*result.nth(2), 99); // First update wins
    assert_eq!(*result.nth(3), 3);
    assert_eq!(*result.nth(4), 88);
    assert_eq!(*result.nth(5), 5);
}

#[test]
fn test_inject_out_of_bounds() {
    let values = ArraySeqMtPer![0, 1, 2];
    let changes = ArraySeqMtPer![Pair(1, 99), Pair(5, 77)]; // index 5 is out of bounds
    let result = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::inject(&values, &changes);
    
    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 0);
    assert_eq!(*result.nth(1), 99); // Valid update applied
    assert_eq!(*result.nth(2), 2);  // Out of bounds update ignored
}

#[test]
fn test_inject_empty_changes() {
    let values = ArraySeqMtPer![1, 2, 3];
    let changes: ArrayMtPerS<Pair<N, N>> = ArraySeqMtPer![];
    let result = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::inject(&values, &changes);
    
    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 1);
    assert_eq!(*result.nth(1), 2);
    assert_eq!(*result.nth(2), 3);
}

#[test]
fn test_inject_empty_values() {
    let values: ArrayMtPerS<N> = ArraySeqMtPer![];
    let changes = ArraySeqMtPer![Pair(0, 99)];
    let result = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::inject(&values, &changes);
    
    assert_eq!(result.length(), 0); // No values to update
}

// TODO: AtomicWriteLowestChangeNumberWins tests require complex setup
// The function signature expects values_with_change_number and changes arrays
// to have specific relationships that need more investigation

// Migrated from tests/11_TestArraySeqStPerChap19.rs - this was the commented MT code
#[test]
#[ignore] // TODO: AtomicWriteLowestChangeNumberWins is currently a stub implementation
fn test_atomic_write_migrated_from_st_test() {
    let values = ArraySeqMtPer![0, 1, 2, 3, 4, 5];
    let changes = ArraySeqMtPer![Pair(2, 99), Pair(2, 7), Pair(4, 20)];
    let n = values.length();
    
    // Create values with change numbers initialized to n
    let with_num = ArraySeqMtPer![
        Mutex::new(Pair(*values.nth(0), n)),
        Mutex::new(Pair(*values.nth(1), n)),
        Mutex::new(Pair(*values.nth(2), n)),
        Mutex::new(Pair(*values.nth(3), n)),
        Mutex::new(Pair(*values.nth(4), n)),
        Mutex::new(Pair(*values.nth(5), n))
    ];
    
    // Apply atomic writes with different change numbers
    <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 1);
    <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 0);
    
    // Check that the value at index 2 was updated (change number 0 should win over 1)
    let guard = with_num.nth(2).lock().unwrap();
    assert_eq!(guard.0, 99); // Value should be from the changes with change number 0
}

#[test]
fn test_inject_string_values() {
    let values = ArraySeqMtPer!["hello", "world", "test"];
    let changes = ArraySeqMtPer![Pair(1, "rust"), Pair(0, "hi")];
    let result = <ArrayMtPerS<&str> as ArraySeqMtPerChap19Trait<&str>>::inject(&values, &changes);
    
    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), "hi");
    assert_eq!(*result.nth(1), "rust");
    assert_eq!(*result.nth(2), "test");
}

}
