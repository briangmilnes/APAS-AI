//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistMtPer - Minimum Edit Distance (persistent, multithreaded).

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Chap49::MinEditDistMtPer::MinEditDistMtPer::*;

#[test]
fn test_new_empty() {
    let med: MinEditDistMtPerS<char> = MinEditDistMtPerTrait::new();
    assert_eq!(med.source().length(), 0);
    assert_eq!(med.target().length(), 0);
}

#[test]
fn test_from_sequences() {
    let source = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtPerS::from_vec(vec!['d', 'e', 'f']);

    let med = MinEditDistMtPerS::from_sequences(source.clone(), target.clone());

    assert_eq!(med.source().length(), 3);
    assert_eq!(med.target().length(), 3);
}

#[test]
fn test_identical_strings() {
    let source = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);

    let med = MinEditDistMtPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 0);
}

#[test]
fn test_empty_to_nonempty() {
    let source = ArraySeqMtPerS::from_vec(vec![]);
    let target = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);

    let med = MinEditDistMtPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 3);
}

#[test]
fn test_single_substitution() {
    let source = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtPerS::from_vec(vec!['a', 'x', 'c']);

    let med = MinEditDistMtPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 2);
}

