//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap21::Problem21_3::Problem21_3::*;
use apas_ai::Types::Types::*;

#[test]
fn test_points3d_loops_empty() {
    let result = points3d_loops(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_points3d_loops_one() {
    let result = points3d_loops(1);
    assert_eq!(result.length(), 1);
    // n=1: x in [0,1), y in [1,1], z in [2,2] => (0, 1, 2)
    assert_eq!(*result.nth(0), Pair(0, Pair(1, 2)));
}

#[test]
fn test_points3d_loops_two() {
    let result = points3d_loops(2);
    assert_eq!(result.length(), 8); // 2 * 2 * 2 = 8
    
    // x=0, y=1: (0,1,2), (0,1,3)
    assert_eq!(*result.nth(0), Pair(0, Pair(1, 2)));
    assert_eq!(*result.nth(1), Pair(0, Pair(1, 3)));
    
    // x=0, y=2: (0,2,2), (0,2,3)
    assert_eq!(*result.nth(2), Pair(0, Pair(2, 2)));
    assert_eq!(*result.nth(3), Pair(0, Pair(2, 3)));
    
    // x=1, y=1: (1,1,2), (1,1,3)
    assert_eq!(*result.nth(4), Pair(1, Pair(1, 2)));
    assert_eq!(*result.nth(5), Pair(1, Pair(1, 3)));
    
    // x=1, y=2: (1,2,2), (1,2,3)
    assert_eq!(*result.nth(6), Pair(1, Pair(2, 2)));
    assert_eq!(*result.nth(7), Pair(1, Pair(2, 3)));
}

#[test]
fn test_points3d_loops_three() {
    let result = points3d_loops(3);
    assert_eq!(result.length(), 27); // 3 * 3 * 3 = 27
    
    // Check first point
    assert_eq!(*result.nth(0), Pair(0, Pair(1, 2)));
    
    // Check last point
    assert_eq!(*result.nth(26), Pair(2, Pair(3, 4)));
}

#[test]
fn test_points3d_loops_ordering() {
    let result = points3d_loops(3);
    
    // Verify x-major, then y, then z ordering
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut prev_z = 0;
    
    for i in 0..result.length() {
        let Pair(x, Pair(y, z)) = *result.nth(i);
        
        if x != prev_x {
            assert!(x > prev_x);
            prev_y = 0;
            prev_z = 0;
        } else if y != prev_y {
            assert!(y > prev_y);
            prev_z = 0;
        } else {
            assert!(z > prev_z);
        }
        
        prev_x = x;
        prev_y = y;
        prev_z = z;
    }
}

#[test]
fn test_points3d_loops_bounds() {
    let n = 5;
    let result = points3d_loops(n);
    
    // Verify all points satisfy: 0 <= x < n, 1 <= y <= n, 2 <= z <= n+1
    for i in 0..result.length() {
        let Pair(x, Pair(y, z)) = *result.nth(i);
        assert!(x < n);
        assert!(y >= 1 && y <= n);
        assert!(z >= 2 && z <= n + 1);
    }
}

#[test]
fn test_points3d_loops_count() {
    for n in 1..=10 {
        let result = points3d_loops(n);
        assert_eq!(result.length(), n * n * n);
    }
}

#[test]
fn test_points3d_loops_large() {
    let result = points3d_loops(20);
    assert_eq!(result.length(), 8000); // 20^3 = 8000
    
    // Spot check first and last
    assert_eq!(*result.nth(0), Pair(0, Pair(1, 2)));
    assert_eq!(*result.nth(7999), Pair(19, Pair(20, 21)));
}
