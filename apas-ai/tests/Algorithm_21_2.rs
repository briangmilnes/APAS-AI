//! Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.

use apas_ai::Types::Types::*;
use apas_ai::ArraySeqPer::ArraySeqPer::*;
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use apas_ai::ArraySeqPer;

/// Comprehension form: 〈(x,y,z): 0 ≤ x ≤ n−1, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
/// Implemented as: flatten ∘ (tabulate_x (flatten ∘ (tabulate_y (tabulate_z))))
/// gpt-5-hard: Work: Θ(n^3), Span: Θ(lg n)
fn points3d_tab_flat(n: N) -> ArrayPerS<(N, N, N)> {
    if n == 0 { return ArraySeqPer![]; }
    let outer: ArrayPerS<ArrayPerS<(N, N, N)>> =
        <ArrayPerS<ArrayPerS<(N, N, N)>> as ArraySeqPerChap19Trait>::tabulate(
            |x| {
                let mid: ArrayPerS<ArrayPerS<(N, N, N)>> =
                    <ArrayPerS<ArrayPerS<(N, N, N)>> as ArraySeqPerChap19Trait>::tabulate(
                        |y| <ArrayPerS<(N, N, N)> as ArraySeqPerChap18Trait>::tabulate(
                            |z_idx| (x, y + 1, z_idx + 2),
                            n + 1 - 2 + 1, // z: 2..=n+1 has length n
                        ),
                        n, // y: 1..=n has length n
                    );
                <ArrayPerS<(N, N, N)> as ArraySeqPerChap18Trait>::flatten(&mid)
            },
            n, // x: 0..=n−1 has length n
        );
    <ArrayPerS<(N, N, N)> as ArraySeqPerChap18Trait>::flatten(&outer)
}

#[test]
fn test_points3d_tab_flat_n0_empty() {
    let s = points3d_tab_flat(0);
    assert_eq!(s.length(), 0);
}

#[test]
fn test_points3d_tab_flat_n1_single() {
    let s = points3d_tab_flat(1);
    let expect = ArraySeqPer![(0, 1, 2)];
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_tab_flat_n2_values_and_order() {
    let s = points3d_tab_flat(2);
    let expect = ArraySeqPer![
        (0,1,2),(0,1,3),
        (0,2,2),(0,2,3),
        (1,1,2),(1,1,3),
        (1,2,2),(1,2,3),
    ];
    assert_eq!(s.length(), 8);
    assert_eq!(s, expect);
}

#[test]
fn test_points3d_tab_flat_iterator_order() {
    let s = points3d_tab_flat(2);
    let collected: Vec<(N, N, N)> = s.iter().copied().collect();
    assert_eq!(collected, vec![(0,1,2),(0,1,3),(0,2,2),(0,2,3),(1,1,2),(1,1,3),(1,2,2),(1,2,3)]);
}

#[test]
fn test_points3d_tab_flat_debug_shape() {
    let s = points3d_tab_flat(2);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
