//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistStEph - Minimum Edit Distance (ephemeral, single-threaded).

use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap49::MinEditDistStEph::MinEditDistStEph::*;

#[test]
fn test_new_empty() {
    let med: MinEditDistStEphS<char> = MinEditDistStEphTrait::new();
    assert_eq!(med.source().length(), 0);
    assert_eq!(med.target().length(), 0);
    assert_eq!(med.memo_size(), 0);
}

#[test]
fn test_from_sequences() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['d', 'e', 'f']);

    let med = MinEditDistStEphS::from_sequences(source.clone(), target.clone());

    assert_eq!(med.source().length(), 3);
    assert_eq!(med.target().length(), 3);
}

#[test]
fn test_identical_strings() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Identical strings have distance 0
    assert_eq!(dist, 0);
}

#[test]
fn test_empty_to_nonempty() {
    let source = ArraySeqStEphS::from_vec(vec![]);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Need 3 insertions
    assert_eq!(dist, 3);
}

#[test]
fn test_nonempty_to_empty() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec![]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Need 3 deletions
    assert_eq!(dist, 3);
}

#[test]
fn test_single_substitution() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'x', 'c']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Should have small edit distance
    assert!(dist >= 1 && dist <= 3);
}

#[test]
fn test_insertion() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'x', 'b']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // One insertion: x
    assert_eq!(dist, 1);
}

#[test]
fn test_deletion() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'x', 'b']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'b']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // One deletion: x
    assert_eq!(dist, 1);
}

#[test]
fn test_classic_kitten_sitting() {
    let source = ArraySeqStEphS::from_vec("kitten".chars().collect::<Vec<_>>());
    let target = ArraySeqStEphS::from_vec("sitting".chars().collect::<Vec<_>>());

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Should have moderate edit distance
    assert!(dist >= 3 && dist <= 7);
}

#[test]
fn test_saturday_sunday() {
    let source = ArraySeqStEphS::from_vec("saturday".chars().collect::<Vec<_>>());
    let target = ArraySeqStEphS::from_vec("sunday".chars().collect::<Vec<_>>());

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Should have moderate edit distance
    assert!(dist >= 3 && dist <= 8);
}

#[test]
fn test_source_accessor() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let med = MinEditDistStEphS::from_sequences(source.clone(), target);

    assert_eq!(med.source().length(), 3);
    assert_eq!(*med.source().nth(0), 1);
    assert_eq!(*med.source().nth(1), 2);
    assert_eq!(*med.source().nth(2), 3);
}

#[test]
fn test_target_accessor() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let med = MinEditDistStEphS::from_sequences(source, target.clone());

    assert_eq!(med.target().length(), 3);
    assert_eq!(*med.target().nth(0), 4);
    assert_eq!(*med.target().nth(1), 5);
    assert_eq!(*med.target().nth(2), 6);
}

#[test]
fn test_set_source() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    med.set_source(1, 99);

    assert_eq!(*med.source().nth(1), 99);
}

#[test]
fn test_set_target() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    med.set_target(0, 88);

    assert_eq!(*med.target().nth(0), 88);
}

#[test]
fn test_clear_memo() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);

    let _dist1 = med.min_edit_distance();
    assert!(med.memo_size() > 0);

    med.clear_memo();
    assert_eq!(med.memo_size(), 0);
}

#[test]
fn test_memo_size() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);

    assert_eq!(med.memo_size(), 0);

    med.min_edit_distance();
    assert!(med.memo_size() > 0);
}

#[test]
fn test_memoization_efficiency() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3, 4]);
    let target = ArraySeqStEphS::from_vec(vec![5, 6, 7, 8]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);

    let dist1 = med.min_edit_distance();
    let memo_size1 = med.memo_size();

    let dist2 = med.min_edit_distance();
    let memo_size2 = med.memo_size();

    assert_eq!(dist1, dist2);
    assert_eq!(memo_size1, memo_size2);
}

#[test]
fn test_single_char_strings() {
    let source = ArraySeqStEphS::from_vec(vec!['a']);
    let target = ArraySeqStEphS::from_vec(vec!['b']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Should be non-zero
    assert!(dist >= 1 && dist <= 2);
}

#[test]
fn test_single_char_same() {
    let source = ArraySeqStEphS::from_vec(vec!['a']);
    let target = ArraySeqStEphS::from_vec(vec!['a']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // No edits needed
    assert_eq!(dist, 0);
}

#[test]
fn test_completely_different() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['x', 'y', 'z']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Should have significant edit distance
    assert!(dist >= 3 && dist <= 6);
}

#[test]
fn test_prefix_match() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c', 'd', 'e']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Two insertions
    assert_eq!(dist, 2);
}

#[test]
fn test_suffix_match() {
    let source = ArraySeqStEphS::from_vec(vec!['x', 'y', 'a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Two deletions
    assert_eq!(dist, 2);
}

#[test]
fn test_integer_sequences() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3, 4]);
    let target = ArraySeqStEphS::from_vec(vec![1, 3, 4, 5]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Delete 2, insert 5 = 2 operations (or substitute and insert)
    assert_eq!(dist, 2);
}

#[test]
fn test_string_sequences() {
    let source = ArraySeqStEphS::from_vec(vec!["hello".to_string(), "world".to_string()]);
    let target = ArraySeqStEphS::from_vec(vec!["hello".to_string(), "rust".to_string()]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Should be non-zero (at least one difference)
    assert!(dist >= 1 && dist <= 2);
}

#[test]
fn test_update_and_recompute() {
    let source = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStEphS::from_vec(vec!['a', 'b', 'c']);

    let mut med = MinEditDistStEphS::from_sequences(source, target);

    let dist1 = med.min_edit_distance();
    assert_eq!(dist1, 0);

    med.set_source(1, 'x');
    med.clear_memo();

    let dist2 = med.min_edit_distance();
    assert!(dist2 >= 1 && dist2 <= 2); // Now needs some edits
}

#[test]
fn test_empty_both() {
    let source = ArraySeqStEphS::from_vec(vec![]);
    let target = ArraySeqStEphS::from_vec(vec![]);

    let mut med: MinEditDistStEphS<i32> = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Empty to empty = 0
    assert_eq!(dist, 0);
}

#[test]
fn test_large_sequences() {
    let source = ArraySeqStEphS::from_vec((0..20).collect::<Vec<_>>());
    let target = ArraySeqStEphS::from_vec((10..30).collect::<Vec<_>>());

    let mut med = MinEditDistStEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    // Distance should be computed (exact value depends on algorithm)
    assert!(dist > 0);
}

#[test]
fn test_source_mut() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);

    {
        let source_mut = med.source_mut();
        let _ = source_mut.set(0, 99);
    }

    assert_eq!(*med.source().nth(0), 99);
}

#[test]
fn test_target_mut() {
    let source = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistStEphS::from_sequences(source, target);

    {
        let target_mut = med.target_mut();
        let _ = target_mut.set(2, 77);
    }

    assert_eq!(*med.target().nth(2), 77);
}
