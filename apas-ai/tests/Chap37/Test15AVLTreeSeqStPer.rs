//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestAVLTreeSeqPer {
    use apas_ai::AVLTreeSeqStPerLit;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_persistent_set_does_not_mutate() {
        let _t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![7];
        let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3]; // tabulate(&|i| i, 4)
        let b = a.update(1, 99).unwrap();
        assert_eq!(*a.nth(1), 1);
        assert_eq!(*b.nth(1), 99);
    }

    #[test]
    fn test_iterator_inorder_values() {
        let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![1, 2, 3, 4, 5]; // tabulate(&|i| i + 1, 5)
        let vals: Vec<N> = a.iter().map(|x| *x).collect();
        assert_eq!(vals, vec![1, 2, 3, 4, 5]);
    }
}
