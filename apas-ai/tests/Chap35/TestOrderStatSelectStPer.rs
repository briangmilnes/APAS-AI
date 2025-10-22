//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap35::OrderStatSelectStPer::OrderStatSelectStPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_empty() {
    let a: ArraySeqStPerS<i32> = ArraySeqStPerSLit![];
    assert_eq!(a.select(0), None);
}

#[test]
fn test_single() {
    let a = ArraySeqStPerSLit![42];
    assert_eq!(a.select(0), Some(42));
    assert_eq!(a.select(1), None);
}

#[test]
fn test_small() {
    let a = ArraySeqStPerSLit![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted = ArraySeqStPerSLit![1, 1, 2, 3, 4, 5, 6, 9];

    for k in 0..sorted.length() {
        assert_eq!(a.select(k), Some(*sorted.nth(k)), "Failed at k={k}");
    }
}

#[test]
fn test_already_sorted() {
    let a = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    for k in 0..5 {
        assert_eq!(a.select(k), Some(k as i32 + 1));
    }
}

#[test]
fn test_reverse_sorted() {
    let a = ArraySeqStPerSLit![5, 4, 3, 2, 1];
    for k in 0..5 {
        assert_eq!(a.select(k), Some(k as i32 + 1));
    }
}

#[test]
fn test_duplicates() {
    let a = ArraySeqStPerSLit![3, 3, 3, 3, 3];
    for k in 0..5 {
        assert_eq!(a.select(k), Some(3));
    }
}

#[test]
fn test_negative() {
    let a = ArraySeqStPerSLit![-5, -2, -8, -1, -9];
    let sorted = ArraySeqStPerSLit![-9, -8, -5, -2, -1];

    for k in 0..sorted.length() {
        assert_eq!(a.select(k), Some(*sorted.nth(k)));
    }
}

#[test]
fn test_mixed() {
    let a = ArraySeqStPerSLit![-3, 7, -1, 0, 4, -5, 2];
    let sorted = ArraySeqStPerSLit![-5, -3, -1, 0, 2, 4, 7];

    for k in 0..sorted.length() {
        assert_eq!(a.select(k), Some(*sorted.nth(k)));
    }
}
