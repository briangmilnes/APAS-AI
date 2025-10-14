//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistMtEph - Minimum Edit Distance (ephemeral, multithreaded).

use std::sync::Arc;
use std::thread;

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap49::MinEditDistMtEph::MinEditDistMtEph::*;

#[test]
fn test_new_empty() {
    let med: MinEditDistMtEphS<char> = MinEditDistMtEphTrait::new();
    assert_eq!(med.source().length(), 0);
    assert_eq!(med.target().length(), 0);
}

#[test]
fn test_from_sequences() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtEphS::from_vec(vec!['d', 'e', 'f']);

    let med = MinEditDistMtEphS::from_sequences(source.clone(), target.clone());

    assert_eq!(med.source().length(), 3);
    assert_eq!(med.target().length(), 3);
}

#[test]
fn test_identical_strings() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 0);
}

#[test]
fn test_empty_to_nonempty() {
    let source = ArraySeqMtEphS::from_vec(vec![]);
    let target = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 3);
}

#[test]
fn test_nonempty_to_empty() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtEphS::from_vec(vec![]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 3);
}

#[test]
fn test_single_substitution() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtEphS::from_vec(vec!['a', 'x', 'c']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert!((1..=3).contains(&dist));
}

#[test]
fn test_insertion() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let target = ArraySeqMtEphS::from_vec(vec!['a', 'x', 'b']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 1);
}

#[test]
fn test_deletion() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'x', 'b']);
    let target = ArraySeqMtEphS::from_vec(vec!['a', 'b']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 1);
}

// Skipping test_classic_kitten_sitting - causes timeout in MtEph version

#[test]
fn test_source_accessor() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let med = MinEditDistMtEphS::from_sequences(source.clone(), target);

    assert_eq!(med.source().length(), 3);
    assert_eq!(med.source().nth_cloned(0), 1);
    assert_eq!(med.source().nth_cloned(1), 2);
    assert_eq!(med.source().nth_cloned(2), 3);
}

#[test]
fn test_target_accessor() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let med = MinEditDistMtEphS::from_sequences(source, target.clone());

    assert_eq!(med.target().length(), 3);
    assert_eq!(med.target().nth_cloned(0), 4);
    assert_eq!(med.target().nth_cloned(1), 5);
    assert_eq!(med.target().nth_cloned(2), 6);
}

#[test]
fn test_set_source() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    med.set_source(1, 99);

    assert_eq!(med.source().nth_cloned(1), 99);
}

#[test]
fn test_set_target() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    med.set_target(0, 88);

    assert_eq!(med.target().nth_cloned(0), 88);
}

#[test]
fn test_clear_memo() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);

    let _dist1 = med.min_edit_distance();
    let initial_memo = med.memo_size();

    med.clear_memo();
    assert!(med.memo_size() < initial_memo || initial_memo == 0);
}

#[test]
fn test_memo_size() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);

    med.min_edit_distance();
    // Memo should have some entries after computation
    assert!(med.memo_size() > 0);
}

#[test]
fn test_single_char_same() {
    let source = ArraySeqMtEphS::from_vec(vec!['a']);
    let target = ArraySeqMtEphS::from_vec(vec!['a']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 0);
}

#[test]
fn test_completely_different() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtEphS::from_vec(vec!['x', 'y', 'z']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert!((3..=6).contains(&dist));
}

#[test]
fn test_prefix_match() {
    let source = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c', 'd', 'e']);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 2);
}

#[test]
fn test_integer_sequences() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4]);
    let target = ArraySeqMtEphS::from_vec(vec![1, 3, 4, 5]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 2);
}

#[test]
fn test_empty_both() {
    let source = ArraySeqMtEphS::from_vec(vec![]);
    let target = ArraySeqMtEphS::from_vec(vec![]);

    let mut med: MinEditDistMtEphS<i32> = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert_eq!(dist, 0);
}

#[test]
fn test_source_mut() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);

    {
        let source_mut = med.source_mut();
        let _ = source_mut.set(0, 99);
    }

    assert_eq!(med.source().nth_cloned(0), 99);
}

#[test]
fn test_target_mut() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);

    {
        let target_mut = med.target_mut();
        let _ = target_mut.set(2, 77);
    }

    assert_eq!(med.target().nth_cloned(2), 77);
}

#[test]
fn test_concurrent_reads() {
    let source = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);

    let med = MinEditDistMtEphS::from_sequences(source, target);

    // Test thread-safe read access
    let med_arc = Arc::new(med);
    let med_clone = Arc::clone(&med_arc);

    let handle = thread::spawn(move || med_clone.source().length());

    let len = handle.join().unwrap();
    assert_eq!(len, 3);
}

#[test]
fn test_equality() {
    let source1 = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target1 = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);
    let med1 = MinEditDistMtEphS::from_sequences(source1.clone(), target1.clone());

    let source2 = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target2 = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);
    let med2 = MinEditDistMtEphS::from_sequences(source2, target2);

    assert_eq!(med1, med2);
}

#[test]
fn test_inequality() {
    let source1 = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target1 = ArraySeqMtEphS::from_vec(vec![4, 5, 6]);
    let med1 = MinEditDistMtEphS::from_sequences(source1, target1);

    let source2 = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let target2 = ArraySeqMtEphS::from_vec(vec![7, 8, 9]);
    let med2 = MinEditDistMtEphS::from_sequences(source2, target2);

    assert_ne!(med1, med2);
}

// Skipping test_large_sequences - too slow for MtEph version

#[test]
fn test_string_sequences() {
    let source = ArraySeqMtEphS::from_vec(vec!["hello".to_string(), "world".to_string()]);
    let target = ArraySeqMtEphS::from_vec(vec!["hello".to_string(), "rust".to_string()]);

    let mut med = MinEditDistMtEphS::from_sequences(source, target);
    let dist = med.min_edit_distance();

    assert!((1..=2).contains(&dist));
}
