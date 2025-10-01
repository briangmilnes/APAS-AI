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
        assert_eq!(a.partition(&p), true);
    }

    #[test]
    fn test_partition_example_5_2_false_due_to_overlap() {
        let a: Set<N> = SetLit![1, 2, 3, 4, 5, 6];
        let odd_with_6: Set<N> = SetLit![1, 3, 5, 6];
        let even_with_6: Set<N> = SetLit![2, 4, 6];
        let q: Set<Set<N>> = SetLit![odd_with_6, even_with_6];
        assert_eq!(a.partition(&q), false);
    }

    #[test]
    fn test_partition_false_due_to_missing_element() {
        let a: Set<N> = SetLit![1, 2, 3];
        let s1: Set<N> = SetLit![1];
        let s2: Set<N> = SetLit![2];
        let parts: Set<Set<N>> = SetLit![s1, s2];
        assert_eq!(a.partition(&parts), false);
    }

    #[test]
    fn test_set_empty() {
        let empty_set: Set<i32> = Set::empty();
        assert_eq!(empty_set.size(), 0);
        assert_eq!(empty_set.mem(&42), false);
    }

    #[test]
    fn test_set_singleton() {
        let single_set = Set::singleton(42);
        assert_eq!(single_set.size(), 1);
        assert_eq!(single_set.mem(&42), true);
        assert_eq!(single_set.mem(&99), false);
    }

    #[test]
    fn test_set_size_comprehensive() {
        let empty: Set<i32> = Set::empty();
        assert_eq!(empty.size(), 0);

        let single = Set::singleton(1);
        assert_eq!(single.size(), 1);

        let multi = SetLit![1, 2, 3, 4, 5];
        assert_eq!(multi.size(), 5);
    }

    #[test]
    fn test_set_mem_comprehensive() {
        let set = SetLit![1, 2, 3];
        assert_eq!(set.mem(&1), true);
        assert_eq!(set.mem(&2), true);
        assert_eq!(set.mem(&3), true);
        assert_eq!(set.mem(&4), false);
        assert_eq!(set.mem(&0), false);
    }

    #[test]
    fn test_set_union() {
        let set1 = SetLit![1, 2, 3];
        let set2 = SetLit![3, 4, 5];
        let union_set = set1.union(&set2);

        assert_eq!(union_set.size(), 5);
        assert_eq!(union_set.mem(&1), true);
        assert_eq!(union_set.mem(&2), true);
        assert_eq!(union_set.mem(&3), true);
        assert_eq!(union_set.mem(&4), true);
        assert_eq!(union_set.mem(&5), true);
        assert_eq!(union_set.mem(&6), false);
    }

    #[test]
    fn test_set_intersection() {
        let set1 = SetLit![1, 2, 3, 4];
        let set2 = SetLit![3, 4, 5, 6];
        let intersect_set = set1.intersection(&set2);

        assert_eq!(intersect_set.size(), 2);
        assert_eq!(intersect_set.mem(&3), true);
        assert_eq!(intersect_set.mem(&4), true);
        assert_eq!(intersect_set.mem(&1), false);
        assert_eq!(intersect_set.mem(&5), false);
    }

    #[test]
    fn test_set_insert() {
        let mut set = Set::empty();
        set.insert(1);
        set.insert(2);
        set.insert(1); // duplicate

        assert_eq!(set.size(), 2);
        assert_eq!(set.mem(&1), true);
        assert_eq!(set.mem(&2), true);
    }

    #[test]
    fn test_set_iter() {
        let set = SetLit![1, 2, 3];
        let mut collected: Vec<i32> = set.iter().cloned().collect();
        collected.sort(); // HashSet order is not guaranteed

        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_set_fromvec() {
        let vec_data = vec![1, 2, 3, 2, 1]; // with duplicates
        let set = Set::FromVec(vec_data);

        assert_eq!(set.size(), 3);
        assert_eq!(set.mem(&1), true);
        assert_eq!(set.mem(&2), true);
        assert_eq!(set.mem(&3), true);
    }

    #[test]
    fn test_cartesian_product_empty_edge() {
        let empty_set: Set<i32> = Set::empty();
        let normal_set = SetLit![1, 2];

        let prod1 = empty_set.CartesianProduct(&normal_set);
        assert_eq!(prod1.size(), 0);

        let prod2 = normal_set.CartesianProduct(&empty_set);
        assert_eq!(prod2.size(), 0);
    }

    #[test]
    fn test_setlit_macro_direct() {
        let empty: Set<i32> = SetLit![];
        assert_eq!(empty.size(), 0);

        let single = SetLit![42];
        assert_eq!(single.size(), 1);
        assert_eq!(single.mem(&42), true);

        let multi = SetLit![1, 2, 3];
        assert_eq!(multi.size(), 3);
    }

    #[test]
    fn test_empty_set_union() {
        let empty: Set<i32> = Set::empty();
        let normal = SetLit![1, 2, 3];

        let union1 = empty.union(&normal);
        assert_eq!(union1.size(), 3);

        let union2 = normal.union(&empty);
        assert_eq!(union2.size(), 3);

        let union_empty = empty.union(&empty);
        assert_eq!(union_empty.size(), 0);
    }

    #[test]
    fn test_empty_set_intersection() {
        let empty: Set<i32> = Set::empty();
        let normal = SetLit![1, 2, 3];

        let intersect1 = empty.intersection(&normal);
        assert_eq!(intersect1.size(), 0);

        let intersect2 = normal.intersection(&empty);
        assert_eq!(intersect2.size(), 0);

        let intersect_empty = empty.intersection(&empty);
        assert_eq!(intersect_empty.size(), 0);
    }

    #[test]
    fn test_set_large_operations_stress() {
        // Test with large sets to verify no panics occur
        let large_vec: Vec<i32> = (0..10000).collect();
        let large_set = Set::FromVec(large_vec);

        assert_eq!(large_set.size(), 10000);
        assert_eq!(large_set.mem(&5000), true);
        assert_eq!(large_set.mem(&15000), false);

        // Test union with another large set
        let large_vec2: Vec<i32> = (5000..15000).collect();
        let large_set2 = Set::FromVec(large_vec2);

        let union_result = large_set.union(&large_set2);
        assert_eq!(union_result.size(), 15000); // 0-4999 + 5000-14999 = 15000 unique elements

        // Test intersection
        let intersection_result = large_set.intersection(&large_set2);
        assert_eq!(intersection_result.size(), 5000); // 5000-9999 overlap

        // Verify no panics on extreme operations
        let empty_set: Set<i32> = Set::empty();
        let union_with_empty = large_set.union(&empty_set);
        assert_eq!(union_with_empty.size(), 10000);

        let intersection_with_empty = large_set.intersection(&empty_set);
        assert_eq!(intersection_with_empty.size(), 0);
    }

    #[test]
    fn test_set_single_element_boundary() {
        // Test single element set operations
        let single = Set::singleton(42);
        assert_eq!(single.size(), 1);
        assert_eq!(single.mem(&42), true);
        assert_eq!(single.mem(&43), false);

        // Operations with single element set
        let empty: Set<i32> = Set::empty();
        let union_with_empty = single.union(&empty);
        assert_eq!(union_with_empty.size(), 1);
        assert_eq!(union_with_empty.mem(&42), true);

        let intersection_with_empty = single.intersection(&empty);
        assert_eq!(intersection_with_empty.size(), 0);

        // Union with another single element
        let single2 = Set::singleton(99);
        let union_singles = single.union(&single2);
        assert_eq!(union_singles.size(), 2);
        assert_eq!(union_singles.mem(&42), true);
        assert_eq!(union_singles.mem(&99), true);

        // Intersection with same element
        let single_same = Set::singleton(42);
        let intersection_same = single.intersection(&single_same);
        assert_eq!(intersection_same.size(), 1);
        assert_eq!(intersection_same.mem(&42), true);

        // Intersection with different element
        let intersection_diff = single.intersection(&single2);
        assert_eq!(intersection_diff.size(), 0);

        // Cartesian product with single element
        let single_char = Set::singleton('a');
        let cartesian = single.CartesianProduct(&single_char);
        assert_eq!(cartesian.size(), 1);
        assert_eq!(cartesian.mem(&Pair(42, 'a')), true);

        // Iterator on single element
        let collected: Vec<i32> = single.iter().cloned().collect();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0], 42);
    }

    #[test]
    fn test_set_iterator_boundaries() {
        // Test iterator at beginning/end boundaries
        let set = SetLit![10, 20, 30, 40, 50];

        // Test iterator starting from beginning
        let mut iter = set.iter();
        let first = iter.next();
        assert!(first.is_some()); // Should have first element
        let second = iter.next();
        assert!(second.is_some()); // Should have second element

        // Test iterator ending at end
        let iter_end = set.iter();
        let collected: Vec<i32> = iter_end.cloned().collect();
        assert_eq!(collected.len(), 5);
        // Note: HashSet iteration order is not guaranteed, so we check membership
        for val in &collected {
            assert_eq!(set.mem(val), true);
        }

        // Test iterator on single element - both beginning and end
        let single = Set::singleton(99);
        let mut single_iter = single.iter();
        assert_eq!(single_iter.next(), Some(&99)); // Beginning = end
        assert_eq!(single_iter.next(), None); // Past end

        // Test iterator on empty set - no boundaries
        let empty: Set<i32> = Set::empty();
        let mut empty_iter = empty.iter();
        assert_eq!(empty_iter.next(), None); // No beginning

        // Test iterator exhaustion and reset
        let set_reset = SetLit![1, 2];
        let mut iter1 = set_reset.iter();
        // Exhaust iterator by consuming all elements
        let first_elem = iter1.next();
        let second_elem = iter1.next();
        assert!(first_elem.is_some());
        assert!(second_elem.is_some());
        assert_eq!(iter1.next(), None); // Should be exhausted

        // New iterator should start fresh at beginning
        let mut iter2 = set_reset.iter();
        let fresh_first = iter2.next();
        assert!(fresh_first.is_some()); // Fresh start at beginning

        // Test iterator with functional operations at boundaries
        let set_func = SetLit![100, 200, 300];

        // First element via iterator (order not guaranteed)
        let first = set_func.iter().next();
        assert!(first.is_some());
        assert_eq!(set_func.mem(first.unwrap()), true);

        // Count elements via iterator
        let count = set_func.iter().count();
        assert_eq!(count, 3);

        // Test iterator chain boundaries
        let set1 = SetLit![1, 2];
        let set2 = SetLit![3, 4];
        let chained: Vec<i32> = set1.iter().chain(set2.iter()).cloned().collect();
        assert_eq!(chained.len(), 4);
        // Check all elements are present
        for val in &chained {
            assert!(set1.mem(val) == true || set2.mem(val) == true);
        }

        // Test iterator skip/take boundaries
        let set_skip = SetLit![10, 20, 30, 40, 50];
        let skipped: Vec<i32> = set_skip.iter().skip(2).cloned().collect();
        assert_eq!(skipped.len(), 3);
        // All skipped elements should be in original set
        for val in &skipped {
            assert_eq!(set_skip.mem(val), true);
        }

        let taken: Vec<i32> = set_skip.iter().take(3).cloned().collect();
        assert_eq!(taken.len(), 3);
        // All taken elements should be in original set
        for val in &taken {
            assert_eq!(set_skip.mem(val), true);
        }

        // Test iterator collect and verify completeness
        let original = SetLit![1, 2, 3, 4, 5];
        let collected_all: Vec<i32> = original.iter().cloned().collect();
        assert_eq!(collected_all.len(), 5);

        // Create new set from collected elements and verify equality
        let reconstructed = Set::FromVec(collected_all);
        assert_eq!(reconstructed.size(), original.size());
        for i in 1..=5 {
            assert_eq!(original.mem(&i), reconstructed.mem(&i));
        }
    }

    #[test]
    fn test_set_maximum_size_boundary() {
        // Test large set boundary (reduced from 100k to 20k for faster testing)
        let large_size = 20_000usize;
        let large_vec: Vec<i32> = (0..large_size as i32).collect();
        let large_set = Set::FromVec(large_vec);

        // Verify basic operations work on large set
        assert_eq!(large_set.size(), large_size);
        assert_eq!(large_set.mem(&0), true);
        assert_eq!(large_set.mem(&((large_size - 1) as i32)), true);
        assert_eq!(large_set.mem(&(large_size as i32)), false);

        // Test operations on large set
        let empty_set: Set<i32> = Set::empty();
        let union_with_empty = large_set.union(&empty_set);
        assert_eq!(union_with_empty.size(), large_size);

        let intersection_with_empty = large_set.intersection(&empty_set);
        assert_eq!(intersection_with_empty.size(), 0);

        // Test with another large set
        let large_vec2: Vec<i32> = (10_000..30_000).collect();
        let large_set2 = Set::FromVec(large_vec2);

        let union_large = large_set.union(&large_set2);
        assert_eq!(union_large.size(), 30_000); // 0-9999 + 10000-29999 = 30000 unique

        let intersection_large = large_set.intersection(&large_set2);
        assert_eq!(intersection_large.size(), 10_000); // 10000-19999 overlap

        // Test iterator on large set (sample check)
        let mut count = 0;
        for val in large_set.iter() {
            if count < 10 {
                assert_eq!(large_set.mem(val), true);
            }
            count += 1;
            if count > large_size + 100 {
                // Safety check
                break;
            }
        }
        assert_eq!(count, large_size);

        // Test Cartesian product with small set to avoid explosion
        let small_set = Set::singleton('a');
        let cartesian_large = large_set.CartesianProduct(&small_set);
        assert_eq!(cartesian_large.size(), large_size);
        assert_eq!(cartesian_large.mem(&Pair(0, 'a')), true);
        assert_eq!(cartesian_large.mem(&Pair((large_size - 1) as i32, 'a')), true);
    }
}
