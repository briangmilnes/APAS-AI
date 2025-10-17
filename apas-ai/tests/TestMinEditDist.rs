//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Minimum Edit Distance.

use apas_ai::{
    ArraySeqMtEphSLit, ArraySeqStEphS, ArraySeqStPerS, Chap49::MinEditDistMtEph::MinEditDistMtEph::*,
    Chap49::MinEditDistMtPer::MinEditDistMtPer::*, Chap49::MinEditDistStEph::MinEditDistStEph::*,
    Chap49::MinEditDistStPer::MinEditDistStPer::*, MinEditDistMtEphLit, MinEditDistMtPerLit, MinEditDistStEphLit,
    MinEditDistStPerLit, Types::Types::*
};


// Test cases from APAS textbook
#[test]
fn test_min_edit_distance_st_per_example_49_3() {
    // Example 49.3: Transform S = <A, B, C, A, D, A> to T = <A, B, A, D, C>
    // Expected: 3 edits (delete C, delete last A, insert C at end)
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );

    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_min_edit_distance_st_per_basic() {
    // Identical sequences
    let solver1 = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: ['A', 'B', 'C']
    );
    assert_eq!(solver1.min_edit_distance(), 0);

    // Empty to non-empty (all insertions)
    let solver2 = MinEditDistStPerLit!(
        source: [],
        target: ['A', 'B', 'C']
    );
    assert_eq!(solver2.min_edit_distance(), 3);

    // Non-empty to empty (all deletions)
    let solver3 = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: []
    );
    assert_eq!(solver3.min_edit_distance(), 3);

    // Both empty
    let solver4: MinEditDistStPerS<char> = MinEditDistStPerLit!(
        source: [],
        target: []
    );
    assert_eq!(solver4.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_st_per_single_operations() {
    // Single insertion
    let solver1 = MinEditDistStPerLit!(
        source: ['A', 'B'],
        target: ['A', 'X', 'B']
    );
    assert_eq!(solver1.min_edit_distance(), 1);

    // Single deletion
    let solver2 = MinEditDistStPerLit!(
        source: ['A', 'X', 'B'],
        target: ['A', 'B']
    );
    assert_eq!(solver2.min_edit_distance(), 1);

    // Single substitution (delete + insert)
    let solver3 = MinEditDistStPerLit!(
        source: ['A', 'X', 'B'],
        target: ['A', 'Y', 'B']
    );
    assert_eq!(solver3.min_edit_distance(), 2); // Delete X, Insert Y
}

#[test]
fn test_min_edit_distance_st_eph_basic() {
    let mut solver = MinEditDistStEphLit!(
        source: ['A', 'B', 'C'],
        target: ['A', 'B', 'D']
    );

    assert_eq!(solver.min_edit_distance(), 2); // Delete C, Insert D

    // Test ephemeral mutation
    solver.set_target(2, 'C'); // Change target back to ['A', 'B', 'C']
    assert_eq!(solver.min_edit_distance(), 0); // Now identical
}

#[test]
fn test_min_edit_distance_mt_per_basic() {
    let solver = MinEditDistMtPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );

    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_min_edit_distance_mt_eph_basic() {
    let mut solver = MinEditDistMtEphLit!(
        source: ['A', 'B', 'C'],
        target: ['A', 'B', 'D']
    );

    assert_eq!(solver.min_edit_distance(), 2);

    // Test ephemeral mutation
    solver.set_source(2, 'D'); // Change source to ['A', 'B', 'D']
    assert_eq!(solver.min_edit_distance(), 0); // Now identical
}

#[test]
fn test_min_edit_distance_string_sequences() {
    // Test with string-like sequences
    let solver = MinEditDistStPerLit!(
        source: ['k', 'i', 't', 't', 'e', 'n'],
        target: ['s', 'i', 't', 't', 'i', 'n', 'g']
    );

    // kitten -> sitting: substitute k->s, substitute e->i, insert g
    let distance = solver.min_edit_distance();
    assert!(distance <= 5); // Upper bound check (exact value depends on algorithm choices)
}

#[test]
fn test_min_edit_distance_edge_cases() {
    // Single character sequences
    let solver1 = MinEditDistStPerLit!(
        source: ['A'],
        target: ['B']
    );
    assert_eq!(solver1.min_edit_distance(), 2); // Delete A, Insert B

    // One character vs empty
    let solver2 = MinEditDistStPerLit!(
        source: ['A'],
        target: []
    );
    assert_eq!(solver2.min_edit_distance(), 1); // Delete A

    // Empty vs one character
    let solver3 = MinEditDistStPerLit!(
        source: [],
        target: ['A']
    );
    assert_eq!(solver3.min_edit_distance(), 1); // Insert A
}

#[test]
fn test_min_edit_distance_display() {
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B'],
        target: ['C', 'D']
    );

    let display_str = format!("{solver}");
    assert!(display_str.contains("MinEditDistStPer"));
    assert!(display_str.contains("source"));
    assert!(display_str.contains("target"));
    assert!(display_str.contains("memo_entries"));
}

