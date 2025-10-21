//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistStPer.

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap49::MinEditDistStPer::MinEditDistStPer::*;
use apas_ai::MinEditDistStPerLit;

#[test]
fn test_min_edit_distance_st_per_example_49_3() {
    // Example 49.3: Transform S = <A, B, C, A, D, A> to T = <A, B, A, D, C>
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );
    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_min_edit_distance_st_per_basic() {
    let solver1 = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'C']);
    assert_eq!(solver1.min_edit_distance(), 0);

    let solver2 = MinEditDistStPerLit!(source: [], target: ['A', 'B', 'C']);
    assert_eq!(solver2.min_edit_distance(), 3);

    let solver3 = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: []);
    assert_eq!(solver3.min_edit_distance(), 3);

    let solver4: MinEditDistStPerS<char> = MinEditDistStPerLit!(source: [], target: []);
    assert_eq!(solver4.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_st_per_single_operations() {
    let solver1 = MinEditDistStPerLit!(source: ['A', 'B'], target: ['A', 'X', 'B']);
    assert_eq!(solver1.min_edit_distance(), 1);

    let solver2 = MinEditDistStPerLit!(source: ['A', 'X', 'B'], target: ['A', 'B']);
    assert_eq!(solver2.min_edit_distance(), 1);

    let solver3 = MinEditDistStPerLit!(source: ['A', 'X', 'B'], target: ['A', 'Y', 'B']);
    assert_eq!(solver3.min_edit_distance(), 2);
}
