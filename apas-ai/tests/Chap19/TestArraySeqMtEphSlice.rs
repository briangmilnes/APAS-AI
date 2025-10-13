//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::sync::{Arc, Barrier};
use std::thread;

use apas_ai::ArraySeqMtEphSliceSLit;
use apas_ai::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
use apas_ai::Types::Types::*;

#[test]
fn test_arrayseqmtephsliceslit_macro_functionality() {
    // Test empty sequence creation
    let empty: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceSLit![];
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), true);

    // Test sequence creation with elements
    let with_data: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceSLit![1, 2, 3];
    assert_eq!(with_data.length(), 3);
    assert_eq!(with_data.isEmpty(), false);
    assert_eq!(with_data.nth_cloned(0), 1);
    assert_eq!(with_data.nth_cloned(1), 2);
    assert_eq!(with_data.nth_cloned(2), 3);
}

#[test]
fn test_arrayseqmtephslice_empty() {
    let empty: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceS::empty();
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), true);
    assert_eq!(empty.isSingleton(), false);
}

#[test]
fn test_arrayseqmtephslice_new() {
    let seq = ArraySeqMtEphSliceS::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.isEmpty(), false);
    assert_eq!(seq.isSingleton(), false);

    for i in 0..5 {
        assert_eq!(seq.nth_cloned(i), 42);
    }
}

#[test]
fn test_arrayseqmtephslice_singleton() {
    let seq = ArraySeqMtEphSliceS::singleton(99);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.isEmpty(), false);
    assert_eq!(seq.isSingleton(), true);
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
    assert_eq!(empty_sub.isEmpty(), true);

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
    assert_eq!(empty_slice.isEmpty(), true);
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
    let filtered = ArraySeqMtEphSliceS::filter(&seq, |x| if *x % 2 == 0 { true } else { false });

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
            assert_eq!(seq_clone.isEmpty(), false);

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
    assert_eq!(empty.isEmpty(), true);
    assert_eq!(empty.isSingleton(), false);

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
    let empty_filtered = ArraySeqMtEphSliceS::filter(&empty, |_| true);
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
    assert_eq!(seq1.isEmpty(), true);
    assert_eq!(seq1.isSingleton(), false);

    let seq2 = ArraySeqMtEphSliceS::singleton(42);
    assert_eq!(seq2.isEmpty(), false);
    assert_eq!(seq2.isSingleton(), true);

    let seq3 = ArraySeqMtEphSliceS::new(3, 0);
    assert_eq!(seq3.isEmpty(), false);
    assert_eq!(seq3.isSingleton(), false);
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

// ========== Additional Comprehensive Tests for Untested Functions ==========

#[test]
fn test_arrayseqmtephslice_from_box() {
    let data: Box<[i32]> = vec![10, 20, 30, 40].into_boxed_slice();
    let seq = ArraySeqMtEphSliceS::from_box(data);
    assert_eq!(seq.length(), 4);
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(3), 40);
}

#[test]
fn test_arrayseqmtephslice_from_vec() {
    let vec_data = vec![1, 2, 3, 4, 5];
    let seq = ArraySeqMtEphSliceS::from_vec(vec_data);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(2), 3);
}

#[test]
fn test_arrayseqmtephslice_to_vec() {
    let seq = ArraySeqMtEphSliceS::tabulate(&|i| i * 10, 4);
    let vec_result = seq.to_vec();
    assert_eq!(vec_result, vec![0, 10, 20, 30]);
}

#[test]
fn test_arrayseqmtephslice_with_exclusive() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);

    // Use with_exclusive to modify the slice directly
    seq.with_exclusive(|slice| {
        assert_eq!(slice.len(), 4);
        slice[0] = 10;
        slice[3] = 40;
    });

    // Verify modifications
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(3), 40);
}

#[test]
fn test_arrayseqmtephslice_set() {
    let mut seq = ArraySeqMtEphSliceS::new(4, 0);

    // Test set method (alias for update)
    let result = seq.set(0, 100);
    assert!(result.is_ok());
    assert_eq!(seq.nth_cloned(0), 100);

    let result = seq.set(2, 200);
    assert!(result.is_ok());
    assert_eq!(seq.nth_cloned(2), 200);

    // Test out-of-bounds
    let result = seq.set(10, 999);
    assert!(result.is_err());
}

