//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPer multithreaded algorithms.



    use std::sync::Mutex;

    use apas_ai::ArrayMtPerSLit;
    use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS;
    use apas_ai::Chap19::ArraySeqMtPer::ArraySeqMtPer::*;
    use apas_ai::PairLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_arraymtperslit_macro_functionality() {
        // Test empty sequence creation
        let empty: ArraySeqMtPerS<i32> = ArrayMtPerSLit![];
        assert_eq!(empty.length(), 0);
        
        // Test sequence creation with elements
        let with_data: ArraySeqMtPerS<i32> = ArrayMtPerSLit![1, 2, 3];
        assert_eq!(with_data.length(), 3);
        assert_eq!(*with_data.nth(0), 1);
        assert_eq!(*with_data.nth(1), 2);
        assert_eq!(*with_data.nth(2), 3);
    }

    #[test]
    fn test_inject_basic() {
        let values = ArrayMtPerSLit![0, 1, 2, 3, 4, 5];
        let changes = ArrayMtPerSLit![PairLit!(2, 99), PairLit!(4, 88)];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 6);
        // inject should apply first occurrence of each index
        assert_eq!(*result.nth(0), 0);
        assert_eq!(*result.nth(1), 1);
        assert_eq!(*result.nth(2), 99);
        assert_eq!(*result.nth(3), 3);
        assert_eq!(*result.nth(4), 88);
        assert_eq!(*result.nth(5), 5);
    }

    #[test]
    fn test_inject_conflicting_updates() {
        let values = ArrayMtPerSLit![0, 1, 2, 3, 4, 5];
        let changes = ArrayMtPerSLit![PairLit!(2, 99), PairLit!(2, 77), PairLit!(4, 88)];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 6);
        // inject takes FIRST update for each index (99 for index 2)
        assert_eq!(*result.nth(0), 0);
        assert_eq!(*result.nth(1), 1);
        assert_eq!(*result.nth(2), 99); // First update wins
        assert_eq!(*result.nth(3), 3);
        assert_eq!(*result.nth(4), 88);
        assert_eq!(*result.nth(5), 5);
    }

    #[test]
    fn test_inject_out_of_bounds() {
        let values = ArrayMtPerSLit![0, 1, 2];
        let changes = ArrayMtPerSLit![PairLit!(1, 99), PairLit!(5, 77)]; // index 5 is out of bounds
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), 0);
        assert_eq!(*result.nth(1), 99); // Valid update applied
        assert_eq!(*result.nth(2), 2); // Out of bounds update ignored
    }

    #[test]
    fn test_inject_empty_changes() {
        let values = ArrayMtPerSLit![1, 2, 3];
        let changes: ArraySeqMtPerS<Pair<N, N>> = ArrayMtPerSLit![];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), 1);
        assert_eq!(*result.nth(1), 2);
        assert_eq!(*result.nth(2), 3);
    }

    #[test]
    fn test_inject_empty_values() {
        let values: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
        let changes = ArrayMtPerSLit![PairLit!(0, 99)];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 0); // No values to update
    }

    // TODO: AtomicWriteLowestChangeNumberWins tests require complex setup
    // The function signature expects values_with_change_number and changes arrays
    // to have specific relationships that need more investigation

    // Mutex test removed - Mutex doesn't implement required traits (Clone, Eq, Display)
    // atomicWrite method requires types that implement these traits
    // #[test]
    // fn test_atomic_write_migrated_from_st_test() {
    //     // Invalid test - Mutex doesn't implement required traits
    // }

    #[test]
    fn test_inject_string_values() {
        let values = ArrayMtPerSLit!["hello", "world", "test"];
        let changes = ArrayMtPerSLit![PairLit!(1, "rust"), PairLit!(0, "hi")];
        let result = <ArraySeqMtPerS<&str> as ArraySeqMtPerTrait<&str>>::inject(&values, &changes);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), "hi");
        assert_eq!(*result.nth(1), "rust");
        assert_eq!(*result.nth(2), "test");
    }

    #[test]
    fn test_tabulate_basic() {
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::tabulate(&|i| i * 2, 5);
        assert_eq!(result.length(), 5);
        assert_eq!(*result.nth(0), 0);
        assert_eq!(*result.nth(1), 2);
        assert_eq!(*result.nth(4), 8);
    }

    #[test]
    fn test_map_basic() {
        let seq = ArrayMtPerSLit![1, 2, 3, 4];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::map(&seq, |x| x * x);
        assert_eq!(result.length(), 4);
        assert_eq!(*result.nth(0), 1);
        assert_eq!(*result.nth(1), 4);
        assert_eq!(*result.nth(3), 16);
    }

    #[test]
    fn test_filter_basic() {
        let seq = ArrayMtPerSLit![1, 2, 3, 4, 5, 6];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::filter(&seq, |x| *x % 2 == 0);
        assert_eq!(result.length(), 3);
    }

    #[test]
    fn test_reduce_basic() {
        let seq = ArrayMtPerSLit![1, 2, 3, 4, 5];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::reduce(&seq, |a, b| a + b, 0);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_scan_basic() {
        let seq = ArrayMtPerSLit![1, 2, 3, 4];
        let (result, total) = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::scan(&seq, &|a, b| a + b, 0);
        assert_eq!(result.length(), 4);
        assert_eq!(total, 10);
    }

    #[test]
    fn test_append_basic() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let b = ArrayMtPerSLit![4, 5, 6];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::append(&a, &b);
        assert_eq!(result.length(), 6);
        assert_eq!(*result.nth(0), 1);
        assert_eq!(*result.nth(5), 6);
    }

    #[test]
    fn test_flatten_basic() {
        let a = ArrayMtPerSLit![1, 2];
        let b = ArrayMtPerSLit![3, 4];
        let c = ArrayMtPerSLit![5, 6];
        let nested = ArrayMtPerSLit![a, b, c];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::flatten(&nested);
        assert_eq!(result.length(), 6);
    }

    #[test]
    fn test_isEmpty() {
        let empty: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
        let non_empty = ArrayMtPerSLit![1];
        assert!(<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isEmpty(&empty));
        assert!(!<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isEmpty(&non_empty));
    }

    #[test]
    fn test_isSingleton() {
        let empty: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
        let single = ArrayMtPerSLit![42];
        let multi = ArrayMtPerSLit![1, 2];
        assert!(!<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isSingleton(&empty));
        assert!(<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isSingleton(&single));
        assert!(!<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isSingleton(&multi));
    }

    #[test]
    fn test_append_select() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let b = ArrayMtPerSLit![4, 5];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::append_select(&a, &b);
        assert_eq!(result.length(), 5);
    }

    #[test]
    fn test_select() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let b = ArrayMtPerSLit![4, 5];
        assert_eq!(*<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::select(&a, &b, 0).unwrap(), 1);
        assert_eq!(*<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::select(&a, &b, 3).unwrap(), 4);
    }

    #[test]
    fn test_iterate_basic() {
        let seq = ArrayMtPerSLit![1, 2, 3, 4];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::iterate(&seq, &|acc, x| acc + x, 0);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_ninject_basic() {
        let seq = ArrayMtPerSLit![0, 1, 2, 3, 4];
        let updates = ArrayMtPerSLit![PairLit!(1, 99), PairLit!(3, 88)];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::ninject(&seq, &updates);
        assert_eq!(result.length(), 5);
        assert_eq!(*result.nth(1), 99);
        assert_eq!(*result.nth(3), 88);
    }

    #[test]
    fn test_update_single() {
        let seq = ArrayMtPerSLit![0, 1, 2, 3];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::update_single(&seq, 2, 99);
        assert_eq!(*result.nth(2), 99);
        assert_eq!(*result.nth(0), 0);
    }

    #[test]
    fn test_large_tabulate() {
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::tabulate(&|i| i, 1000);
        assert_eq!(result.length(), 1000);
        assert_eq!(*result.nth(500), 500);
    }

    #[test]
    fn test_large_reduce() {
        let seq = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::tabulate(&|i| i + 1, 100);
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::reduce(&seq, |a, b| a + b, 0);
        assert_eq!(result, 5050);
    }

    #[test]
    fn test_parallel_map_large() {
        let seq = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::tabulate(&|i| i, 100);
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::map(&seq, |x| x * 2);
        assert_eq!(*result.nth(50), 100);
    }

    #[test]
    fn test_parallel_filter_large() {
        let seq = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::tabulate(&|i| i, 100);
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::filter(&seq, |x| *x % 2 == 0);
        assert_eq!(result.length(), 50);
    }

