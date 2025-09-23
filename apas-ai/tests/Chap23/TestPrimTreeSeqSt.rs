pub mod TestPrimTreeSeqSt {
    use apas_ai::Chap23::PrimTreeSeqSt::PrimTreeSeqSt::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn expose_zero_returns_zero() {
        let seq: PrimTreeSeqStS<N> = PrimTreeSeqStS::empty();
        assert!(matches!(seq.expose(), PrimTreeSeqStTree::Zero));
    }

    #[test]
    fn expose_one_returns_one() {
        let seq = PrimTreeSeqStS::singleton(7);
        match seq.expose() {
            PrimTreeSeqStTree::One(value) => assert_eq!(value, 7),
            other => panic!("expected One variant, got {:?}", other),
        }
    }

    #[test]
    fn expose_two_splits_sequence() {
        let seq = PrimTreeSeqStS::from_vec((0..6).collect());
        match seq.expose() {
            PrimTreeSeqStTree::Two(left, right) => {
                assert!(left.length() > 0);
                assert!(right.length() > 0);
                assert_eq!(left.length() + right.length(), 6);
                assert_eq!(left.as_slice(), &[0, 1, 2]);
                assert_eq!(right.as_slice(), &[3, 4, 5]);
            }
            other => panic!("expected Two variant, got {:?}", other),
        }
    }

    #[test]
    fn join_zero_creates_empty_sequence() {
        let seq: PrimTreeSeqStS<N> = PrimTreeSeqStS::join(PrimTreeSeqStTree::Zero);
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn join_two_concatenates_sequences() {
        let left = PrimTreeSeqStS::from_vec(vec![1, 2]);
        let right = PrimTreeSeqStS::from_vec(vec![3, 4, 5]);
        let joined = PrimTreeSeqStS::join(PrimTreeSeqStTree::Two(left.clone(), right.clone()));
        assert_eq!(joined.as_slice(), &[1, 2, 3, 4, 5]);
        assert_eq!(joined.length(), left.length() + right.length());
    }

    #[test]
    fn expose_then_join_roundtrip() {
        let original = PrimTreeSeqStS::from_vec((1..=9).collect());
        let exposed = original.expose();
        let reconstructed = PrimTreeSeqStS::join(exposed);
        assert_eq!(original.as_slice(), reconstructed.as_slice());
    }
}
