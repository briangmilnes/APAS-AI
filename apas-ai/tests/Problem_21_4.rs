//! Problem 21.4 (Cartesian Product) tests.

use apas_ai::Types::Types::*;
use apas_ai::ArraySeqPer::ArraySeqPer::*;
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use apas_ai::ArraySeqPer;

/// Cartesian product by explicit loops (x-major then y).
/// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(|a|·|b|)
fn cartesian_loops(a: &ArrayPerS<N>, b: &ArrayPerS<&'static str>) -> ArrayPerS<(N, &'static str)> {
    let mut v: Vec<(N, &'static str)> = Vec::with_capacity(a.length() * b.length());
    for i in 0..a.length() {
        for j in 0..b.length() {
            v.push((*a.nth(i), *b.nth(j)));
        }
    }
    ArrayPerS::from_vec(v)
}

/// Cartesian product using map + flatten: flatten(map(\x. map(\y. (x,y)) b) a)
/// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
fn cartesian_tab_flat(a: &ArrayPerS<N>, b: &ArrayPerS<&'static str>) -> ArrayPerS<(N, &'static str)> {
    let nested: ArrayPerS<ArrayPerS<(N, &'static str)>> =
        <ArrayPerS<ArrayPerS<(N, &'static str)>> as ArraySeqPerChap19Trait>::map(
            a,
            |x| <ArrayPerS<(N, &'static str)> as ArraySeqPerChap19Trait>::map(b, |y| (*x, *y)),
        );
    <ArrayPerS<(N, &'static str)> as ArraySeqPerChap18Trait>::flatten(&nested)
}

#[test]
fn test_cartesian_loops_basic() {
    let a = ArraySeqPer![1, 2];
    let b = ArraySeqPer!["3.0", "4.0", "5.0"];
    let s = cartesian_loops(&a, &b);
    let expect = ArraySeqPer![(1, "3.0"), (1, "4.0"), (1, "5.0"), (2, "3.0"), (2, "4.0"), (2, "5.0")];
    assert_eq!(s, expect);
}

#[test]
fn test_cartesian_tab_flat_basic() {
    let a = ArraySeqPer![1, 2];
    let b = ArraySeqPer!["3.0", "4.0", "5.0"];
    let s = cartesian_tab_flat(&a, &b);
    let expect = ArraySeqPer![(1, "3.0"), (1, "4.0"), (1, "5.0"), (2, "3.0"), (2, "4.0"), (2, "5.0")];
    assert_eq!(s, expect);
}

#[test]
fn test_cartesian_iterator_order() {
    let a = ArraySeqPer![1, 2];
    let b = ArraySeqPer!["3", "4"];
    let s = cartesian_tab_flat(&a, &b);
    let v: Vec<(N, &str)> = s.iter().copied().collect();
    assert_eq!(v, vec![(1, "3"), (1, "4"), (2, "3"), (2, "4")]);
}

#[test]
fn test_cartesian_debug_shape() {
    let a = ArraySeqPer![1];
    let b = ArraySeqPer!["x"];
    let s = cartesian_loops(&a, &b);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
