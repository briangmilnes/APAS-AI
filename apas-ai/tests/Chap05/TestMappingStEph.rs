//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MappingStEphChap5_5 ephemeral mappings.



    use apas_ai::Chap05::MappingStEph::MappingStEph::*;
    use apas_ai::Chap05::RelationStEph::RelationStEph::*;
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use apas_ai::*;

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
        assert_eq!(m.mem(&1, &"one"), true);
        assert_eq!(m.mem(&2, &"two"), true);
        assert_eq!(m.mem(&3, &"three"), true);
        assert_eq!(m.mem(&1, &"wrong"), false);
        assert_eq!(m.mem(&99, &"one"), false);
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
        assert!(m.mem(&1, &"one") == true || m.mem(&1, &"uno") == true);
        assert_eq!(m.mem(&2, &"two"), true);
    }

    #[test]
    fn test_domain_and_range() {
        let m = MappingLit![(1, "one"), (2, "two"), (3, "one")];

        let domain = m.domain();
        assert_eq!(domain.size(), 3);
        assert_eq!(domain.mem(&1), true);
        assert_eq!(domain.mem(&2), true);
        assert_eq!(domain.mem(&3), true);
        assert_eq!(domain.mem(&4), false);

        let range = m.range();
        assert_eq!(range.size(), 2); // "one" and "two"
        assert_eq!(range.mem(&"one"), true);
        assert_eq!(range.mem(&"two"), true);
        assert_eq!(range.mem(&"three"), false);
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
        assert_eq!(m.mem(&"a", &1), true);
        assert_eq!(m.mem(&"b", &2), true);
        assert_eq!(m.mem(&"c", &3), true);

        // Test wrong key-value combinations
        assert_eq!(m.mem(&"a", &2), false);
        assert_eq!(m.mem(&"b", &3), false);

        // Test non-existent keys/values
        assert_eq!(m.mem(&"d", &1), false);
        assert_eq!(m.mem(&"a", &99), false);
    }

    #[test]
    fn test_empty_mapping_operations() {
        let m: Mapping<N, &str> = MappingLit![];

        assert_eq!(m.size(), 0);
        assert_eq!(m.domain().size(), 0);
        assert_eq!(m.range().size(), 0);
        assert_eq!(m.mem(&1, &"anything"), false);

        let collected: Vec<_> = m.iter().collect();
        assert_eq!(collected.len(), 0);
    }

    #[test]
    fn test_from_relation_empty_edge() {
        // Test FromRelation with empty relation
        let empty_rel: Relation<i32, String> = Relation::empty();
        let m = <Mapping<i32, String> as MappingStEphTrait<i32, String>>::FromRelation(&empty_rel);

        assert_eq!(m.size(), 0);
        assert_eq!(m.domain().size(), 0);
        assert_eq!(m.range().size(), 0);
        assert_eq!(m.mem(&42, &"test".to_string()), false);
    }

    #[test]
    fn test_mapping_extreme_values_graceful() {
        // Test with extreme values to verify no panics occur
        // APAS style: bad arguments produce empty sequences/sets, not panics

        // Test with very large keys
        let large_key = i32::MAX;
        let small_key = i32::MIN;
        let m = MappingLit![(large_key, "max"), (small_key, "min"), (0, "zero")];

        assert_eq!(m.size(), 3);
        assert_eq!(m.mem(&large_key, &"max"), true);
        assert_eq!(m.mem(&small_key, &"min"), true);
        assert_eq!(m.mem(&0, &"zero"), true);

        // Test domain and range operations with extreme values
        let domain = m.domain();
        assert_eq!(domain.size(), 3);
        assert_eq!(domain.mem(&large_key), true);
        assert_eq!(domain.mem(&small_key), true);

        let range = m.range();
        assert_eq!(range.size(), 3);
        assert_eq!(range.mem(&"max"), true);
        assert_eq!(range.mem(&"min"), true);

        // Test with non-existent extreme keys - should return False, not panic
        assert_eq!(m.mem(&(large_key - 1), &"max"), false);
        assert_eq!(m.mem(&(small_key + 1), &"min"), false);
    }

    #[test]
    fn test_mapping_large_dataset_stress() {
        // Test with large mapping to verify no panics occur
        let large_pairs: Vec<Pair<i32, String>> = (0..10000).map(|i| Pair(i, format!("value_{}", i))).collect();

        let m = <Mapping<i32, String> as MappingStEphTrait<i32, String>>::FromVec(large_pairs);

        assert_eq!(m.size(), 10000);
        assert_eq!(m.mem(&5000, &"value_5000".to_string()), true);
        assert_eq!(m.mem(&15000, &"value_15000".to_string()), false);

        // Test domain and range operations on large mapping
        let domain = m.domain();
        assert_eq!(domain.size(), 10000);
        assert_eq!(domain.mem(&9999), true);
        assert_eq!(domain.mem(&10000), false);

        let range = m.range();
        assert_eq!(range.size(), 10000);
        assert_eq!(range.mem(&"value_0".to_string()), true);
        assert_eq!(range.mem(&"value_10000".to_string()), false);

        // Test iteration on large mapping - should not panic
        let mut count = 0;
        for _pair in m.iter() {
            count += 1;
            if count > 10010 {
                // Safety check to prevent infinite loop
                break;
            }
        }
        assert_eq!(count, 10000);
    }

