//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPer multithreaded algorithms.

pub mod Test28ArraySeqMtPer {
    use apas_ai::Chap19::ArraySeqMtPer::ArraySeqMtPer::*;
    use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS;
    use apas_ai::ArrayMtPerSLit;
    use apas_ai::PairLit;
    use apas_ai::Types::Types::*;
    use std::sync::Mutex;

    #[test]
    fn test_inject_basic() {
        let values = ArraySeqMtPerSLit![0, 1, 2, 3, 4, 5];
        let changes = ArraySeqMtPerSLit![PairLit!(2, 99), PairLit!(4, 88)];
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
        let values = ArraySeqMtPerSLit![0, 1, 2, 3, 4, 5];
        let changes = ArraySeqMtPerSLit![PairLit!(2, 99), PairLit!(2, 77), PairLit!(4, 88)];
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
        let values = ArraySeqMtPerSLit![0, 1, 2];
        let changes = ArraySeqMtPerSLit![PairLit!(1, 99), PairLit!(5, 77)]; // index 5 is out of bounds
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), 0);
        assert_eq!(*result.nth(1), 99); // Valid update applied
        assert_eq!(*result.nth(2), 2); // Out of bounds update ignored
    }

    #[test]
    fn test_inject_empty_changes() {
        let values = ArraySeqMtPerSLit![1, 2, 3];
        let changes: ArraySeqMtPerS<Pair<N, N>> = ArraySeqMtPerSLit![];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), 1);
        assert_eq!(*result.nth(1), 2);
        assert_eq!(*result.nth(2), 3);
    }

    #[test]
    fn test_inject_empty_values() {
        let values: ArraySeqMtPerS<N> = ArraySeqMtPerSLit![];
        let changes = ArraySeqMtPerSLit![PairLit!(0, 99)];
        let result = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::inject(&values, &changes);

        assert_eq!(result.length(), 0); // No values to update
    }

    // TODO: AtomicWriteLowestChangeNumberWins tests require complex setup
    // The function signature expects values_with_change_number and changes arrays
    // to have specific relationships that need more investigation

    // Migrated from tests/11_TestArraySeqStPerChap19.rs - this was the commented MT code
    #[test]
    fn test_atomic_write_migrated_from_st_test() {
        let values = ArraySeqMtPerSLit![0, 1, 2, 3, 4, 5];
        let changes = ArraySeqMtPerSLit![PairLit!(2, 99), PairLit!(2, 7), PairLit!(4, 20)];
        let n = values.length();

        // Create values with change numbers initialized to n
        let with_num = ArraySeqMtPerSLit![
            Mutex::new(PairLit!(*values.nth(0), n)),
            Mutex::new(PairLit!(*values.nth(1), n)),
            Mutex::new(PairLit!(*values.nth(2), n)),
            Mutex::new(PairLit!(*values.nth(3), n)),
            Mutex::new(PairLit!(*values.nth(4), n)),
            Mutex::new(PairLit!(*values.nth(5), n))
        ];

        // Apply atomic writes with different change numbers
        <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 1);
        <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 0);

        // Check that the value at index 2 was updated (change number 0 should win over 1)
        let guard = with_num.nth(2).lock();
        assert_eq!(guard.0, 99); // Value should be from the changes with change number 0
    }

    #[test]
    fn test_inject_string_values() {
        let values = ArraySeqMtPerSLit!["hello", "world", "test"];
        let changes = ArraySeqMtPerSLit![PairLit!(1, "rust"), PairLit!(0, "hi")];
        let result = <ArraySeqMtPerS<&str> as ArraySeqMtPerTrait<&str>>::inject(&values, &changes);

        assert_eq!(result.length(), 3);
        assert_eq!(*result.nth(0), "hi");
        assert_eq!(*result.nth(1), "rust");
        assert_eq!(*result.nth(2), "test");
    }
}
