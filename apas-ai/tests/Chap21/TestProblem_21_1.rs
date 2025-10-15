//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap21::Problem21_1::Problem21_1::*;
use apas_ai::Types::Types::*;

#[test]
fn test_points2d_empty() {
    let result = points2d(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_points2d_one() {
    let result = points2d(1);
    assert_eq!(result.length(), 0); // n=1: x in [0,1), y in [1,1) => 0 points
}

#[test]
fn test_points2d_two() {
    let result = points2d(2);
    assert_eq!(result.length(), 2); // n=2: x in [0,2), y in [1,2) => 2 points
    assert_eq!(*result.nth(0), Pair(0, 1));
    assert_eq!(*result.nth(1), Pair(1, 1));
}

#[test]
fn test_points2d_three() {
    let result = points2d(3);
    assert_eq!(result.length(), 6); // n=3: 3 * 2 = 6 points
    
    // x=0: (0,1), (0,2)
    assert_eq!(*result.nth(0), Pair(0, 1));
    assert_eq!(*result.nth(1), Pair(0, 2));
    
    // x=1: (1,1), (1,2)
    assert_eq!(*result.nth(2), Pair(1, 1));
    assert_eq!(*result.nth(3), Pair(1, 2));
    
    // x=2: (2,1), (2,2)
    assert_eq!(*result.nth(4), Pair(2, 1));
    assert_eq!(*result.nth(5), Pair(2, 2));
}

#[test]
fn test_points2d_five() {
    let result = points2d(5);
    assert_eq!(result.length(), 20); // n=5: 5 * 4 = 20 points
    
    // Check first row (x=0)
    for y in 1..5 {
        let idx = y - 1;
        assert_eq!(*result.nth(idx), Pair(0, y));
    }
    
    // Check last row (x=4)
    for y in 1..5 {
        let idx = 4 * 4 + (y - 1);
        assert_eq!(*result.nth(idx), Pair(4, y));
    }
}

#[test]
fn test_points2d_ordering() {
    let result = points2d(4);
    assert_eq!(result.length(), 12);
    
    // Verify x-major ordering
    let mut prev_x = 0;
    let mut prev_y = 0;
    
    for i in 0..result.length() {
        let Pair(x, y) = *result.nth(i);
        
        if x != prev_x {
            assert!(x > prev_x);
            prev_y = 0; // Reset y when x changes
        } else {
            assert!(y > prev_y);
        }
        
        prev_x = x;
        prev_y = y;
    }
}

#[test]
fn test_points2d_bounds() {
    let n = 10;
    let result = points2d(n);
    
    // Verify all points satisfy: 0 <= x < n and 1 <= y < n
    for i in 0..result.length() {
        let Pair(x, y) = *result.nth(i);
        assert!(x < n);
        assert!(y >= 1 && y < n);
    }
}

#[test]
fn test_points2d_large() {
    let result = points2d(100);
    assert_eq!(result.length(), 9900); // 100 * 99 = 9900
    
    // Spot check first and last
    assert_eq!(*result.nth(0), Pair(0, 1));
    assert_eq!(*result.nth(9899), Pair(99, 99));
}

