//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeq Chapter 18 algorithms.

pub mod TestAVLTreeSeqStPer {
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerS;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_tabulate_inorder() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3, 4]);
        assert_eq!(t.to_arrayseq(), ArraySeqStPerSLit![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_map_increment() {
        let base: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3]);
        let mapped: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(mapped.to_arrayseq(), ArraySeqStPerSLit![1, 2, 3, 4]);
    }

    #[test]
    fn test_append_union() {
        let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3]);
        let b: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![2, 3, 4, 5]);
        let u: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3, 2, 3, 4, 5]);
        assert_eq!(u.to_arrayseq(), ArraySeqStPerSLit![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_filter_even() {
        let base: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 1, 2, 3, 4, 5]);
        let evens: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::from_vec(vec![0, 2, 4]);
        assert_eq!(evens.to_arrayseq(), ArraySeqStPerSLit![0, 2, 4]);
    }
}
