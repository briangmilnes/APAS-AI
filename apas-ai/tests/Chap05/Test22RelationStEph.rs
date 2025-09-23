pub mod TestRelationStEphChap5_2 {
    use apas_ai::Chap5::RelationStEphChap5_2::RelationStEphChap5_2::*;
    use apas_ai::SetLit;
    use apas_ai::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use apas_ai::Types::Types::*; // macro import
    use apas_ai::{RelationLit, PairLit};

    #[test]
    fn test_relation_domain_range_and_mem() {
        // R subset of A x B
        let pairs = SetLit![PairLit!(1usize, 'a'), PairLit!(2usize, 'b'), PairLit!(1usize, 'b')];
        let r = Relation::FromSet(pairs);

        let d = r.domain();
        let e = SetLit![1usize, 2usize];
        assert_eq!(d, e);

        let rg = r.range();
        let rexp = SetLit!['a', 'b'];
        assert_eq!(rg, rexp);

        assert_eq!(r.mem(&1usize, &'a'), B::True);
        assert_eq!(r.mem(&2usize, &'a'), B::False);
    }
}
