pub mod TestAVLTreeSeqPerChap19 {
use apas_ai::Types::Types::*;
use apas_ai::AVLTreeSeqPer::AVLTreeSeqPer::*;
use apas_ai::AVLTreeSeqPerChap19::AVLTreeSeqPerChap19Trait;
use apas_ai::AVLTreeSeqPer; // macro import

#[test]
fn test_tabulate_and_map_ch19() {
    let t: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i, 5);
    let m: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::map(&t, |x| x * 2);
    let expected = AVLTreeSeqPer![0, 2, 4, 6, 8];
    assert_eq!(m, expected);
}

#[test]
fn test_select_and_append_ch19() {
    let a: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i, 3);
    let b: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i + 3, 3);
    assert_eq!(<AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::select(&a, &b, 0), Some(0));
    assert_eq!(<AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::select(&a, &b, 4), Some(4));
    assert_eq!(<AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::select(&a, &b, 6), None);
    let c: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::append(&a, &b);
    let expected = AVLTreeSeqPer![0, 1, 2, 3, 4, 5];
    assert_eq!(c, expected);
}

#[test]
fn test_deflate_and_filter_ch19() {
    let t: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i, 6);
    let d: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::deflate(|x| if *x % 2 == 0 { B::True } else { B::False }, &2);
    let expected_d = AVLTreeSeqPer![2];
    assert_eq!(d, expected_d);
    let f: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::filter(&t, |x| if *x < 3 { B::True } else { B::False });
    let expected_f = AVLTreeSeqPer![0, 1, 2];
    assert_eq!(f, expected_f);
}

#[test]
fn test_iter_inorder_after_pipeline_ch19() {
    let a: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i, 4);
    let b: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i + 3, 4);
    let c = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::append(&a, &b);
    let f = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::filter(&c, |x| if *x >= 2 { B::True } else { B::False });
    let m = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::map(&f, |x| x * 2);
    let vals: Vec<N> = m.iter().map(|x| *x).collect();
    assert_eq!(vals, vec![4, 6, 8, 10, 12]);
}

}
