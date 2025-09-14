//! Exercises 21.5 and 21.6: All contiguous subsequences and cost analysis.

use apas_ai::Types::Types::*;
use apas_ai::ArraySeqPer::ArraySeqPer::*;
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use apas_ai::ArraySeqPer;

/// Generate all contiguous subsequences using nested tabulate + flatten.
/// gpt-5-hard: Work: Θ(n^2), Span: Θ(lg n)
/// APAS: Work: Θ(n^2), Span: Θ(lg n)
fn all_contiguous_subseqs<T: Clone + Eq>(a: &ArrayPerS<T>) -> ArrayPerS<ArrayPerS<T>> {
    let n = a.length();
    let nested: ArrayPerS<ArrayPerS<ArrayPerS<T>>> =
        <ArrayPerS<ArrayPerS<ArrayPerS<T>>> as ArraySeqPerChap19Trait>::tabulate(
            |i| <ArrayPerS<ArrayPerS<T>> as ArraySeqPerChap19Trait>::tabulate(
                |j| ArrayPerS::from_vec(a.subseq(i, j + 1).to_vec()),
                n - i,
            ),
            n,
        );
    // flatten twice
    let mid: ArrayPerS<ArrayPerS<T>> =
        <ArrayPerS<ArrayPerS<T>> as ArraySeqPerChap18Trait>::flatten(&nested);
    mid
}

#[test]
fn test_all_contiguous_subseqs_n0() {
    let a: ArrayPerS<N> = ArraySeqPer![];
    let res = all_contiguous_subseqs(&a);
    assert_eq!(res.length(), 0);
}

#[test]
fn test_all_contiguous_subseqs_n3_values() {
    let a = ArraySeqPer![1, 2, 3];
    let res = all_contiguous_subseqs(&a);
    let v: Vec<Vec<N>> = res.iter().map(|s| s.iter().copied().collect()).collect();
    let expect = vec![
        vec![1], vec![1,2], vec![1,2,3],
        vec![2], vec![2,3],
        vec![3],
    ];
    assert_eq!(v, expect);
}

#[test]
fn test_all_contiguous_subseqs_debug_shape() {
    let a = ArraySeqPer![1, 2];
    let res = all_contiguous_subseqs(&a);
    let dbg_str = format!("{:?}", res);
    assert!(!dbg_str.is_empty());
}
