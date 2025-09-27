//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Problem 21.1 (Points in 2D) tests and construction using ArraySeqPer.

pub mod Test38Problem21_1 {
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Types::Types::*;
    use apas_ai::PairLit;

/// Construct the sequence of 2D points (x, y) with 0 ≤ x < n and 1 ≤ y < n,
/// ordered by x major, then y.
fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>> {
    if n == 0 {
        return ArraySeqStPerS::from_vec(vec![]);
    }
    let len = n * (n - 1);
    let mut v: Vec<Pair<N, N>> = Vec::with_capacity(len);
    for x in 0..n {
        for y in 1..n {
            v.push(PairLit!(x, y));
        }
    }
    ArraySeqStPerS::from_vec(v)
}

#[test]
fn test_points2d_n3_example() {
    let s = points2d(3);
    let expect = ArraySeqStPerS::from_vec(vec![
        PairLit!(0, 1),
        PairLit!(0, 2),
        PairLit!(1, 1),
        PairLit!(1, 2),
        PairLit!(2, 1),
        PairLit!(2, 2),
    ]);
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
    assert_eq!(*s.nth(0), PairLit!(0, 1));
    assert_eq!(*s.nth(1), PairLit!(1, 1));
}

#[test]
fn test_points2d_iterator_in_order() {
    let s = points2d(4);
    let collected: Vec<Pair<N, N>> = s.iter().copied().collect();
    let expect = vec![
        PairLit!(0, 1),
        PairLit!(0, 2),
        PairLit!(0, 3),
        PairLit!(1, 1),
        PairLit!(1, 2),
        PairLit!(1, 3),
        PairLit!(2, 1),
        PairLit!(2, 2),
        PairLit!(2, 3),
        PairLit!(3, 1),
        PairLit!(3, 2),
        PairLit!(3, 3),
    ];
    assert_eq!(collected, expect);
}

#[test]
fn test_points2d_debug_display_shapes() {
    let s = points2d(3);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}

}
