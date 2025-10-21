//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistStEph.

use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap49::MinEditDistStEph::MinEditDistStEph::*;
use apas_ai::MinEditDistStEphLit;

#[test]
fn test_min_edit_distance_st_eph_basic() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);
    assert_eq!(solver.min_edit_distance(), 2);

    // Test ephemeral mutation
    solver.set_target(2, 'C');
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_eph_mutation() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: ['X', 'Y', 'Z']);
    assert_eq!(solver.min_edit_distance(), 6);

    // Mutate source to match target
    solver.set_source(0, 'X');
    solver.set_source(1, 'Y');
    solver.set_source(2, 'Z');
    assert_eq!(solver.min_edit_distance(), 0);

    // Mutate target back
    solver.set_target(0, 'A');
    solver.set_target(1, 'B');
    solver.set_target(2, 'C');
    assert_eq!(solver.min_edit_distance(), 6);
}
