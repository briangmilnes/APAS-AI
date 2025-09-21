pub mod TestAVLTreeSeqPer {
    use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::AVLTreeSeqStPerChap18::AVLTreeSeqStPerChap18::*;
    use apas_ai::AVLTreeSeqStPerSLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_persistent_set_does_not_mutate() {
        let _t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerSLit![7];
        let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerSLit![0, 1, 2, 3]; // tabulate(|i| i, 4)
        let b = a.set(1, 99).unwrap();
        assert_eq!(*a.nth(1), 1);
        assert_eq!(*b.nth(1), 99);
    }

    #[test]
    fn test_iterator_inorder_values() {
        let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerSLit![1, 2, 3, 4, 5]; // tabulate(|i| i + 1, 5)
        let vals: Vec<N> = a.iter().map(|x| *x).collect();
        assert_eq!(vals, vec![1, 2, 3, 4, 5]);
    }
}
