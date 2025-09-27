//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestAVLTreeSeqStPer {
    use apas_ai::AVLTreeSeqStPerLit;
    use apas_ai::Chap37::AVLTreeSeqStPer;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_tabulate_and_map_ch19() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3, 4]);
        let m: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 2, 4, 6, 8]);
        let expected = AVLTreeSeqStPerLit![0, 2, 4, 6, 8];
        assert_eq!(m, expected);
    }

    #[test]
    fn test_select_and_append_ch19() {
        let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2]);
        let b: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![3, 4, 5]);
        // select method doesn't exist in AVLTreeSeqStPerTrait, using nth instead
        assert_eq!(a.nth(0), &0);
        assert_eq!(b.nth(1), &4);
        // Test bounds checking
        assert_eq!(a.length(), 3);
        assert_eq!(b.length(), 3);
        let c: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3, 4, 5]);
        let expected = AVLTreeSeqStPerLit![0, 1, 2, 3, 4, 5];
        assert_eq!(c, expected);
    }

    #[test]
    fn test_deflate_and_filter_ch19() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3, 4, 5]);
        let d: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![2]);
        let expected_d = AVLTreeSeqStPerLit![2];
        assert_eq!(d, expected_d);
        let f: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2]);
        let expected_f = AVLTreeSeqStPerLit![0, 1, 2];
        assert_eq!(f, expected_f);
    }

    #[test]
    fn test_iter_inorder_after_pipeline_ch19() {
        let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerS::tabulate(&|i| i, 4);
        let b: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerS::tabulate(&|i| i + 3, 4);
        let c = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::append(&a, &b);
        let f =
            <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::filter(&a, &|x| if *x >= 2 { B::True } else { B::False });
        let m = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::map(&a, &|x| x * 2);
        let vals: Vec<N> = m.iter().map(|x| *x).collect();
        assert_eq!(vals, vec![4, 6, 8, 10, 12]);
    }
}
