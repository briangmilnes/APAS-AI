//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Problem 21.4 (Cartesian Product) tests.

use apas_ai::ArraySeqStPer;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Types::Types::*;

/// Cartesian product by explicit loops (x-major then y).
/// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(|a|·|b|)
fn cartesian_loops(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<&'static str>) -> ArraySeqStPerS<Pair<N, &'static str>> {
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());
    for i in 0..a.length() {
        for j in 0..b.length() {
            v.push(Pair(*a.nth(i), *b.nth(j)));
        }
    }
    ArraySeqStPerS::from_vec(v)
}

/// Cartesian product using map + flatten: flatten(map(\x. map(\y. (x,y)) b) a)
/// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
fn cartesian_tab_flat(
    a: &ArraySeqStPerS<N>,
    b: &ArraySeqStPerS<&'static str>,
) -> ArraySeqStPerS<Pair<N, &'static str>> {
    let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> =
        <ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> as ArraySeqStPerTrait<
            ArraySeqStPerS<Pair<N, &'static str>>,
        >>::tabulate(
            |i| {
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::tabulate(
                    |j| Pair(*a.nth(i), *b.nth(j)),
                    b.length(),
                )
            },
            a.length(),
        );
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flatten(&nested)
}

#[test]
fn test_cartesian_loops_basic() {
    let a = ArraySeqStPerSLit![1, 2];
    let b = ArraySeqStPerSLit!["3.0", "4.0", "5.0"];
    let s = cartesian_loops(&a, &b);
    let expect = ArraySeqStPerSLit![
        Pair(1, "3.0"),
        Pair(1, "4.0"),
        Pair(1, "5.0"),
        Pair(2, "3.0"),
        Pair(2, "4.0"),
        Pair(2, "5.0")
    ];
    assert_eq!(s, expect);
}

#[test]
fn test_cartesian_tab_flat_basic() {
    let a = ArraySeqStPerSLit![1, 2];
    let b = ArraySeqStPerSLit!["3.0", "4.0", "5.0"];
    let s = cartesian_tab_flat(&a, &b);
    let expect = ArraySeqStPerSLit![
        Pair(1, "3.0"),
        Pair(1, "4.0"),
        Pair(1, "5.0"),
        Pair(2, "3.0"),
        Pair(2, "4.0"),
        Pair(2, "5.0")
    ];
    assert_eq!(s, expect);
}

#[test]
fn test_cartesian_iterator_order() {
    let a = ArraySeqStPerSLit![1, 2];
    let b = ArraySeqStPerSLit!["3", "4"];
    let s = cartesian_tab_flat(&a, &b);
    let v: Vec<Pair<N, &str>> = s.iter().copied().collect();
    assert_eq!(v, vec![Pair(1, "3"), Pair(1, "4"), Pair(2, "3"), Pair(2, "4")]);
}

#[test]
fn test_cartesian_debug_shape() {
    let a = ArraySeqStPerSLit![1];
    let b = ArraySeqStPerSLit!["x"];
    let s = cartesian_loops(&a, &b);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