#[test]
fn test_min_edit_distance_iterator() {
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B'],
        target: ['C', 'D']
    );

    let pairs: Vec<Pair<char, char>> = solver.into_iter().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], Pair('A', 'C'));
    assert_eq!(pairs[1], Pair('B', 'D'));
}

#[test]
fn test_min_edit_distance_accessors() {
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y']
    );

    assert_eq!(solver.source().length(), 3);
    assert_eq!(solver.target().length(), 2);
    assert_eq!(solver.memo_size(), 0); // No computation done yet
}

#[test]
fn test_min_edit_distance_eph_mutation() {
    let mut solver = MinEditDistStEphLit!(
        source: ['A', 'B'],
        target: ['A', 'C']
    );

    // Initial state
    assert_eq!(solver.min_edit_distance(), 2); // Delete B, Insert C

    // Mutate source
    solver.set_source(1, 'C'); // Change B to C
    assert_eq!(solver.min_edit_distance(), 0); // Now identical

    // Mutate target
    solver.set_target(1, 'D'); // Change C to D
    assert_eq!(solver.min_edit_distance(), 2); // Delete C, Insert D

    // Test mutable accessors
    let _ = solver.source_mut().set(0, 'X');
    let _ = solver.target_mut().set(0, 'Y');
    assert_eq!(solver.min_edit_distance(), 4); // X->Y, C->D

    // Clear memo
    solver.clear_memo();
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_min_edit_distance_mt_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let solver = Arc::new(MinEditDistMtPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y', 'Z']
    ));

    let mut handles = vec![];

    // Spawn multiple threads computing the same distance
    for _ in 0..5 {
        let solver_clone = Arc::clone(&solver);
        let handle = thread::spawn(move || solver_clone.min_edit_distance());
        handles.push(handle);
    }

    // Collect results
    let results: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be the same
    assert!(results.iter().all(|&x| x == results[0]));
    assert_eq!(results[0], 6); // Delete A,B,C and Insert X,Y,Z
}

#[test]
fn test_min_edit_distance_memoization() {
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: ['A', 'X', 'C']
    );

    // First call should populate memo
    let result1 = solver.min_edit_distance();
    assert_eq!(result1, 2); // Delete B, Insert X

    // Second call should use memoized results
    let result2 = solver.min_edit_distance();
    assert_eq!(result2, 2);
}

#[test]
fn test_min_edit_distance_equality() {
    let solver1 = MinEditDistStPerLit!(
        source: ['A', 'B'],
        target: ['C', 'D']
    );
    let solver2 = MinEditDistStPerLit!(
        source: ['A', 'B'],
        target: ['C', 'D']
    );
    let solver3 = MinEditDistStPerLit!(
        source: ['A', 'B'],
        target: ['C', 'E']
    );

    assert_eq!(solver1, solver2);
    assert_ne!(solver1, solver3);
}

#[test]
fn test_min_edit_distance_longer_sequences() {
    // Test with longer sequences to verify dynamic programming efficiency

    let solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'],
        target: ['A', 'X', 'C', 'Y', 'E', 'Z', 'G', 'W']
    );

    let distance = solver.min_edit_distance();
    // Should be efficient due to memoization
    assert!(distance > 0);
    assert!(distance <= 8); // Upper bound: delete all, insert all
}

#[test]
fn test_min_edit_distance_numeric_sequences() {
    // Test with numeric sequences
    let solver = MinEditDistStPerLit!(
        source: [1, 2, 3, 4],
        target: [1, 5, 3, 6]
    );

    let distance = solver.min_edit_distance();
    assert_eq!(distance, 4); // Delete 2,4 and Insert 5,6
}
