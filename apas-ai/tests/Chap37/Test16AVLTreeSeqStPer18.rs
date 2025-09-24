//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeq Chapter 18 algorithms.

pub mod TestAVLTreeSeqStPer {
    use apas_ai::ArraySeqStPer; // macro import
    use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerS;
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_tabulate_inorder() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::tabulate(|i| i, 5);
        assert_eq!(t.to_arrayseq(), ArraySeqStPerSLit![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_map_increment() {
        let base: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::tabulate(|i| i, 4);
        let mapped: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::map(&base, |x| x + 1);
        assert_eq!(mapped.to_arrayseq(), ArraySeqStPerSLit![1, 2, 3, 4]);
    }

    #[test]
    fn test_append_union() {
        let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::tabulate(|i| i, 4); // 0..3
        let b: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::tabulate(|i| i + 2, 4); // 2..5
        let u: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::append(&a, &b);
        assert_eq!(u.to_arrayseq(), ArraySeqStPerSLit![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_filter_even() {
        let base: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::tabulate(|i| i, 6);
        let evens: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::filter(&base, |x| {
            if x % 2 == 0 { B::True } else { B::False }
        });
        assert_eq!(evens.to_arrayseq(), ArraySeqStPerSLit![0, 2, 4]);
    }
}
