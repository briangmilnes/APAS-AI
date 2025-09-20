pub mod TestTypes {
    use apas_ai::Types::Types::*;

    #[test]
    fn test_boolean_eq_neq_and_ordering() {
        assert_eq!(B::True, B::True);
        assert_ne!(B::True, B::False);
        assert_ne!(B::False, B::True);
        assert_eq!(B::False, B::False);

        assert!(B::True < B::False);
    }

    #[test]
    fn test_ordering_on_n_naturals() {
        let a: N = 0;
        let b: N = 1;
        assert!(matches!(a.cmp(&b), O::Less));
        assert!(matches!(b.cmp(&a), O::Greater));
        assert!(matches!(a.cmp(&a), O::Equal));
    }

    #[test]
    fn test_cmp_on_b_returns_expected_ordering_variants() {
        assert!(matches!(B::True.cmp(&B::False), O::Less));
        assert!(matches!(B::False.cmp(&B::True), O::Greater));
        assert!(matches!(B::True.cmp(&B::True), O::Equal));
        assert!(matches!(B::False.cmp(&B::False), O::Equal));
    }

    #[test]
    fn test_btree_set_orders_b_true_before_false() {
        let mut set = std::collections::BTreeSet::new();
        set.insert(B::False);
        set.insert(B::True);

        let ordered: Vec<B> = set.into_iter().collect();
        // Note: vec![B::True, B::False] kept as-is since it's a simple test assertion
        // and there's no BLit! macro for Boolean enum values
        assert_eq!(ordered, vec![B::True, B::False]);
    }

    #[test]
    fn test_n_aliases_usize_and_cmp_examples() {
        assert_eq!(std::mem::size_of::<N>(), std::mem::size_of::<usize>());

        let x: N = 2;
        let y: N = 5;
        assert!(matches!(x.cmp(&y), O::Less));
        assert!(matches!(y.cmp(&x), O::Greater));
        assert!(matches!(x.cmp(&x), O::Equal));

        let max_n: N = usize::MAX;
        let zero_n: N = 0;
        assert!(matches!(zero_n.cmp(&max_n), O::Less));
    }

    #[test]
    fn test_debug_format_for_b_variants() {
        assert_eq!(format!("{:?}", B::True), "True");
        assert_eq!(format!("{:?}", B::False), "False");
    }

    #[test]
    fn test_display_format_for_b_variants() {
        assert_eq!(format!("{}", B::True), "True");
        assert_eq!(format!("{}", B::False), "False");
    }
}
