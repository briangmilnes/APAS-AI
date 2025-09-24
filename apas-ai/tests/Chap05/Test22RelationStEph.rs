//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestRelationStEphChap5_2 {
    use apas_ai::Chap05::RelationStEph::RelationStEph::*;
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*; // macro import
    use apas_ai::{PairLit, RelationLit};

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
