pub mod TestAVLTreeSeqEph {
    use apas_ai::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS;
    use apas_ai::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait as AVLTreeSeqEph;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_avl_ephemeral_basic() {
        let mut t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::new();
        assert_eq!(t.length(), 0);
        t.push_back(1);
        t.push_back(2);
        t.push_back(3);
        assert_eq!(t.length(), 3);
        assert_eq!(*t.nth(1), 2);
        let _ = t.set(1, 9).unwrap();
        assert_eq!(*t.nth(1), 9);
    }
}
