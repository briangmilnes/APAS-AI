//! Exercises 21.5 and 21.6: All contiguous subsequences and cost analysis.

use apas_ai::ArraySeqStPer;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArrayStPerSLit;
use apas_ai::Types::Types::*;

/// Generate all contiguous subsequences using nested tabulate + flatten.
/// gpt-5-hard: Work: Θ(n^2), Span: Θ(lg n)
/// APAS: Work: Θ(n^2), Span: Θ(lg n)
fn all_contiguous_subseqs<T: StT>(a: &ArrayStPerS<T>) -> ArrayStPerS<ArrayStPerS<T>> {
    let n = a.length();
    let nested: ArrayStPerS<ArrayStPerS<ArrayStPerS<T>>> =
        <ArrayStPerS<ArrayStPerS<ArrayStPerS<T>>> as ArraySeqStPerTrait<ArrayStPerS<ArrayStPerS<T>>>>::tabulate(
            |i| {
                <ArrayStPerS<ArrayStPerS<T>> as ArraySeqStPerTrait<ArrayStPerS<T>>>::tabulate(
                    |j| ArrayStPerS::from_vec(a.subseq(i, j + 1).to_vec()),
                    n - i,
                )
            },
            n,
        );
    // flatten twice
    let mid: ArrayStPerS<ArrayStPerS<T>> =
        <ArrayStPerS<ArrayStPerS<T>> as ArraySeqStPerTrait<ArrayStPerS<T>>>::flatten(&nested);
    mid
}

#[test]
fn test_all_contiguous_subseqs_n0() {
    let a: ArrayStPerS<N> = ArrayStPerSLit![];
    let res = all_contiguous_subseqs(&a);
    assert_eq!(res.length(), 0);
}

#[test]
fn test_all_contiguous_subseqs_n3_values() {
    let a = ArrayStPerSLit![1, 2, 3];
    let res = all_contiguous_subseqs(&a);
    let v: Vec<Vec<N>> = res.iter().map(|s| s.iter().copied().collect()).collect();
    let expect = vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![2], vec![2, 3], vec![3]];
    assert_eq!(v, expect);
}

#[test]
fn test_all_contiguous_subseqs_debug_shape() {
    let a = ArrayStPerSLit![1, 2];
    let res = all_contiguous_subseqs(&a);
    let dbg_str = format!("{:?}", res);
    assert!(!dbg_str.is_empty());
}
