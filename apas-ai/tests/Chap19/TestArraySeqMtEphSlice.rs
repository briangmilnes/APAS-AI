//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestArraySeqMtEphSlice {
    use apas_ai::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use apas_ai::Types::Types::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_arrayseqmtephslice_empty() {
        let empty: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceS::empty();
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), B::True);
        assert_eq!(empty.isSingleton(), B::False);
    }

    #[test]
    fn test_arrayseqmtephslice_new() {
        let seq = ArraySeqMtEphSliceS::new(5, 42);
        assert_eq!(seq.length(), 5);
        assert_eq!(seq.isEmpty(), B::False);
        assert_eq!(seq.isSingleton(), B::False);
        
        for i in 0..5 {
            assert_eq!(seq.nth_cloned(i), 42);
        }
    }

    #[test]
    fn test_arrayseqmtephslice_singleton() {
        let seq = ArraySeqMtEphSliceS::singleton(99);
        assert_eq!(seq.length(), 1);
        assert_eq!(seq.isEmpty(), B::False);
        assert_eq!(seq.isSingleton(), B::True);
        assert_eq!(seq.nth_cloned(0), 99);
    }

    #[test]
    fn test_arrayseqmtephslice_update() {
        let mut seq = ArraySeqMtEphSliceS::new(3, 0);
        
        // Test successful updates
        let result = seq.update(0, 10);
        assert!(result.is_ok());
        assert_eq!(seq.nth_cloned(0), 10);
        
        let result = seq.update(1, 20);
        assert!(result.is_ok());
        assert_eq!(seq.nth_cloned(1), 20);
        
        let result = seq.update(2, 30);
        assert!(result.is_ok());
        assert_eq!(seq.nth_cloned(2), 30);
        
        // Test out-of-bounds update
        let result = seq.update(5, 99);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Index out of bounds");
    }

    #[test]
    fn test_arrayseqmtephslice_subseq_copy() {
        let seq = ArraySeqMtEphSliceS::tabulate(&|i| i * 10, 5);
        
        // Test normal subseq
        let sub = seq.subseq_copy(1, 3);
        assert_eq!(sub.length(), 3);
        assert_eq!(sub.nth_cloned(0), 10);
        assert_eq!(sub.nth_cloned(1), 20);
        assert_eq!(sub.nth_cloned(2), 30);
        
        // Test empty subseq
        let empty_sub = seq.subseq_copy(2, 0);
        assert_eq!(empty_sub.length(), 0);
        assert_eq!(empty_sub.isEmpty(), B::True);
        
        // Test out-of-bounds start (should return empty)
        let oob_sub = seq.subseq_copy(10, 2);
        assert_eq!(oob_sub.length(), 0);
        
        // Test length extending beyond bounds (should clamp)
        let clamped_sub = seq.subseq_copy(3, 5);
        assert_eq!(clamped_sub.length(), 2); // Only elements 3 and 4 available
        assert_eq!(clamped_sub.nth_cloned(0), 30);
        assert_eq!(clamped_sub.nth_cloned(1), 40);
    }

    #[test]
    fn test_arrayseqmtephslice_slice() {
        let seq = ArraySeqMtEphSliceS::tabulate(&|i| i + 100, 6);
        
        // Test normal slice (shares backing data)
        let sliced = seq.slice(2, 3);
        assert_eq!(sliced.length(), 3);
        assert_eq!(sliced.nth_cloned(0), 102);
        assert_eq!(sliced.nth_cloned(1), 103);
        assert_eq!(sliced.nth_cloned(2), 104);
        
        // Test empty slice
        let empty_slice = seq.slice(3, 0);
        assert_eq!(empty_slice.length(), 0);
        assert_eq!(empty_slice.isEmpty(), B::True);
    }

    #[test]
    fn test_arrayseqmtephslice_tabulate() {
        let seq = ArraySeqMtEphSliceS::tabulate(&|i| i * i, 4);
        assert_eq!(seq.length(), 4);
        assert_eq!(seq.nth_cloned(0), 0);
        assert_eq!(seq.nth_cloned(1), 1);
        assert_eq!(seq.nth_cloned(2), 4);
        assert_eq!(seq.nth_cloned(3), 9);
    }

    #[test]
    fn test_arrayseqmtephslice_map() {
        let seq = ArraySeqMtEphSliceS::tabulate(&|i| i + 1, 4);
        let mapped = ArraySeqMtEphSliceS::map(&seq, |x| x * 2);
        
        assert_eq!(mapped.length(), 4);
        assert_eq!(mapped.nth_cloned(0), 2);
        assert_eq!(mapped.nth_cloned(1), 4);
        assert_eq!(mapped.nth_cloned(2), 6);
        assert_eq!(mapped.nth_cloned(3), 8);
    }

    #[test]
    fn test_arrayseqmtephslice_filter() {
        let seq = ArraySeqMtEphSliceS::tabulate(&|i| i, 6);
        let filtered = ArraySeqMtEphSliceS::filter(&seq, |x| if *x % 2 == 0 { B::True } else { B::False });
        
        assert_eq!(filtered.length(), 3); // 0, 2, 4
        assert_eq!(filtered.nth_cloned(0), 0);
        assert_eq!(filtered.nth_cloned(1), 2);
        assert_eq!(filtered.nth_cloned(2), 4);
    }

    #[test]
    fn test_arrayseqmtephslice_concurrent_access() {
        let seq = Arc::new(ArraySeqMtEphSliceS::tabulate(&|i| i * 10, 10));
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let mut handles = vec![];
        
        for thread_id in 0..num_threads {
            let seq_clone = Arc::clone(&seq);
            let barrier_clone = Arc::clone(&barrier);
            
            handles.push(thread::spawn(move || {
                barrier_clone.wait(); // Wait for all threads to be ready
                
                // Perform concurrent read operations
                let mut sum = 0;
                for i in 0..seq_clone.length() {
                    sum += seq_clone.nth_cloned(i);
                }
                
                // Verify basic properties
                assert_eq!(seq_clone.length(), 10);
                assert_eq!(seq_clone.isEmpty(), B::False);
                
                (thread_id, sum, seq_clone.length())
            }));
        }
        
        for handle in handles {
            let (_thread_id, sum, length) = handle.join().unwrap();
            assert_eq!(length, 10);
            assert_eq!(sum, 450); // 0+10+20+...+90 = 450
        }
    }

    #[test]
    fn test_arrayseqmtephslice_concurrent_updates() {
        let seq = Arc::new(std::sync::Mutex::new(ArraySeqMtEphSliceS::new(8, 0)));
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let mut handles = vec![];
        
        for thread_id in 0..num_threads {
            let seq_clone = Arc::clone(&seq);
            let barrier_clone = Arc::clone(&barrier);
            
            handles.push(thread::spawn(move || {
                barrier_clone.wait(); // Wait for all threads to be ready
                
                // Each thread updates 2 elements
                let start_idx = thread_id * 2;
                let end_idx = start_idx + 2;
                
                for i in start_idx..end_idx {
                    let mut guard = seq_clone.lock().unwrap();
                    let _ = guard.update(i, (i + 1) * 100);
                }
                
                thread_id
            }));
        }
        
        for handle in handles {
            let _ = handle.join().unwrap();
        }
        
        // Verify all updates were applied
        let final_seq = seq.lock().unwrap();
        assert_eq!(final_seq.length(), 8);
        for i in 0..8 {
            assert_eq!(final_seq.nth_cloned(i), (i + 1) * 100);
        }
    }

    #[test]
    fn test_arrayseqmtephslice_edge_cases() {
        // Test with zero-length sequence
        let empty = ArraySeqMtEphSliceS::new(0, 42);
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), B::True);
        assert_eq!(empty.isSingleton(), B::False);
        
        // Test subseq on empty sequence
        let empty_sub = empty.subseq_copy(0, 0);
        assert_eq!(empty_sub.length(), 0);
        
        // Test slice on empty sequence
        let empty_slice = empty.slice(0, 0);
        assert_eq!(empty_slice.length(), 0);
        
        // Test map on empty sequence
        let empty_mapped = ArraySeqMtEphSliceS::map(&empty, |x| x * 2);
        assert_eq!(empty_mapped.length(), 0);
        
        // Test filter on empty sequence
        let empty_filtered = ArraySeqMtEphSliceS::filter(&empty, |_| B::True);
        assert_eq!(empty_filtered.length(), 0);
    }

    #[test]
    fn test_arrayseqmtephslice_large_sequence() {
        let large_seq = ArraySeqMtEphSliceS::tabulate(&|i| i, 1000);
        assert_eq!(large_seq.length(), 1000);
        
        // Test random access
        assert_eq!(large_seq.nth_cloned(0), 0);
        assert_eq!(large_seq.nth_cloned(500), 500);
        assert_eq!(large_seq.nth_cloned(999), 999);
        
        // Test subseq on large sequence
        let mid_section = large_seq.subseq_copy(400, 200);
        assert_eq!(mid_section.length(), 200);
        assert_eq!(mid_section.nth_cloned(0), 400);
        assert_eq!(mid_section.nth_cloned(199), 599);
        
        // Test slice on large sequence
        let sliced_section = large_seq.slice(300, 100);
        assert_eq!(sliced_section.length(), 100);
        assert_eq!(sliced_section.nth_cloned(0), 300);
        assert_eq!(sliced_section.nth_cloned(99), 399);
    }

    #[test]
    fn test_arrayseqmtephslice_predicates() {
        let seq1: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceS::empty();
        assert_eq!(seq1.isEmpty(), B::True);
        assert_eq!(seq1.isSingleton(), B::False);
        
        let seq2 = ArraySeqMtEphSliceS::singleton(42);
        assert_eq!(seq2.isEmpty(), B::False);
        assert_eq!(seq2.isSingleton(), B::True);
        
        let seq3 = ArraySeqMtEphSliceS::new(3, 0);
        assert_eq!(seq3.isEmpty(), B::False);
        assert_eq!(seq3.isSingleton(), B::False);
    }

    #[test]
    fn test_arrayseqmtephslice_complex_operations() {
        // Create a sequence and perform multiple operations
        let mut seq = ArraySeqMtEphSliceS::tabulate(&|i| i + 10, 5);
        
        // Update some elements
        let _ = seq.update(1, 100);
        let _ = seq.update(3, 300);
        
        // Verify updates
        assert_eq!(seq.nth_cloned(0), 10);
        assert_eq!(seq.nth_cloned(1), 100);
        assert_eq!(seq.nth_cloned(2), 12);
        assert_eq!(seq.nth_cloned(3), 300);
        assert_eq!(seq.nth_cloned(4), 14);
        
        // Create subseq and verify independence
        let sub = seq.subseq_copy(1, 3);
        assert_eq!(sub.length(), 3);
        assert_eq!(sub.nth_cloned(0), 100);
        assert_eq!(sub.nth_cloned(1), 12);
        assert_eq!(sub.nth_cloned(2), 300);
        
        // Create slice and verify it shares backing data conceptually
        let sliced = seq.slice(2, 2);
        assert_eq!(sliced.length(), 2);
        assert_eq!(sliced.nth_cloned(0), 12);
        assert_eq!(sliced.nth_cloned(1), 300);
    }
}
