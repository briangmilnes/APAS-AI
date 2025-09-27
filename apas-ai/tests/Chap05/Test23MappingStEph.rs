//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MappingStEphChap5_5 ephemeral mappings.

pub mod Test23MappingStEphChap5_5 {
    use apas_ai::Chap05::MappingStEph::MappingStEph::*;
    use apas_ai::Chap05::RelationStEph::RelationStEph::*;
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use apas_ai::{MappingLit, PairLit};

    #[test]
    fn test_empty_mapping() {
        let m: Mapping<N, &str> = MappingLit![];
        assert_eq!(m.size(), 0);
        assert_eq!(m.domain().size(), 0);
        assert_eq!(m.range().size(), 0);
    }

    #[test]
    fn test_from_vec_basic() {
        let m = MappingLit![(1, "one"), (2, "two"), (3, "three")];
        assert_eq!(m.size(), 3);
        assert_eq!(m.mem(&1, &"one"), B::True);
        assert_eq!(m.mem(&2, &"two"), B::True);
        assert_eq!(m.mem(&3, &"three"), B::True);
        assert_eq!(m.mem(&1, &"wrong"), B::False);
        assert_eq!(m.mem(&99, &"one"), B::False);
    }

    #[test]
    #[should_panic(expected = "MappingLit!: duplicate domain element")]
    fn test_from_vec_duplicate_keys() {
        // Mappings should panic on duplicate domain elements
        let _m = MappingLit![(1, "first"), (2, "two"), (1, "second")];
    }

    #[test]
    fn test_from_relation() {
        let pairs_set = SetLit![PairLit!(1, "one"), PairLit!(2, "two"), PairLit!(1, "uno")];
        let rel = <Relation<N, &str> as RelationStEphTrait<N, &str>>::FromSet(pairs_set);
        let m = <Mapping<N, &str> as MappingStEphTrait<N, &str>>::FromRelation(&rel);

        // Mapping should convert relation to function (one value per key)
        assert!(m.size() <= 2); // At most 2 keys (1 and 2)
        // Either "one" or "uno" for key 1, depending on implementation
        assert!(m.mem(&1, &"one") == B::True || m.mem(&1, &"uno") == B::True);
        assert_eq!(m.mem(&2, &"two"), B::True);
    }

    #[test]
    fn test_domain_and_range() {
        let m = MappingLit![(1, "one"), (2, "two"), (3, "one")];

        let domain = m.domain();
        assert_eq!(domain.size(), 3);
        assert_eq!(domain.mem(&1), B::True);
        assert_eq!(domain.mem(&2), B::True);
        assert_eq!(domain.mem(&3), B::True);
        assert_eq!(domain.mem(&4), B::False);

        let range = m.range();
        assert_eq!(range.size(), 2); // "one" and "two"
        assert_eq!(range.mem(&"one"), B::True);
        assert_eq!(range.mem(&"two"), B::True);
        assert_eq!(range.mem(&"three"), B::False);
    }

    #[test]
    fn test_iter() {
        let m = MappingLit![(1, "one"), (2, "two")];

        let collected: Vec<_> = m.iter().cloned().collect();
        assert_eq!(collected.len(), 2);

        // Check that all expected pairs are present (order may vary)
        assert!(collected.contains(&PairLit!(1, "one")));
        assert!(collected.contains(&PairLit!(2, "two")));
    }

    #[test]
    fn test_mem_comprehensive() {
        let m = MappingLit![("a", 1), ("b", 2), ("c", 3)];

        // Test existing pairs
        assert_eq!(m.mem(&"a", &1), B::True);
        assert_eq!(m.mem(&"b", &2), B::True);
        assert_eq!(m.mem(&"c", &3), B::True);

        // Test wrong key-value combinations
        assert_eq!(m.mem(&"a", &2), B::False);
        assert_eq!(m.mem(&"b", &3), B::False);

        // Test non-existent keys/values
        assert_eq!(m.mem(&"d", &1), B::False);
        assert_eq!(m.mem(&"a", &99), B::False);
    }

    #[test]
    fn test_empty_mapping_operations() {
        let m: Mapping<N, &str> = MappingLit![];

        assert_eq!(m.size(), 0);
        assert_eq!(m.domain().size(), 0);
        assert_eq!(m.range().size(), 0);
        assert_eq!(m.mem(&1, &"anything"), B::False);

        let collected: Vec<_> = m.iter().collect();
        assert_eq!(collected.len(), 0);
    }
}
