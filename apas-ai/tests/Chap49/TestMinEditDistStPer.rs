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

#[test]
fn test_nonempty_to_empty() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'b', 'c']);
    let target = ArraySeqStPerS::from_vec(vec![]);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert_eq!(dist, 3);
}

#[test]
fn test_insertion() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'b']);
    let target = ArraySeqStPerS::from_vec(vec!['a', 'x', 'b']);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert_eq!(dist, 1);
}

#[test]
fn test_deletion() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'x', 'b']);
    let target = ArraySeqStPerS::from_vec(vec!['a', 'b']);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert_eq!(dist, 1);
}

#[test]
fn test_source_accessor() {
    let source = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStPerS::from_vec(vec![4, 5, 6]);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    
    assert_eq!(med.source().length(), 3);
    assert_eq!(*med.source().nth(0), 1);
}

#[test]
fn test_target_accessor() {
    let source = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStPerS::from_vec(vec![4, 5, 6]);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    
    assert_eq!(med.target().length(), 3);
    assert_eq!(*med.target().nth(0), 4);
}

#[test]
fn test_memo_size() {
    let source = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStPerS::from_vec(vec![4, 5, 6]);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    assert_eq!(med.memo_size(), 0);
    
    let _ = med.min_edit_distance();
    // Memo not exposed in persistent version
}

#[test]
fn test_clone() {
    let source = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let target = ArraySeqStPerS::from_vec(vec![4, 5, 6]);
    let med = MinEditDistStPerS::from_sequences(source, target);
    
    let cloned = med.clone();
    assert_eq!(cloned.source().length(), 3);
    assert_eq!(cloned.target().length(), 3);
}

#[test]
fn test_debug() {
    let source = ArraySeqStPerS::from_vec(vec![1, 2]);
    let target = ArraySeqStPerS::from_vec(vec![3, 4]);
    let med = MinEditDistStPerS::from_sequences(source, target);
    
    let debug = format!("{:?}", med);
    assert!(debug.contains("MinEditDistStPerS"));
}

#[test]
fn test_display() {
    let source = ArraySeqStPerS::from_vec(vec!['a', 'b']);
    let target = ArraySeqStPerS::from_vec(vec!['c', 'd']);
    let med = MinEditDistStPerS::from_sequences(source, target);
    
    let display = format!("{}", med);
    assert!(display.contains("MinEditDist"));
}

#[test]
fn test_equality() {
    let source1 = ArraySeqStPerS::from_vec(vec![1, 2]);
    let target1 = ArraySeqStPerS::from_vec(vec![3, 4]);
    let med1 = MinEditDistStPerS::from_sequences(source1, target1);
    
    let source2 = ArraySeqStPerS::from_vec(vec![1, 2]);
    let target2 = ArraySeqStPerS::from_vec(vec![3, 4]);
    let med2 = MinEditDistStPerS::from_sequences(source2, target2);
    
    assert_eq!(med1, med2);
}

#[test]
fn test_large_sequences() {
    let source = ArraySeqStPerS::from_vec((0..20).collect::<Vec<_>>());
    let target = ArraySeqStPerS::from_vec((10..30).collect::<Vec<_>>());
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert!(dist > 0);
}

#[test]
fn test_empty_both() {
    let source = ArraySeqStPerS::from_vec(vec![]);
    let target = ArraySeqStPerS::from_vec(vec![]);
    
    let med: MinEditDistStPerS<i32> = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert_eq!(dist, 0);
}

#[test]
fn test_single_char_strings() {
    let source = ArraySeqStPerS::from_vec(vec!['a']);
    let target = ArraySeqStPerS::from_vec(vec!['b']);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert!(dist > 0);
}

#[test]
fn test_single_char_same() {
    let source = ArraySeqStPerS::from_vec(vec!['a']);
    let target = ArraySeqStPerS::from_vec(vec!['a']);
    
    let med = MinEditDistStPerS::from_sequences(source, target);
    let dist = med.min_edit_distance();
    
    assert_eq!(dist, 0);
}
