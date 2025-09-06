//! Problem 21.3 (Points in 3D) using ArraySeqPer — imperative triple loop.

use apas_ai::Types::N;
use apas_ai::{ArrayPerS, ArraySeqPerTrait};

/// Generate points (x, y, z) with 0 ≤ x ≤ n−1, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1 in x-major, then y, then z order.
/// gpt-5-hard: Work: Θ(n^3), Span: Θ(n^3)
fn points3d_loops(n: N) -> ArrayPerS<(N, N, N)> {
    if n == 0 { return ArrayPerS::from_vec(Vec::new()); }
    let len = n * n * n;
    let mut v: Vec<(N, N, N)> = Vec::with_capacity(len);
    for x in 0..n {
        for y in 1..=n {
            for z in 2..=n + 1 {
                v.push((x, y, z));
            }
        }
    }
    ArrayPerS::from_vec(v)
}

#[test]
fn test_points3d_loops_n0_empty() {
    let s = points3d_loops(0);
    assert_eq!(s.length(), 0);
}

#[test]
fn test_points3d_loops_n1_single() {
    let s = points3d_loops(1);
    let expect = ArrayPerS::from_vec(vec![(0, 1, 2)]);
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_loops_n2_values_and_order() {
    let s = points3d_loops(2);
    let expect = ArrayPerS::from_vec(vec![
        (0,1,2),(0,1,3),
        (0,2,2),(0,2,3),
        (1,1,2),(1,1,3),
        (1,2,2),(1,2,3),
    ]);
    assert_eq!(s.length(), 8);
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_loops_iterator_order() {
    let s = points3d_loops(2);
    let collected: Vec<(N, N, N)> = s.iter().copied().collect();
    assert_eq!(collected, vec![(0,1,2),(0,1,3),(0,2,2),(0,2,3),(1,1,2),(1,1,3),(1,2,2),(1,2,3)]);
}

#[test]
fn test_points3d_loops_debug_shape() {
    let s = points3d_loops(2);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
