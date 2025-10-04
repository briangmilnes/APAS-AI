//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::EdgeLit;
use apas_ai::Types::Types::*;

#[test]
fn test_edgelit_macro_functionality() {
    // Test edge creation with two vertices
    let edge: Edge<i32> = EdgeLit![1, 2];
    assert_eq!(edge, Edge(1, 2));
    
    // Test another edge
    let edge2: Edge<i32> = EdgeLit![5, 10];
    assert_eq!(edge2, Edge(5, 10));
}

#[test]
fn test_boolean_eq_neq_and_ordering() {
        assert_eq!(true, true);
        assert_ne!(true, false);
        assert_ne!(false, true);
        assert_eq!(false, false);

        assert!(false < true); // bool ordering: false < true
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
        assert!(matches!(false.cmp(&true), O::Less)); // false < true
        assert!(matches!(true.cmp(&false), O::Greater)); // true > false
        assert!(matches!(true.cmp(&true), O::Equal));
        assert!(matches!(false.cmp(&false), O::Equal));
    }

    #[test]
    fn test_btree_set_orders_b_false_before_true() {
        let mut set = std::collections::BTreeSet::new();
        set.insert(false);
        set.insert(true);

        let ordered: Vec<B> = set.into_iter().collect();
        // bool ordering: false < true, so BTreeSet orders false before true
        assert_eq!(ordered, vec![false, true]);
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
        assert_eq!(format!("{:?}", true), "true"); // bool debug format
        assert_eq!(format!("{:?}", false), "false"); // bool debug format
    }

    #[test]
    fn test_display_format_for_b_variants() {
        assert_eq!(format!("{}", true), "true"); // bool display format
        assert_eq!(format!("{}", false), "false"); // bool display format
    }

