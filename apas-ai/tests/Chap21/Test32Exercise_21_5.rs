//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.5: All contiguous subsequences tests.

pub mod Test32Exercise_21_5 {
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Chap21::Exercise21_5::Exercise21_5::*;
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Types::Types::*;

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

}
