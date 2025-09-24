//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Problem 21.3 (Points in 3D) using ArraySeqPer — imperative triple loop.

use apas_ai::ArraySeqStPer;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Types::Types::*;

/// Generate points (x, y, z) with 0 ≤ x ≤ n−1, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1 in x-major, then y, then z order.
/// gpt-5-hard: Work: Θ(n^3), Span: Θ(n^3)
fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {
    if n == 0 {
        return ArraySeqStPerSLit![];
    }
    let len = n * n * n;
    let mut v: Vec<Pair<N, Pair<N, N>>> = Vec::with_capacity(len);
    for x in 0..n {
        for y in 1..=n {
            for z in 2..=n + 1 {
                v.push(Pair(x, Pair(y, z)));
            }
        }
    }
    ArraySeqStPerS::from_vec(v)
}

#[test]
fn test_points3d_loops_n0_empty() {
    let s = points3d_loops(0);
    assert_eq!(s.length(), 0);
}

#[test]
fn test_points3d_loops_n1_single() {
    let s = points3d_loops(1);
    let expect = ArraySeqStPerSLit![Pair(0, Pair(1, 2))];
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_loops_n2_values_and_order() {
    let s = points3d_loops(2);
    let expect = ArraySeqStPerSLit![
        Pair(0, Pair(1, 2)),
        Pair(0, Pair(1, 3)),
        Pair(0, Pair(2, 2)),
        Pair(0, Pair(2, 3)),
        Pair(1, Pair(1, 2)),
        Pair(1, Pair(1, 3)),
        Pair(1, Pair(2, 2)),
        Pair(1, Pair(2, 3)),
    ];
    assert_eq!(s.length(), 8);
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_loops_iterator_order() {
    let s = points3d_loops(2);
    let collected: Vec<Pair<N, Pair<N, N>>> = s.iter().copied().collect();
    assert_eq!(
        collected,
        vec![
            Pair(0, Pair(1, 2)),
            Pair(0, Pair(1, 3)),
            Pair(0, Pair(2, 2)),
            Pair(0, Pair(2, 3)),
            Pair(1, Pair(1, 2)),
            Pair(1, Pair(1, 3)),
            Pair(1, Pair(2, 2)),
            Pair(1, Pair(2, 3))
        ]
    );
}

#[test]
fn test_points3d_loops_debug_shape() {
    let s = points3d_loops(2);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
