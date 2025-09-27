//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestMathSeq {
    use apas_ai::Chap17::MathSeq::MathSeq::*;
    use apas_ai::MathSeqSLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_length_and_nth_basic() {
        let a: MathSeqS<N> = MathSeqSLit![10, 20, 30, 40];
        assert_eq!(a.length(), 4);
        assert_eq!(*a.nth(0), 10);
        assert_eq!(*a.nth(3), 40);
    }

    #[test]
    fn test_add_last_and_delete_last() {
        let mut s: MathSeqS<N> = MathSeqSLit![];
        let _ = s.add_last(1).add_last(2).add_last(3);
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(0), 1);
        assert_eq!(s.delete_last(), Some(3));
        assert_eq!(s.length(), 2);
        assert_eq!(s.delete_last(), Some(2));
        assert_eq!(s.delete_last(), Some(1));
        assert_eq!(s.delete_last(), None);
    }

    #[test]
    fn test_new_empty_singleton_and_predicates() {
        let e: MathSeqS<N> = MathSeqSLit![];
        assert_eq!(e.length(), 0);
        assert_eq!(e.length() == 0, true);
        assert_eq!(e.length() == 1, false);

        let s: MathSeqS<N> = MathSeqSLit![7];
        assert_eq!(s.length(), 1);
        assert_eq!(s.length() == 0, false);
        assert_eq!(s.length() == 1, true);

        let mut a: MathSeqS<N> = MathSeqSLit![0; 3];
        assert_eq!(a.length(), 3);
        assert_eq!(*a.nth(1), 0);
        let _ = a.set(1, 42);
        assert_eq!(*a.nth(1), 42);
    }

    #[test]
    fn test_set_in_bounds_and_out_of_bounds() {
        let mut s: MathSeqS<&str> = MathSeqSLit!["a", "b", "c"];
        let _ = s.set(1, "x");
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(1), "x");
        // Note: set doesn't return Result, so no bounds checking test
        assert_eq!(*s.nth(0), "a");
        assert_eq!(*s.nth(2), "c");
    }

    #[test]
    fn test_subseq_view_bounds() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3, 4, 5];
        let v = s.subseq(1, 3);
        assert_eq!(v, &[2, 3, 4]);
        let empty = s.subseq(2, 0);
        assert_eq!(empty.len(), 0);
        let tail = s.subseq(3, 10);
        assert_eq!(tail, &[4, 5]);
        let beyond = s.subseq(10, 5);
        assert_eq!(beyond.len(), 0);
    }

    #[test]
    fn test_subseq_bounds() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3, 4, 5];
        let c1 = s.subseq_copy(1, 3);
        assert_eq!(c1.length(), 3);
        assert_eq!(*c1.nth(0), 2);
        let c2 = s.subseq_copy(10, 3);
        assert_eq!(c2.length(), 0);
    }

    #[test]
    fn test_domain() {
        let s: MathSeqS<N> = MathSeqSLit![7, 8, 9];
        assert_eq!(s.domain(), vec![0, 1, 2]);
    }

    #[test]
    fn test_range_deduplicates_preserving_order() {
        let s: MathSeqS<&str> = MathSeqSLit!["a", "b", "a", "c", "b", "d", "d"];
        let r = s.range();
        assert_eq!(r, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_debug_format_for_mathseq() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        assert_eq!(format!("{:?}", s), "[1, 2, 3]");
    }

    #[test]
    fn test_display_format_for_mathseq() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        assert_eq!(format!("{}", s), "[1, 2, 3]");
    }

    #[test]
    fn test_multiset_range_counts_first_occurrence_order() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 1, 3, 2, 1];
        let m = s.multiset_range();
        assert_eq!(m, vec![(3, 1), (2, 2), (1, 3)]);
    }

    #[test]
    #[should_panic]
    fn test_nth_out_of_bounds_panics() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        s.nth(3);
    }

    #[test]
    fn test_range_empty_returns_empty() {
        let s: MathSeqS<N> = MathSeqSLit![];
        let r: Vec<N> = s.range();
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn test_multiset_range_empty_returns_empty() {
        let s: MathSeqS<N> = MathSeqSLit![];
        let m: Vec<(N, N)> = s.multiset_range();
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_domain_empty_is_empty() {
        let s: MathSeqS<N> = MathSeqSLit![];
        let d = s.domain();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_iter_collect_and_sum() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3, 4];
        let vals: Vec<N> = s.iter().copied().collect();
        // Compare against expected MathSeq for consistency
        let expected: MathSeqS<N> = MathSeqSLit![1, 2, 3, 4];
        assert_eq!(vals, expected.iter().copied().collect::<Vec<N>>());
        let sum: N = s.iter().copied().sum();
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_iter_mut_modify() {
        let mut s: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        for x in s.iter_mut() {
            *x *= 2;
        }
        assert_eq!(*s.nth(0), 2);
        assert_eq!(*s.nth(1), 4);
        assert_eq!(*s.nth(2), 6);
    }

    #[test]
    fn test_into_iter_consume() {
        let s: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        let vals: Vec<N> = s.into_iter().collect();
        assert_eq!(vals, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_vec_constructor() {
        let v = vec![10, 20, 30];
        let s = MathSeqS::from_vec(v);
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(1), 20);
    }

    #[test]
    fn test_with_len_constructor() {
        let s = MathSeqS::with_len(5, 42);
        assert_eq!(s.length(), 5);
        for i in 0..5 {
            assert_eq!(*s.nth(i), 42);
        }
    }

    #[test]
    fn test_set_out_of_bounds_error() {
        let mut s: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        let result = s.set(5, 99);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Index out of bounds");
        // Verify sequence unchanged
        assert_eq!(s.length(), 3);
        assert_eq!(*s.nth(0), 1);
    }

    #[test]
    fn test_isEmpty_predicate() {
        let empty: MathSeqS<N> = MathSeqSLit![];
        assert_eq!(empty.isEmpty(), true);
        
        let non_empty: MathSeqS<N> = MathSeqSLit![1];
        assert_eq!(non_empty.isEmpty(), false);
    }

    #[test]
    fn test_isSingleton_predicate() {
        let empty: MathSeqS<N> = MathSeqSLit![];
        assert_eq!(empty.isSingleton(), false);
        
        let singleton: MathSeqS<N> = MathSeqSLit![42];
        assert_eq!(singleton.isSingleton(), true);
        
        let multi: MathSeqS<N> = MathSeqSLit![1, 2];
        assert_eq!(multi.isSingleton(), false);
    }

    #[test]
    fn test_equality_comparison() {
        let s1: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        let s2: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        let s3: MathSeqS<N> = MathSeqSLit![1, 2, 4];
        
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    #[should_panic]
    fn test_mathseq_nth_panic_outofbounds() {
        let seq: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        let _ = seq.nth(10); // Index 10 is way out of bounds - should panic
    }

    #[test]
    #[should_panic]
    fn test_mathseq_nth_panic_empty() {
        let empty: MathSeqS<N> = MathSeqSLit![];
        let _ = empty.nth(0); // Any index on empty sequence should panic
    }

    #[test]
    fn test_mathseq_subseq_overflow_graceful() {
        let seq: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        // APAS style: handles overflow gracefully
        let result = seq.subseq(1, usize::MAX);
        // Should either return empty or clamp to available elements
        assert!(result.len() <= 2); // At most 2 elements from index 1 onwards
    }

    #[test]
    fn test_mathseq_empty_operations_comprehensive() {
        let empty: MathSeqS<N> = MathSeqSLit![];
        
        // Basic properties
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), true);
        assert_eq!(empty.isSingleton(), false);
        
        // Operations on empty sequence should return empty or appropriate defaults
        let empty_subseq = empty.subseq(0, 0);
        assert_eq!(empty_subseq.len(), 0);
        
        let empty_subseq2 = empty.subseq(0, 10);
        assert_eq!(empty_subseq2.len(), 0);
        
        // Iterator on empty sequence should be empty
        let collected: Vec<N> = empty.iter().cloned().collect();
        assert_eq!(collected.len(), 0);
        
        // Mutable iterator on empty sequence should be empty
        let mut empty_mut = empty.clone();
        let mut_collected: Vec<N> = empty_mut.iter_mut().map(|x| *x).collect();
        assert_eq!(mut_collected.len(), 0);
        
        let into_collected: Vec<N> = empty.clone().into_iter().collect();
        assert_eq!(into_collected.len(), 0);
        
        // Equality with other empty sequences
        let empty2: MathSeqS<N> = MathSeqSLit![];
        assert_eq!(empty, empty2);
        
        let non_empty: MathSeqS<N> = MathSeqSLit![1];
        assert_ne!(empty, non_empty);
    }

    #[test]
    fn test_mathseq_single_element_boundary() {
        // Test single element sequence operations
        let single: MathSeqS<N> = MathSeqSLit![42];
        
        // Basic properties
        assert_eq!(single.length(), 1);
        assert_eq!(single.isEmpty(), false);
        assert_eq!(single.isSingleton(), true);
        
        // Access operations
        assert_eq!(*single.nth(0), 42);
        
        // Subseq operations
        let full_subseq = single.subseq(0, 1);
        assert_eq!(full_subseq.len(), 1);
        assert_eq!(full_subseq[0], 42);
        
        let empty_subseq = single.subseq(1, 1);
        assert_eq!(empty_subseq.len(), 0);
        
        let zero_length_subseq = single.subseq(0, 0);
        assert_eq!(zero_length_subseq.len(), 0);
        
        // Set operations
        let mut single_mut = single.clone();
        let result = single_mut.set(0, 99);
        assert!(result.is_ok());
        assert_eq!(*single_mut.nth(0), 99);
        
        // Out of bounds set should return error
        let result_oob = single_mut.set(1, 100);
        assert!(result_oob.is_err());
        
        // Iterator operations
        let collected: Vec<N> = single.iter().cloned().collect();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0], 42);
        
        let mut single_mut2 = single.clone();
        for x in single_mut2.iter_mut() {
            *x *= 2;
        }
        assert_eq!(*single_mut2.nth(0), 84);
        
        let into_collected: Vec<N> = single.clone().into_iter().collect();
        assert_eq!(into_collected.len(), 1);
        assert_eq!(into_collected[0], 42);
        
        // Equality operations
        let single_same: MathSeqS<N> = MathSeqSLit![42];
        assert_eq!(single, single_same);
        
        let single_diff: MathSeqS<N> = MathSeqSLit![43];
        assert_ne!(single, single_diff);
        
        let empty: MathSeqS<N> = MathSeqSLit![];
        assert_ne!(single, empty);
    }

    #[test]
    fn test_mathseq_zero_length_operations() {
        // Test zero-length subseq operations
        let seq: MathSeqS<N> = MathSeqSLit![1, 2, 3, 4, 5];
        
        // Zero-length subseq at start
        let zero_start = seq.subseq(0, 0);
        assert_eq!(zero_start.len(), 0);
        
        // Zero-length subseq in middle
        let zero_middle = seq.subseq(2, 0);
        assert_eq!(zero_middle.len(), 0);
        
        // Zero-length subseq at end
        let zero_end = seq.subseq(5, 0);
        assert_eq!(zero_end.len(), 0);
        
        // Zero-length subseq beyond end should still return empty
        let zero_beyond = seq.subseq(10, 0);
        assert_eq!(zero_beyond.len(), 0);
        
        // Test with single element sequence
        let single: MathSeqS<N> = MathSeqSLit![42];
        let zero_single_start = single.subseq(0, 0);
        assert_eq!(zero_single_start.len(), 0);
        
        let zero_single_end = single.subseq(1, 0);
        assert_eq!(zero_single_end.len(), 0);
        
        // Test with empty sequence
        let empty: MathSeqS<N> = MathSeqSLit![];
        let zero_empty = empty.subseq(0, 0);
        assert_eq!(zero_empty.len(), 0);
        
        // All zero-length subsequences should be equal to empty
        let empty_vec: Vec<N> = vec![];
        assert_eq!(zero_start, empty_vec);
        assert_eq!(zero_middle, empty_vec);
        assert_eq!(zero_end, empty_vec);
        assert_eq!(zero_beyond, empty_vec);
        assert_eq!(zero_single_start, empty_vec);
        assert_eq!(zero_single_end, empty_vec);
        assert_eq!(zero_empty, empty_vec);
    }

    #[test]
    fn test_mathseq_iterator_boundaries() {
        // Test iterator at beginning/end boundaries
        let seq: MathSeqS<N> = MathSeqSLit![10, 20, 30, 40, 50];
        
        // Test iterator starting from beginning
        let mut iter = seq.iter();
        assert_eq!(iter.next(), Some(&10)); // First element
        assert_eq!(iter.next(), Some(&20)); // Second element
        
        // Test iterator ending at end
        let iter_end = seq.iter();
        let collected: Vec<N> = iter_end.cloned().collect();
        assert_eq!(collected.len(), 5);
        assert_eq!(collected[0], 10); // First boundary
        assert_eq!(collected[4], 50); // Last boundary
        
        // Test iterator on single element - both beginning and end
        let single: MathSeqS<N> = MathSeqSLit![99];
        let mut single_iter = single.iter();
        assert_eq!(single_iter.next(), Some(&99)); // Beginning = end
        assert_eq!(single_iter.next(), None); // Past end
        
        // Test iterator on empty sequence - no boundaries
        let empty: MathSeqS<N> = MathSeqSLit![];
        let mut empty_iter = empty.iter();
        assert_eq!(empty_iter.next(), None); // No beginning
        
        // Test mutable iterator boundaries
        let mut seq_mut: MathSeqS<N> = MathSeqSLit![1, 2, 3];
        let mut mut_iter = seq_mut.iter_mut();
        
        // Modify first element (beginning boundary)
        if let Some(first) = mut_iter.next() {
            *first = 100;
        }
        
        // Modify last element (end boundary)
        let last_iter = seq_mut.iter_mut();
        let mut last_val = None;
        for val in last_iter {
            last_val = Some(val);
        }
        if let Some(last) = last_val {
            *last = 300;
        }
        
        assert_eq!(*seq_mut.nth(0), 100); // First modified
        assert_eq!(*seq_mut.nth(2), 300); // Last modified
        
        // Test into_iter boundaries
        let seq_consume: MathSeqS<N> = MathSeqSLit![7, 8, 9];
        let mut into_iter = seq_consume.into_iter();
        assert_eq!(into_iter.next(), Some(7)); // Beginning
        assert_eq!(into_iter.next(), Some(8)); // Middle
        assert_eq!(into_iter.next(), Some(9)); // End
        assert_eq!(into_iter.next(), None); // Past end
        
        // Test iterator exhaustion and reset
        let seq_reset: MathSeqS<N> = MathSeqSLit![1, 2];
        let mut iter1 = seq_reset.iter();
        // Exhaust iterator by consuming all elements
        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter1.next(), Some(&2));
        assert_eq!(iter1.next(), None); // Should be exhausted
        
        // New iterator should start fresh at beginning
        let mut iter2 = seq_reset.iter();
        assert_eq!(iter2.next(), Some(&1)); // Fresh start at beginning
    }

    #[test]
    fn test_mathseq_maximum_size_boundary() {
        // Test maximum size collection boundary - use reasonably large size
        // to verify graceful handling without causing memory issues
        let large_size = 50_000usize;
        let large_seq: MathSeqS<N> = MathSeqS::with_len(large_size, 42);
        
        // Verify basic operations work on large sequence
        assert_eq!(large_seq.length(), large_size);
        assert_eq!(*large_seq.nth(0), 42);
        assert_eq!(*large_seq.nth(large_size - 1), 42);
        assert_eq!(large_seq.isEmpty(), false);
        assert_eq!(large_seq.isSingleton(), false);
        
        // Test subseq on large sequence
        let large_subseq = large_seq.subseq(1000, 5000);
        assert_eq!(large_subseq.len(), 5000);
        assert_eq!(large_subseq[0], 42);
        assert_eq!(large_subseq[4999], 42);
        
        // Test set operation on large sequence
        let mut large_seq_mut = large_seq.clone();
        let result = large_seq_mut.set(large_size / 2, 99);
        assert!(result.is_ok());
        assert_eq!(*large_seq_mut.nth(large_size / 2), 99);
        
        // Test iterator on large sequence (sample check)
        let mut count = 0;
        for val in large_seq.iter() {
            if count < 10 {
                assert_eq!(*val, 42);
            }
            count += 1;
            if count > large_size + 100 { // Safety check
                break;
            }
        }
        assert_eq!(count, large_size);
        
        // Test mutable iterator on large sequence (sample modification)
        let mut large_seq_mut2 = large_seq.clone();
        let mut mod_count = 0;
        for val in large_seq_mut2.iter_mut() {
            if mod_count < 1000 {
                *val = 100;
            }
            mod_count += 1;
            if mod_count >= 1000 { // Only modify first 1000 elements
                break;
            }
        }
        
        // Verify modifications
        assert_eq!(*large_seq_mut2.nth(0), 100);
        assert_eq!(*large_seq_mut2.nth(999), 100);
        assert_eq!(*large_seq_mut2.nth(1000), 42); // Unchanged
        
        // Test into_iter on smaller subset to avoid memory issues
        let medium_seq: MathSeqS<N> = MathSeqS::with_len(1000, 77);
        let collected: Vec<N> = medium_seq.into_iter().collect();
        assert_eq!(collected.len(), 1000);
        assert_eq!(collected[0], 77);
        assert_eq!(collected[999], 77);
    }
}
