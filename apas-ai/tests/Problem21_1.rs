//! Problem 21.1 (Points in 2D) tests and construction using ArraySeqPer.

use apas_ai::Types::N;
use apas_ai::{ArrayPerS, ArraySeqPerTrait};

/// Construct the sequence of 2D points (x, y) with 0 ≤ x < n and 1 ≤ y < n,
/// ordered by x major, then y.
fn points2d(n: N) -> ArrayPerS<(N, N)> {
    if n == 0 { return ArrayPerS::from_vec(Vec::new()); }
    let len = n * (n - 1);
    let mut v: Vec<(N, N)> = Vec::with_capacity(len);
    for x in 0..n {
        for y in 1..n {
            v.push((x, y));
        }
    }
    ArrayPerS::from_vec(v)
}

#[test]
fn test_points2d_n3_example() {
    let s = points2d(3);
    let expect = ArrayPerS::from_vec(vec![(0, 1), (0, 2), (1, 1), (1, 2), (2, 1), (2, 2)]);
    assert_eq!(s.length(), expect.length());
    assert_eq!(s, expect);
}

#[test]
fn test_points2d_n1_empty() {
    let s = points2d(1);
    assert_eq!(s.length(), 0);
}

#[test]
fn test_points2d_n2_basic_values() {
    let s = points2d(2);
    assert_eq!(s.length(), 2);
    assert_eq!(*s.nth(0), (0, 1));
    assert_eq!(*s.nth(1), (1, 1));
}

#[test]
fn test_points2d_iterator_in_order() {
    let s = points2d(4);
    let collected: Vec<(N, N)> = s.iter().copied().collect();
    let expect = vec![(0,1),(0,2),(0,3),(1,1),(1,2),(1,3),(2,1),(2,2),(2,3),(3,1),(3,2),(3,3)];
    assert_eq!(collected, expect);
}

#[test]
fn test_points2d_debug_display_shapes() {
    let s = points2d(3);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
