//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Exercises 21.5 and 21.6: All contiguous subsequences and cost analysis.

use apas_ai::ArraySeqStPer;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Types::Types::*;

/// Generate all contiguous subsequences using nested tabulate + flatten.
/// gpt-5-hard: Work: Θ(n^2), Span: Θ(lg n)
/// APAS: Work: Θ(n^2), Span: Θ(lg n)
fn all_contiguous_subseqs<T: StT>(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<ArraySeqStPerS<T>> {
    let n = a.length();
    let nested: ArraySeqStPerS<ArraySeqStPerS<ArraySeqStPerS<T>>> =
        <ArraySeqStPerS<ArraySeqStPerS<ArraySeqStPerS<T>>> as ArraySeqStPerTrait<ArraySeqStPerS<ArraySeqStPerS<T>>>>::tabulate(
            |i| {
                <ArraySeqStPerS<ArraySeqStPerS<T>> as ArraySeqStPerTrait<ArraySeqStPerS<T>>>::tabulate(
                    |j| a.subseq_copy(i, j + 1),
                    n - i,
                )
            },
            n,
        );
    // flatten twice
    let mid: ArraySeqStPerS<ArraySeqStPerS<T>> =
        <ArraySeqStPerS<ArraySeqStPerS<T>> as ArraySeqStPerTrait<ArraySeqStPerS<T>>>::flatten(&nested);
    mid
}

#[test]
fn test_all_contiguous_subseqs_n0() {
    let a: ArraySeqStPerS<N> = ArraySeqStPerSLit![];
    let res = all_contiguous_subseqs(&a);
    assert_eq!(res.length(), 0);
}

#[test]
fn test_all_contiguous_subseqs_n3_values() {
    let a = ArraySeqStPerSLit![1, 2, 3];
    let res = all_contiguous_subseqs(&a);
    let v: Vec<Vec<N>> = res.iter().map(|s| s.iter().copied().collect()).collect();
    let expect = vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![2], vec![2, 3], vec![3]];
    assert_eq!(v, expect);
}

#[test]
fn test_all_contiguous_subseqs_debug_shape() {
    let a = ArraySeqStPerSLit![1, 2];
    let res = all_contiguous_subseqs(&a);
    let dbg_str = format!("{:?}", res);
    assert!(!dbg_str.is_empty());
}
