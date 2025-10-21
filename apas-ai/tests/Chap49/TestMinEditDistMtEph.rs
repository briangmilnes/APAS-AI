//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistMtEph.

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap49::MinEditDistMtEph::MinEditDistMtEph::*;
use apas_ai::MinEditDistMtEphLit;

#[test]
fn test_min_edit_distance_mt_eph_basic() {
    let mut solver = MinEditDistMtEphLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);
    assert_eq!(solver.min_edit_distance(), 2);

    // Test ephemeral mutation
    solver.set_source(2, 'D');
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_mt_eph_mutation() {
    let mut solver = MinEditDistMtEphLit!(source: ['X', 'Y'], target: ['A', 'B', 'C']);
    assert_eq!(solver.min_edit_distance(), 5);

    solver.set_source(0, 'A');
    solver.set_source(1, 'B');
    assert!(solver.min_edit_distance() < 5);
}

#[test]
fn test_min_edit_distance_mt_eph_empty() {
    let mut solver: MinEditDistMtEphS<char> = MinEditDistMtEphLit!(source: [], target: []);
    assert_eq!(solver.min_edit_distance(), 0);
}
