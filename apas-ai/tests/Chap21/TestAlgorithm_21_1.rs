//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten.



    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Chap21::Algorithm21_1::Algorithm21_1::*;
    use apas_ai::PairLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_points2d_n3_example() {
        let s = points2d_tab_flat(3);
        let expect = ArraySeqStPerS::from_vec(vec![
            PairLit!(0, 1),
            PairLit!(0, 2),
            PairLit!(1, 1),
            PairLit!(1, 2),
            PairLit!(2, 1),
            PairLit!(2, 2),
        ]);
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
        assert_eq!(*s.nth(0), PairLit!(0, 1));
        assert_eq!(*s.nth(1), PairLit!(1, 1));
    }

    #[test]
    fn test_points2d_iterator_in_order() {
        let s = points2d_tab_flat(4);
        let collected: Vec<Pair<N, N>> = s.iter().copied().collect();
        let expect = vec![
            PairLit!(0, 1),
            PairLit!(0, 2),
            PairLit!(0, 3),
            PairLit!(1, 1),
            PairLit!(1, 2),
            PairLit!(1, 3),
            PairLit!(2, 1),
            PairLit!(2, 2),
            PairLit!(2, 3),
            PairLit!(3, 1),
            PairLit!(3, 2),
            PairLit!(3, 3),
        ];
        assert_eq!(collected, expect);
    }

    #[test]
    fn test_points2d_debug_shape() {
        let s = points2d_tab_flat(3);
        let dbg_str = format!("{:?}", s);
        assert!(!dbg_str.is_empty());
    }

