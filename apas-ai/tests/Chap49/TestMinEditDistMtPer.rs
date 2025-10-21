//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistMtPer.

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Chap49::MinEditDistMtPer::MinEditDistMtPer::*;
use apas_ai::MinEditDistMtPerLit;

#[test]
fn test_min_edit_distance_mt_per_basic() {
    let solver = MinEditDistMtPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );
    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_min_edit_distance_mt_per_empty() {
    let solver1 = MinEditDistMtPerLit!(source: [], target: ['A', 'B']);
    assert_eq!(solver1.min_edit_distance(), 2);

    let solver2: MinEditDistMtPerS<char> = MinEditDistMtPerLit!(source: [], target: []);
    assert_eq!(solver2.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_mt_per_identical() {
    let solver = MinEditDistMtPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'C']);
    assert_eq!(solver.min_edit_distance(), 0);
}
