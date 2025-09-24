//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestSetStEphChap5_1 {

    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::PairLit;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*; // macro import

    #[allow(dead_code)]
    fn macro_typecheck_exercise() {
        let _empty: Set<&'static str> = SetLit![];
        let _one = SetLit!["only"];
        let _many = SetLit!["a", "b", "c"];
    }

    #[test]
    fn test_cartesian_product_example_5_1() {
        let a: Set<N> = SetLit![0, 1, 2, 3];
        let b: Set<char> = SetLit!['a', 'b'];
        let prod = a.CartesianProduct(&b);

        let expect: Set<Pair<N, char>> = SetLit![
            PairLit!(0, 'a'),
            PairLit!(0, 'b'),
            PairLit!(1, 'a'),
            PairLit!(1, 'b'),
            PairLit!(2, 'a'),
            PairLit!(2, 'b'),
            PairLit!(3, 'a'),
            PairLit!(3, 'b')
        ];
        assert_eq!(prod, expect);
        assert_eq!(prod.size(), 8);
    }

    #[test]
    fn test_partition_example_5_2_true() {
        let a: Set<N> = SetLit![1, 2, 3, 4, 5, 6];
        let odd: Set<N> = SetLit![1, 3, 5];
        let even: Set<N> = SetLit![2, 4, 6];
        let p: Set<Set<N>> = SetLit![odd, even];
        assert_eq!(a.partition(&p), B::True);
    }

    #[test]
    fn test_partition_example_5_2_false_due_to_overlap() {
        let a: Set<N> = SetLit![1, 2, 3, 4, 5, 6];
        let odd_with_6: Set<N> = SetLit![1, 3, 5, 6];
        let even_with_6: Set<N> = SetLit![2, 4, 6];
        let q: Set<Set<N>> = SetLit![odd_with_6, even_with_6];
        assert_eq!(a.partition(&q), B::False);
    }

    #[test]
    fn test_partition_false_due_to_missing_element() {
        let a: Set<N> = SetLit![1, 2, 3];
        let s1: Set<N> = SetLit![1];
        let s2: Set<N> = SetLit![2];
        let parts: Set<Set<N>> = SetLit![s1, s2];
        assert_eq!(a.partition(&parts), B::False);
    }
}
