//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtEphChap18 multithreaded ephemeral algorithms.

pub mod Test28ArraySeqMtEphChap {
    use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_new_and_set() {
        let mut a: ArraySeqMtEphS<N> = ArraySeqMtEphS::new(5, 0);
        assert_eq!(a.length(), 5);
        
        // Test set method
        assert!(a.set(0, 10).is_ok());
        assert!(a.set(4, 40).is_ok());
        assert!(a.set(5, 50).is_err()); // Out of bounds
        
        assert_eq!(a.nth_cloned(0), 10);
        assert_eq!(a.nth_cloned(4), 40);
    }

    #[test]
    fn test_tabulate_basic() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i * 2, 5);
        assert_eq!(a.length(), 5);
        
        for i in 0..5 {
            assert_eq!(a.nth_cloned(i), i * 2);
        }
    }

    #[test]
    fn test_map_parallel() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i, 2000); // Large enough for parallel
        let doubled: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::map(&a, |x| x * 2);
        
        assert_eq!(doubled.length(), 2000);
        for i in 0..2000 {
            assert_eq!(doubled.nth_cloned(i), i * 2);
        }
    }

    #[test]
    fn test_reduce_parallel() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i + 1, 2000); // [1, 2, 3, ..., 2000]
        let sum = ArraySeqMtEphTrait::reduce(&a, &|x, y| x + y, 0);
        
        // Sum of 1 to 2000 = 2000 * 2001 / 2 = 2001000
        assert_eq!(sum, 2001000);
    }

    #[test]
    fn test_filter() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i, 10);
        let evens = ArraySeqMtEphTrait::filter(&a, |x| if x % 2 == 0 { B::True } else { B::False });
        
        assert_eq!(evens.length(), 5); // 0, 2, 4, 6, 8
        for i in 0..5 {
            assert_eq!(evens.nth_cloned(i), i * 2);
        }
    }

    #[test]
    fn test_append() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i, 3);
        let b: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i + 10, 2);
        let combined = ArraySeqMtEphTrait::append(&a, &b);
        
        assert_eq!(combined.length(), 5);
        assert_eq!(combined.nth_cloned(0), 0);
        assert_eq!(combined.nth_cloned(1), 1);
        assert_eq!(combined.nth_cloned(2), 2);
        assert_eq!(combined.nth_cloned(3), 10);
        assert_eq!(combined.nth_cloned(4), 11);
    }

    #[test]
    fn test_flatten() {
        let inner1: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i, 2);
        let inner2: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i + 10, 3);
        let outer = ArraySeqMtEphS::from_vec(vec![inner1, inner2]);
        
        let flattened = ArraySeqMtEphTrait::flatten(&outer);
        assert_eq!(flattened.length(), 5);
        assert_eq!(flattened.nth_cloned(0), 0);
        assert_eq!(flattened.nth_cloned(1), 1);
        assert_eq!(flattened.nth_cloned(2), 10);
        assert_eq!(flattened.nth_cloned(3), 11);
        assert_eq!(flattened.nth_cloned(4), 12);
    }

    #[test]
    fn test_scan() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i + 1, 5); // [1, 2, 3, 4, 5]
        let (prefixes, final_sum) = ArraySeqMtEphTrait::scan(&a, &|x, y| x + y, 0);
        
        assert_eq!(prefixes.length(), 5);
        assert_eq!(prefixes.nth_cloned(0), 1);  // 0 + 1
        assert_eq!(prefixes.nth_cloned(1), 3);  // 1 + 2
        assert_eq!(prefixes.nth_cloned(2), 6);  // 3 + 3
        assert_eq!(prefixes.nth_cloned(3), 10); // 6 + 4
        assert_eq!(prefixes.nth_cloned(4), 15); // 10 + 5
        assert_eq!(final_sum, 15);
    }

    #[test]
    fn test_iterate() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i + 1, 5); // [1, 2, 3, 4, 5]
        let sum = ArraySeqMtEphTrait::iterate(&a, |acc, x| acc + x, 0);
        assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
    }

    #[test]
    fn test_collect() {
        let pairs = ArraySeqMtEphS::from_vec(vec![
            Pair("a".to_string(), 1),
            Pair("b".to_string(), 2),
            Pair("a".to_string(), 3),
            Pair("c".to_string(), 4),
            Pair("b".to_string(), 5),
        ]);
        
        let grouped = ArraySeqMtEphTrait::collect(&pairs, |x, y| {
            if x == y { O::Equal } else { O::Less }
        });
        
        assert_eq!(grouped.length(), 3); // Three unique keys: a, b, c
    }

    #[test]
    fn test_update() {
        let mut a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i, 5);
        let updated = ArraySeqMtEphTrait::update(&mut a, (2, 99));
        
        assert_eq!(updated.nth_cloned(2), 99);
        assert_eq!(updated.nth_cloned(0), 0); // Others unchanged
        assert_eq!(updated.nth_cloned(4), 4);
    }

    #[test]
    fn test_inject() {
        let a: ArraySeqMtEphS<N> = ArraySeqMtEphTrait::tabulate(|i| i, 5);
        let updates = ArraySeqMtEphS::from_vec(vec![
            Pair(1, 100),
            Pair(3, 300),
            Pair(1, 111), // Duplicate index - first wins
        ]);
        
        let injected = ArraySeqMtEphTrait::inject(&a, &updates);
        assert_eq!(injected.nth_cloned(0), 0);   // Unchanged
        assert_eq!(injected.nth_cloned(1), 100); // Updated (first occurrence)
        assert_eq!(injected.nth_cloned(2), 2);   // Unchanged
        assert_eq!(injected.nth_cloned(3), 300); // Updated
        assert_eq!(injected.nth_cloned(4), 4);   // Unchanged
    }

    #[test]
    fn test_empty_and_singleton() {
        let empty: ArraySeqMtEphS<N> = ArraySeqMtEphS::empty();
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), B::True);
        assert_eq!(empty.isSingleton(), B::False);
        
        let single: ArraySeqMtEphS<N> = ArraySeqMtEphS::singleton(42);
        assert_eq!(single.length(), 1);
        assert_eq!(single.isEmpty(), B::False);
        assert_eq!(single.isSingleton(), B::True);
        assert_eq!(single.nth_cloned(0), 42);
    }
}
