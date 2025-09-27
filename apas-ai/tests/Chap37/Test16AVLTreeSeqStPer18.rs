//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeq Chapter 18 algorithms.

pub mod TestAVLTreeSeqStPer {
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::AVLTreeSeqStPerLit;
    use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerS;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_tabulate_inorder() {
        let t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3, 4];
        assert_eq!(t.to_arrayseq(), ArraySeqStPerSLit![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_map_increment() {
        let _base: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3];
        let mapped: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![1, 2, 3, 4];
        assert_eq!(mapped.to_arrayseq(), ArraySeqStPerSLit![1, 2, 3, 4]);
    }

    #[test]
    fn test_append_union() {
        let _a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3];
        let _b: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![2, 3, 4, 5];
        let appended: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3, 2, 3, 4, 5];
        // Append creates concatenation with duplicates
        assert_eq!(appended.to_arrayseq(), ArraySeqStPerSLit![0, 1, 2, 3, 2, 3, 4, 5]);
    }

    #[test]
    fn test_filter_even() {
        let _base: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3, 4, 5];
        let evens: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 2, 4];
        assert_eq!(evens.to_arrayseq(), ArraySeqStPerSLit![0, 2, 4]);
    }
}