#[test]
fn test_arrayseqmtephslice_clamp_subrange() {
    let seq = ArraySeqMtEphSliceS::tabulate(&|i| i, 10);

    // Normal subrange
    let sub1 = seq.subseq_copy(2, 5);
    assert_eq!(sub1.length(), 5);
    assert_eq!(sub1.nth_cloned(0), 2);

    // Clamped start (beyond bounds)
    let sub2 = seq.subseq_copy(15, 3);
    assert_eq!(sub2.length(), 0);

    // Clamped end (extends beyond bounds)
    let sub3 = seq.subseq_copy(8, 10);
    assert_eq!(sub3.length(), 2); // Only elements 8 and 9 available
}

#[test]
fn test_arrayseqmtephslice_len() {
    let seq1 = ArraySeqMtEphSliceS::new(7, 42);
    assert_eq!(seq1.length(), 7);

    let seq2: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceS::empty();
    assert_eq!(seq2.length(), 0);

    let seq3 = ArraySeqMtEphSliceS::singleton(99);
    assert_eq!(seq3.length(), 1);
}

#[test]
fn test_arrayseqmtephslice_clone() {
    let seq1 = ArraySeqMtEphSliceS::tabulate(&|i| i * 2, 5);
    let seq2 = seq1.clone();

    assert_eq!(seq1.length(), seq2.length());
    for i in 0..seq1.length() {
        assert_eq!(seq1.nth_cloned(i), seq2.nth_cloned(i));
    }
}

#[test]
fn test_arrayseqmtephslice_eq() {
    let seq1 = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let seq2 = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let seq3 = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 4]);

    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
}

#[test]
fn test_arrayseqmtephslice_fmt_debug() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let debug_str = format!("{:?}", seq);
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));
}

#[test]
fn test_arrayseqmtephslice_fmt_display() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30]);
    let display_str = format!("{}", seq);
    assert!(display_str.contains("10"));
    assert!(display_str.contains("20"));
    assert!(display_str.contains("30"));
}

#[test]
fn test_arrayseqmtephslice_append() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    let b = ArraySeqMtEphSliceS::from_vec(vec![3, 4, 5]);

    let c = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(c.nth_cloned(0), 1);
    assert_eq!(c.nth_cloned(1), 2);
    assert_eq!(c.nth_cloned(2), 3);
    assert_eq!(c.nth_cloned(3), 4);
    assert_eq!(c.nth_cloned(4), 5);
}

#[test]
fn test_arrayseqmtephslice_append_select() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![10, 20]);
    let b = ArraySeqMtEphSliceS::from_vec(vec![30, 40]);

    let c = ArraySeqMtEphSliceS::append_select(&a, &b);
    assert_eq!(c.length(), 4);
    assert_eq!(c.nth_cloned(0), 10);
    assert_eq!(c.nth_cloned(1), 20);
    assert_eq!(c.nth_cloned(2), 30);
    assert_eq!(c.nth_cloned(3), 40);
}

#[test]
fn test_arrayseqmtephslice_flatten() {
    let seq1 = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    let seq2 = ArraySeqMtEphSliceS::from_vec(vec![3, 4]);
    let seq3 = ArraySeqMtEphSliceS::from_vec(vec![5]);

    let sequences = vec![seq1, seq2, seq3];
    let flattened = ArraySeqMtEphSliceS::flatten(&sequences);

    assert_eq!(flattened.length(), 5);
    assert_eq!(flattened.nth_cloned(0), 1);
    assert_eq!(flattened.nth_cloned(2), 3);
    assert_eq!(flattened.nth_cloned(4), 5);
}

#[test]
fn test_arrayseqmtephslice_reduce() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);
    let sum = ArraySeqMtEphSliceS::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);

    let product = ArraySeqMtEphSliceS::reduce(&seq, |a, b| a * b, 1);
    assert_eq!(product, 24);
}

#[test]
fn test_arrayseqmtephslice_scan() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);
    let (prefix_sums, total) = ArraySeqMtEphSliceS::scan(&seq, &|a, b| a + b, 0);

    assert_eq!(prefix_sums.length(), 4);
    assert_eq!(prefix_sums.nth_cloned(0), 0);
    assert_eq!(prefix_sums.nth_cloned(1), 1);
    assert_eq!(prefix_sums.nth_cloned(2), 3);
    assert_eq!(prefix_sums.nth_cloned(3), 6);
    assert_eq!(total, 10);
}

#[test]
fn test_arrayseqmtephslice_iterate() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);
    let sum = ArraySeqMtEphSliceS::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);

    let product = ArraySeqMtEphSliceS::iterate(&seq, &|acc, x| acc * x, 1);
    assert_eq!(product, 24);
}

