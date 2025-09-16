//! Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten.

use apas_ai::Types::Types::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
use apas_ai::ArraySeqStPerChap19::ArraySeqStPerChap19::*;
use apas_ai::ArraySeqStPer;

/// Functional form: points2D n = flatten (tabulate (\x. tabulate (\y. (x, y+1)) (n-1)) n)
/// gpt-5-hard: Work: Θ(n^2), Span: Θ(lg n)
fn points2d_tab_flat(n: N) -> ArrayStPerS<Pair<N, N>> {
    if n == 0 { return ArraySeqStPer![]; }
    let inner: ArrayStPerS<ArrayStPerS<Pair<N, N>>> =
        <ArrayStPerS<ArrayStPerS<Pair<N, N>>> as ArraySeqStPerChap19Trait<ArrayStPerS<Pair<N, N>>>>::tabulate(
            |x| <ArrayStPerS<Pair<N, N>> as ArraySeqStPerChap18Trait<Pair<N, N>>>::tabulate(|y| Pair(x, y + 1), n - 1),
            n,
        );
    <ArrayStPerS<Pair<N, N>> as ArraySeqStPerChap18Trait<Pair<N, N>>>::flatten(&inner)
}

#[test]
fn test_points2d_n3_example() {
    let s = points2d_tab_flat(3);
    let expect = ArraySeqStPer![Pair(0, 1), Pair(0, 2), Pair(1, 1), Pair(1, 2), Pair(2, 1), Pair(2, 2)];
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
    assert_eq!(*s.nth(0), Pair(0, 1));
    assert_eq!(*s.nth(1), Pair(1, 1));
}

#[test]
fn test_points2d_iterator_in_order() {
    let s = points2d_tab_flat(4);
    let collected: Vec<Pair<N, N>> = s.iter().copied().collect();
    let expect = vec![Pair(0,1),Pair(0,2),Pair(0,3),Pair(1,1),Pair(1,2),Pair(1,3),Pair(2,1),Pair(2,2),Pair(2,3),Pair(3,1),Pair(3,2),Pair(3,3)];
    assert_eq!(collected, expect);
}

#[test]
fn test_points2d_debug_shape() {
    let s = points2d_tab_flat(3);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
