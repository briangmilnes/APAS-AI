//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.

pub mod Test30Algorithm_21_2 {

    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Chap21::Algorithm21_2::Algorithm21_2::*;
    use apas_ai::PairLit;
    use apas_ai::Types::Types::*;

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
