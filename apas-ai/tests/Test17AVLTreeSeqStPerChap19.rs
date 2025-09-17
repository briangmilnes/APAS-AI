pub mod TestAVLTreeSeqStPerChap19 {
    use apas_ai::AVLTreeSeqStPer;
    use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::AVLTreeSeqStPerChap19::AVLTreeSeqStPerChap19::*;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_tabulate_and_map_ch19() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i, 5);
        let m: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::map(&t, |x| x * 2);
        let expected = AVLTreeSeqStPer![0, 2, 4, 6, 8];
        assert_eq!(m, expected);
    }

    #[test]
    fn test_select_and_append_ch19() {
        let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i, 3);
        let b: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i + 3, 3);
        assert_eq!(
            <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::select(&a, &b, 0),
            Some(0)
        );
        assert_eq!(
            <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::select(&a, &b, 4),
            Some(4)
        );
        assert_eq!(
            <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::select(&a, &b, 6),
            None
        );
        let c: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::append(&a, &b);
        let expected = AVLTreeSeqStPer![0, 1, 2, 3, 4, 5];
        assert_eq!(c, expected);
    }

    #[test]
    fn test_deflate_and_filter_ch19() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i, 6);
        let d: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::deflate(
            |x| if *x % 2 == 0 { B::True } else { B::False },
            &2,
        );
        let expected_d = AVLTreeSeqStPer![2];
        assert_eq!(d, expected_d);
        let f: AVLTreeSeqStPerS<N> =
            <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::filter(
                &t,
                |x| if *x < 3 { B::True } else { B::False },
            );
        let expected_f = AVLTreeSeqStPer![0, 1, 2];
        assert_eq!(f, expected_f);
    }

    #[test]
    fn test_iter_inorder_after_pipeline_ch19() {
        let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i, 4);
        let b: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i + 3, 4);
        let c = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::append(&a, &b);
        let f = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::filter(&c, |x| {
            if *x >= 2 { B::True } else { B::False }
        });
        let m = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::map(&f, |x| x * 2);
        let vals: Vec<N> = m.iter().map(|x| *x).collect();
        assert_eq!(vals, vec![4, 6, 8, 10, 12]);
    }
}
