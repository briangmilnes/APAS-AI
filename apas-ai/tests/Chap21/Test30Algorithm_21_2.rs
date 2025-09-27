//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.

pub mod Test30Algorithm_21_2 {
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Types::Types::*;
    use apas_ai::PairLit;

/// Comprehension form: 〈(x,y,z): 0 ≤ x ≤ n−1, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
/// Implemented as: flatten ∘ (tabulate_x (flatten ∘ (tabulate_y (tabulate_z))))
/// gpt-5-hard: Work: Θ(n^3), Span: Θ(lg n)
fn points3d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {
    if n == 0 {
        return ArraySeqStPerSLit![];
    }
    let outer: ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> =
        <ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> as ArraySeqStPerTrait<
            ArraySeqStPerS<Pair<N, Pair<N, N>>>,
        >>::tabulate(
            &|x| {
                let mid: ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> =
                    <ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> as ArraySeqStPerTrait<
                        ArraySeqStPerS<Pair<N, Pair<N, N>>>,
                    >>::tabulate(
                        &|y| {
                            <ArraySeqStPerS<Pair<N, Pair<N, N>>> as ArraySeqStPerTrait<Pair<N, Pair<N, N>>>>::tabulate(
                                &|z_idx| PairLit!(x, Pair(y + 1, z_idx + 2)),
                                n + 1 - 2 + 1, // z: 2..=n+1 has length n
                            )
                        },
                        n, // y: 1..=n has length n
                    );
                <ArraySeqStPerS<Pair<N, Pair<N, N>>> as ArraySeqStPerTrait<Pair<N, Pair<N, N>>>>::flatten(&mid)
            },
            n, // x: 0..=n−1 has length n
        );
    <ArraySeqStPerS<Pair<N, Pair<N, N>>> as ArraySeqStPerTrait<Pair<N, Pair<N, N>>>>::flatten(&outer)
}

#[test]
fn test_points3d_tab_flat_n0_empty() {
    let s = points3d_tab_flat(0);
    assert_eq!(s.length(), 0);
}

#[test]
fn test_points3d_tab_flat_n1_single() {
    let s = points3d_tab_flat(1);
    let expect = ArraySeqStPerSLit![PairLit!(0, Pair(1, 2))];
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_tab_flat_n2_values_and_order() {
    let s = points3d_tab_flat(2);
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
fn test_points3d_tab_flat_iterator_order() {
    let s = points3d_tab_flat(2);
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
fn test_points3d_tab_flat_debug_shape() {
    let s = points3d_tab_flat(2);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}

}
