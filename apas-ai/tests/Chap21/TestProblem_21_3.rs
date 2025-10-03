//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Problem 21.3 (Points in 3D) using ArraySeqPer — imperative triple loop.

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap21::Problem21_3::Problem21_3::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStPerSLit, PairLit};

fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {
    if n == 0 {
        return ArraySeqStPerSLit![];
    }
    let len = n * n * n;
    let mut v: Vec<Pair<N, Pair<N, N>>> = Vec::with_capacity(len);
    for x in 0..n {
        for y in 1..=n {
            for z in 2..=n + 1 {
                v.push(PairLit!(x, Pair(y, z)));
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
    let expect = ArraySeqStPerSLit![PairLit!(0, Pair(1, 2))];
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_loops_n2_values_and_order() {
    let s = points3d_loops(2);
    let expect = ArraySeqStPerS::from_vec(vec![
        PairLit!(0, Pair(1, 2)),
        PairLit!(0, Pair(1, 3)),
        PairLit!(0, Pair(2, 2)),
        PairLit!(0, Pair(2, 3)),
        PairLit!(1, Pair(1, 2)),
        PairLit!(1, Pair(1, 3)),
        PairLit!(1, Pair(2, 2)),
        PairLit!(1, Pair(2, 3)),
    ]);
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
            PairLit!(0, Pair(1, 2)),
            PairLit!(0, Pair(1, 3)),
            PairLit!(0, Pair(2, 2)),
            PairLit!(0, Pair(2, 3)),
            PairLit!(1, Pair(1, 2)),
            PairLit!(1, Pair(1, 3)),
            PairLit!(1, Pair(2, 2)),
            PairLit!(1, Pair(2, 3))
        ]
    );
}

#[test]
fn test_points3d_loops_debug_shape() {
    let s = points3d_loops(2);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
}