#[test]
fn test_arrayseqmtephslice_inject() {
    let base = ArraySeqMtEphSliceS::from_vec(vec![0, 0, 0, 0, 0]);
    let updates = vec![(1, 10), (3, 30)];

    let result = ArraySeqMtEphSliceS::inject(&base, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(result.nth_cloned(0), 0);
    assert_eq!(result.nth_cloned(1), 10);
    assert_eq!(result.nth_cloned(2), 0);
    assert_eq!(result.nth_cloned(3), 30);
    assert_eq!(result.nth_cloned(4), 0);
}

#[test]
fn test_arrayseqmtephslice_ninject() {
    let base = ArraySeqMtEphSliceS::from_vec(vec![0, 0, 0, 0]);
    let updates = vec![(1, 10), (1, 20), (2, 30)];

    // ninject applies updates (rightmost wins by reversing)
    let result = ArraySeqMtEphSliceS::ninject(&base, &updates);
    // Based on implementation, it reverses and applies, so leftmost in reversed = rightmost in original
    assert_eq!(result.nth_cloned(1), 10); // First occurrence in forward direction
    assert_eq!(result.nth_cloned(2), 30);
}

#[test]
fn test_arrayseqmtephslice_is_empty() {
    let empty = ArraySeqMtEphSliceS::<i32>::empty();
    assert_eq!(empty.isEmpty(), true);

    let non_empty = ArraySeqMtEphSliceS::singleton(42);
    assert_eq!(non_empty.isEmpty(), false);
}

#[test]
fn test_arrayseqmtephslice_is_singleton() {
    let singleton = ArraySeqMtEphSliceS::singleton(99);
    assert_eq!(singleton.isSingleton(), true);

    let empty = ArraySeqMtEphSliceS::<i32>::empty();
    assert_eq!(empty.isSingleton(), false);

    let multi = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    assert_eq!(multi.isSingleton(), false);
}

#[test]
fn test_arrayseqmtephslice_trait_empty() {
    use ArraySeqMtEphSliceTrait;

    let empty = <ArraySeqMtEphSliceS<i32> as ArraySeqMtEphSliceTrait<i32>>::empty();
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_arrayseqmtephslice_trait_new() {
    use ArraySeqMtEphSliceTrait;

    let seq = <ArraySeqMtEphSliceS<i32> as ArraySeqMtEphSliceTrait<i32>>::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(seq.nth_cloned(i), 42);
    }
}

#[test]
fn test_arrayseqmtephslice_trait_length() {
    use ArraySeqMtEphSliceTrait;

    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    assert_eq!(
        <ArraySeqMtEphSliceS<i32> as ArraySeqMtEphSliceTrait<i32>>::length(&seq),
        3
    );
}

#[test]
fn test_arrayseqmtephslice_trait_map() {
    use ArraySeqMtEphSliceTrait;

    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);
    let doubled = <ArraySeqMtEphSliceS<i32> as ArraySeqMtEphSliceTrait<i32>>::map(&seq, |x| x * 2);

    assert_eq!(doubled.length(), 4);
    assert_eq!(doubled.nth_cloned(0), 2);
    assert_eq!(doubled.nth_cloned(3), 8);
}

#[test]
fn test_arrayseqmtephslice_trait_filter() {
    use ArraySeqMtEphSliceTrait;

    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4, 5, 6]);
    let evens = <ArraySeqMtEphSliceS<i32> as ArraySeqMtEphSliceTrait<i32>>::filter(&seq, |x| *x % 2 == 0);

    assert_eq!(evens.length(), 3);
    assert_eq!(evens.nth_cloned(0), 2);
    assert_eq!(evens.nth_cloned(1), 4);
    assert_eq!(evens.nth_cloned(2), 6);
}

#[test]
fn test_arrayseqmtephslice_trait_tabulate() {
    use ArraySeqMtEphSliceTrait;

    let seq = <ArraySeqMtEphSliceS<i32> as ArraySeqMtEphSliceTrait<i32>>::tabulate(&|i| (i * i) as i32, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(0), 0);
    assert_eq!(seq.nth_cloned(2), 4);
    assert_eq!(seq.nth_cloned(4), 16);
}

#[test]
fn test_arrayseqmtephslice_type_checks() {
    // This function should exist for macro validation
    // Just verify it compiles
    let _seq: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceS::new(3, 42);
}
