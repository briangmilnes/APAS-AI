//! Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten.

use apas_ai::Types::N;
use apas_ai::{ArrayPerS, ArraySeqPerTrait};
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;

/// Functional form: points2D n = flatten (tabulate (\x. tabulate (\y. (x, y+1)) (n-1)) n)
/// gpt-5-hard: Work: Θ(n^2), Span: Θ(lg n)
fn points2d_tab_flat(n: N) -> ArrayPerS<(N, N)> {
    if n == 0 { return ArrayPerS::from_vec(Vec::new()); }
    let inner: ArrayPerS<ArrayPerS<(N, N)>> =
        <ArrayPerS<ArrayPerS<(N, N)>> as ArraySeqPerChap19Trait>::tabulate(
            |x| <ArrayPerS<(N, N)> as ArraySeqPerChap18Trait>::tabulate(|y| (x, y + 1), n - 1),
            n,
        );
    <ArrayPerS<(N, N)> as ArraySeqPerChap18Trait>::flatten(&inner)
}

#[test]
fn test_points2d_n3_example() {
    let s = points2d_tab_flat(3);
    let expect = ArrayPerS::from_vec(vec![(0, 1), (0, 2), (1, 1), (1, 2), (2, 1), (2, 2)]);
    assert_eq!(s, expect);
}

#[test]
fn test_points2d_n1_empty() {
    let s = points2d_tab_flat(1);
    assert_eq!(s.length(), 0);
}

#[test]
fn test_points2d_n2_basic_values() {
    let s = points2d_tab_flat(2);
    assert_eq!(s.length(), 2);
    assert_eq!(*s.nth(0), (0, 1));
    assert_eq!(*s.nth(1), (1, 1));
}

#[test]
fn test_points2d_iterator_in_order() {
    let s = points2d_tab_flat(4);
    let collected: Vec<(N, N)> = s.iter().copied().collect();
    let expect = vec![(0,1),(0,2),(0,3),(1,1),(1,2),(1,3),(2,1),(2,2),(2,3),(3,1),(3,2),(3,3)];
    assert_eq!(collected, expect);
}

#[test]
fn test_points2d_debug_shape() {
    let s = points2d_tab_flat(3);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
