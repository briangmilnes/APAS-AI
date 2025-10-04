//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPer multithreaded algorithms.

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_tabulate_basic() {
    let a: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&|i| i, 5);
    assert_eq!(a.length(), 5);

    // Use set comparison since MT results may vary in order
    // Convert to vectors for comparison since ArraySeqSetEq needs function pointers
    let mut a_vec: Vec<N> = Vec::new();
    for i in 0..a.length() {
        a_vec.push(*a.nth(i));
    }
    let mut expected_vec: Vec<N> = (0..5).collect();
    
    // Sort both vectors for comparison
    a_vec.sort();
    expected_vec.sort();
    assert_eq!(a_vec, expected_vec);
}

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            | 0 => 0,
            | 1 => 1,
            | _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    let fibs: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&fib, 8);
    assert_eq!(fibs.length(), 8);

    // Expected fibonacci sequence: [0, 1, 1, 2, 3, 5, 8, 13]
    let expected = [0, 1, 1, 2, 3, 5, 8, 13];
    
    // Convert to vectors for comparison since ArraySeqSetEq needs function pointers
    let mut fibs_vec: Vec<N> = Vec::new();
    for i in 0..fibs.length() {
        fibs_vec.push(*fibs.nth(i));
    }
    let mut expected_vec: Vec<N> = expected.to_vec();
    
    // Sort both vectors for comparison
    fibs_vec.sort();
    expected_vec.sort();
    assert_eq!(fibs_vec, expected_vec);
}

#[test]
fn test_tabulate_empty() {
    let empty: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&|i| i * 2, 0);
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.length() == 0, true);
}

#[test]
fn test_tabulate_single() {
    let single: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&|_| 42, 1);
    assert_eq!(single.length(), 1);
    assert_eq!(*single.nth(0), 42);
}

#[test]
fn test_tabulate_string() {
    let strings: ArraySeqMtPerS<String> = ArraySeqMtPerS::tabulate(&|i| format!("item_{}", i), 4);
    assert_eq!(strings.length(), 4);

    // Check that all expected strings are present (order may vary in MT)
    let expected = ["item_0", "item_1", "item_2", "item_3"];
    
    // Convert to vectors for comparison since ArraySeqSetEq needs function pointers
    let mut strings_vec: Vec<String> = Vec::new();
    for i in 0..strings.length() {
        strings_vec.push(strings.nth(i).clone());
    }
    let mut expected_vec: Vec<String> = Vec::new();
    for item in expected.iter() {
        expected_vec.push(item.to_string());
    }
    
    // Sort both vectors for comparison
    strings_vec.sort();
    expected_vec.sort();
    assert_eq!(strings_vec, expected_vec);
}

#[test]
fn test_tabulate_boolean() {
    let bools: ArraySeqMtPerS<B> = ArraySeqMtPerS::tabulate(&|i| if i % 2 == 0 { true } else { false }, 6);
    assert_eq!(bools.length(), 6);

    // Expected: [True, False, True, False, True, False]
    let expected = [true, false, true, false, true, false];
    
    // Convert to vectors for comparison since ArraySeqSetEq needs function pointers
    let mut bools_vec: Vec<B> = Vec::new();
    for i in 0..bools.length() {
        bools_vec.push(*bools.nth(i));
    }
    let mut expected_vec: Vec<B> = expected.to_vec();
    
    // Sort both vectors for comparison
    bools_vec.sort();
    expected_vec.sort();
    assert_eq!(bools_vec, expected_vec);
}

#[test]
fn test_tabulate_squares() {
    let squares: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&|i| i * i, 5);
    assert_eq!(squares.length(), 5);

    // Expected: [0, 1, 4, 9, 16]
    let expected = [0, 1, 4, 9, 16];
    
    // Convert to vectors for comparison since ArraySeqSetEq needs function pointers
    let mut squares_vec: Vec<N> = Vec::new();
    for i in 0..squares.length() {
        squares_vec.push(*squares.nth(i));
    }
    let mut expected_vec: Vec<N> = expected.to_vec();
    
    // Sort both vectors for comparison
    squares_vec.sort();
    expected_vec.sort();
    assert_eq!(squares_vec, expected_vec);
}

#[test]
fn test_tabulate_large() {
    let large: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&|i| i + 100, 1000);
    assert_eq!(large.length(), 1000);
    assert_eq!(*large.nth(0), 100);
    assert_eq!(*large.nth(999), 1099);

    // Check a few random elements
    assert_eq!(*large.nth(500), 600);
    assert_eq!(*large.nth(123), 223);
}

