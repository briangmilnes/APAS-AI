//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test the CLEAN pattern: one-line defaults in trait, multi-line logic in impl.

use apas_ai::Chap18::ArraySeqStEphClean::*;
use apas_ai::Types::Types::*;

#[test]
fn test_one_line_defaults() {
    // These all come from one-line defaults in the trait
    let seq: ArraySeqStEphCleanS<i32> = ArraySeqStEphCleanS::empty();
    assert_eq!(seq.length(), 0);

    let seq = ArraySeqStEphCleanS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.nth(0), &42);

    let seq = ArraySeqStEphCleanS::new(5, 10);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth(3), &10);

    let sum: i32 = seq.iter().sum();
    assert_eq!(sum, 50);
}

#[test]
fn test_multi_line_defaults() {
    // These come from multi-line implementations in the impl block
    let mut seq = ArraySeqStEphCleanS::new(5, 10);

    let result = seq.set(2, 99);
    assert!(result.is_ok());
    assert_eq!(seq.nth(2), &99);

    let result = seq.set(100, 1);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Index out of bounds");

    let _ = seq.update(Pair(4, 88));
    assert_eq!(seq.nth(4), &88);
}

#[test]
fn test_from_vec() {
    let seq = ArraySeqStEphCleanS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth(0), &1);
    assert_eq!(seq.nth(4), &5);
}

#[test]
fn test_mutations_chain() {
    let mut seq = ArraySeqStEphCleanS::from_vec(vec![10, 20, 30]);
    let _ = seq.update(Pair(0, 100));
    let _ = seq.set(1, 200);
    let _ = seq.update(Pair(2, 300));

    assert_eq!(seq.nth(0), &100);
    assert_eq!(seq.nth(1), &200);
    assert_eq!(seq.nth(2), &300);
}

#[test]
fn test_display_and_debug() {
    let seq = ArraySeqStEphCleanS::from_vec(vec![1, 2, 3]);
    let display = format!("{}", seq);
    let debug = format!("{:?}", seq);

    assert_eq!(display, "[1, 2, 3]");
    assert_eq!(debug, "[1, 2, 3]");
}

#[test]
fn test_equality() {
    let seq1 = ArraySeqStEphCleanS::from_vec(vec![1, 2, 3]);
    let seq2 = ArraySeqStEphCleanS::from_vec(vec![1, 2, 3]);
    let seq3 = ArraySeqStEphCleanS::from_vec(vec![1, 2, 4]);

    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
}

#[test]
fn test_into_iterator() {
    let seq = ArraySeqStEphCleanS::from_vec(vec![10, 20, 30]);

    // Borrow iterator
    let sum: i32 = (&seq).into_iter().sum();
    assert_eq!(sum, 60);
    assert_eq!(seq.length(), 3); // seq still valid

    // Consuming iterator
    let doubled: Vec<i32> = seq.into_iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![20, 40, 60]);
    // seq is now moved/consumed
}
