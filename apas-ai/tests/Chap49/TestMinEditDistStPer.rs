//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistStPer - Minimum Edit Distance (persistent, single-threaded).

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap49::MinEditDistStPer::MinEditDistStPer::*;

#[test]
fn test_new_empty() {
    let med: MinEditDistStPerS<char> = MinEditDistStPerTrait::new();
    assert_eq!(med.source().length(), 0);
    assert_eq!(med.target().length(), 0);
}

#[test]
fn test_from_sequences() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStPerS::from_vec(vec!['d', 'e', 'f']);

    let med = MinEditDistStPerS::from_sequences(source.clone(), target.clone());

    assert_eq!(med.source().length(), 3);
    assert_eq!(med.target().length(), 3);
}

#[test]
fn test_identical_strings() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStPerS::from_vec(vec!['a', 'b', 'c']);

    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 0);
}

#[test]
fn test_empty_to_nonempty() {
    let source = ArraySeqStPerS::from_vec(vec![]);
    let target = ArraySeqStPerS::from_vec(vec!['a', 'b', 'c']);

    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 3);
}

#[test]
fn test_single_substitution() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStPerS::from_vec(vec!['a', 'x', 'c']);

    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 2);
}

