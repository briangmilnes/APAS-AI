//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap21::Problem21_4::Problem21_4::*;
use apas_ai::Types::Types::*;

// Helper to create test sequences
fn make_seq_n(items: Vec<N>) -> ArraySeqStPerS<N> {
    ArraySeqStPerS::from_vec(items)
}

fn make_seq_str(items: Vec<&'static str>) -> ArraySeqStPerS<&'static str> {
    ArraySeqStPerS::from_vec(items)
}

#[test]
fn test_cartesian_loops_empty_first() {
    let a = make_seq_n(vec![]);
    let b = make_seq_str(vec!["x", "y"]);
    let result = cartesian_loops(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_cartesian_loops_empty_second() {
    let a = make_seq_n(vec![1, 2]);
    let b = make_seq_str(vec![]);
    let result = cartesian_loops(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_cartesian_loops_empty_both() {
    let a = make_seq_n(vec![]);
    let b = make_seq_str(vec![]);
    let result = cartesian_loops(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_cartesian_loops_singleton() {
    let a = make_seq_n(vec![1]);
    let b = make_seq_str(vec!["a"]);
    let result = cartesian_loops(&a, &b);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), Pair(1, "a"));
}

#[test]
fn test_cartesian_loops_small() {
    let a = make_seq_n(vec![1, 2]);
    let b = make_seq_str(vec!["a", "b"]);
    let result = cartesian_loops(&a, &b);
    
    assert_eq!(result.length(), 4);
    assert_eq!(*result.nth(0), Pair(1, "a"));
    assert_eq!(*result.nth(1), Pair(1, "b"));
    assert_eq!(*result.nth(2), Pair(2, "a"));
    assert_eq!(*result.nth(3), Pair(2, "b"));
}

#[test]
fn test_cartesian_loops_asymmetric() {
    let a = make_seq_n(vec![1, 2, 3]);
    let b = make_seq_str(vec!["x", "y"]);
    let result = cartesian_loops(&a, &b);
    
    assert_eq!(result.length(), 6);
    assert_eq!(*result.nth(0), Pair(1, "x"));
    assert_eq!(*result.nth(1), Pair(1, "y"));
    assert_eq!(*result.nth(2), Pair(2, "x"));
    assert_eq!(*result.nth(3), Pair(2, "y"));
    assert_eq!(*result.nth(4), Pair(3, "x"));
    assert_eq!(*result.nth(5), Pair(3, "y"));
}

#[test]
fn test_cartesian_loops_ordering() {
    let a = make_seq_n(vec![10, 20, 30]);
    let b = make_seq_str(vec!["a", "b", "c"]);
    let result = cartesian_loops(&a, &b);
    
    assert_eq!(result.length(), 9);
    
    // Verify x-major ordering (outer loop is a, inner loop is b)
    let mut idx = 0;
    for i in 0..a.length() {
        for j in 0..b.length() {
            assert_eq!(*result.nth(idx), Pair(*a.nth(i), *b.nth(j)));
            idx += 1;
        }
    }
}

// Tests for cartesian_tab_flat (functional approach)

#[test]
fn test_cartesian_functional_empty_first() {
    let a = make_seq_n(vec![]);
    let b = make_seq_str(vec!["x", "y"]);
    let result = cartesian_tab_flat(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_cartesian_functional_empty_second() {
    let a = make_seq_n(vec![1, 2]);
    let b = make_seq_str(vec![]);
    let result = cartesian_tab_flat(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_cartesian_functional_singleton() {
    let a = make_seq_n(vec![1]);
    let b = make_seq_str(vec!["a"]);
    let result = cartesian_tab_flat(&a, &b);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), Pair(1, "a"));
}

#[test]
fn test_cartesian_functional_small() {
    let a = make_seq_n(vec![1, 2]);
    let b = make_seq_str(vec!["a", "b"]);
    let result = cartesian_tab_flat(&a, &b);
    
    assert_eq!(result.length(), 4);
    assert_eq!(*result.nth(0), Pair(1, "a"));
    assert_eq!(*result.nth(1), Pair(1, "b"));
    assert_eq!(*result.nth(2), Pair(2, "a"));
    assert_eq!(*result.nth(3), Pair(2, "b"));
}

#[test]
fn test_cartesian_functional_asymmetric() {
    let a = make_seq_n(vec![1, 2, 3]);
    let b = make_seq_str(vec!["x", "y"]);
    let result = cartesian_tab_flat(&a, &b);
    
    assert_eq!(result.length(), 6);
    assert_eq!(*result.nth(0), Pair(1, "x"));
    assert_eq!(*result.nth(1), Pair(1, "y"));
    assert_eq!(*result.nth(2), Pair(2, "x"));
    assert_eq!(*result.nth(3), Pair(2, "y"));
    assert_eq!(*result.nth(4), Pair(3, "x"));
    assert_eq!(*result.nth(5), Pair(3, "y"));
}

// Test that both implementations produce the same results

#[test]
fn test_equivalence_small() {
    let a = make_seq_n(vec![1, 2, 3]);
    let b = make_seq_str(vec!["a", "b", "c"]);
    
    let loops_result = cartesian_loops(&a, &b);
    let functional_result = cartesian_tab_flat(&a, &b);
    
    assert_eq!(loops_result.length(), functional_result.length());
    for i in 0..loops_result.length() {
        assert_eq!(*loops_result.nth(i), *functional_result.nth(i));
    }
}

#[test]
fn test_equivalence_large() {
    let a = make_seq_n((0..20).collect());
    let b = make_seq_str(vec!["a", "b", "c", "d", "e"]);
    
    let loops_result = cartesian_loops(&a, &b);
    let functional_result = cartesian_tab_flat(&a, &b);
    
    assert_eq!(loops_result.length(), 100);
    assert_eq!(functional_result.length(), 100);
    
    for i in 0..100 {
        assert_eq!(*loops_result.nth(i), *functional_result.nth(i));
    }
}

#[test]
fn test_cartesian_large() {
    let a = make_seq_n((0..50).collect());
    let b = make_seq_str(vec!["x", "y", "z"]);
    let result = cartesian_loops(&a, &b);
    
    assert_eq!(result.length(), 150);
    assert_eq!(*result.nth(0), Pair(0, "x"));
    assert_eq!(*result.nth(149), Pair(49, "z"));
}
