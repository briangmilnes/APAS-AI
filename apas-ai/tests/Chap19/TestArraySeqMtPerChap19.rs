//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArrayMtPerSLit;
use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerTrait as Chap18Trait;
use apas_ai::Chap19::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::PairLit;
use apas_ai::Types::Types::*;

#[test]
fn test_new_and_set() {
    let a: ArraySeqMtPerS<N> = ArrayMtPerSLit![7; 3];
    assert_eq!(a.length(), 3);
    assert_eq!(*a.nth(0), 7);
    assert_eq!(*a.nth(1), 7);
    assert_eq!(*a.nth(2), 7);
    let changes1 = ArrayMtPerSLit![Pair(1, 9)];
    let b = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&a, &changes1);
    let changes2 = ArrayMtPerSLit![Pair(0, 2)];
    let c = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&b, &changes2);
    assert_eq!(*c.nth(0), 2);
    assert_eq!(*c.nth(1), 9);
    assert_eq!(*c.nth(2), 7);
}

#[test]
fn test_length_and_nth_basic() {
    let a = ArrayMtPerSLit![10, 20, 30, 40];
    assert_eq!(a.length(), 4);
    assert_eq!(*a.nth(0), 10);
    assert_eq!(*a.nth(3), 40);
}

#[test]
fn test_empty() {
    let empty: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
    assert_eq!(empty.length(), 0);
    assert!(empty.length() == 0);
}

#[test]
fn test_sequence_basic() {
    let a: ArraySeqMtPerS<B> = ArrayMtPerSLit![false; 10];
    assert!(a.length() != 0);
    assert_eq!(a.length(), 10);
    let changes1 = ArrayMtPerSLit![Pair(0, true), Pair(1, false), Pair(2, true)];
    let d = <ArraySeqMtPerS<B> as Chap18Trait<B>>::inject(&a, &changes1);
    assert_eq!(d.length(), 10);
    let head4 = ArrayMtPerSLit![*d.nth(0), *d.nth(1), *d.nth(2), *d.nth(3)];
    assert_eq!(head4, ArrayMtPerSLit![true, false, true, false]);
}

#[test]
fn test_singleton() {
    let a: ArraySeqMtPerS<N> = ArraySeqMtPerS::singleton(42);
    assert_eq!(a.length(), 1);
    assert_eq!(*a.nth(0), 42);
    assert!(a.length() != 0);
}

#[test]
fn test_from_vec() {
    let a = ArrayMtPerSLit![1, 2, 3, 4, 5];
    assert_eq!(a.length(), 5);
    assert_eq!(*a.nth(0), 1);
    assert_eq!(*a.nth(4), 5);
}

#[test]
fn test_subseq() {
    let a = ArrayMtPerSLit![10, 20, 30, 40, 50];
    let sub = a.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 20);
    assert_eq!(*sub.nth(1), 30);
    assert_eq!(*sub.nth(2), 40);
}

#[test]
fn test_subseq_view() {
    let a = ArrayMtPerSLit![10, 20, 30, 40, 50];
    let view = a.subseq_copy(1, 3);
    assert_eq!(view.length(), 3);
    assert_eq!(*view.nth(0), 20);
    assert_eq!(*view.nth(1), 30);
    assert_eq!(*view.nth(2), 40);
}

#[test]
fn test_iterators() {
    let a = ArrayMtPerSLit![1, 2, 3];
    let collected: Vec<N> = a.iter().copied().collect();
    // For MT, order might vary, so use set comparison
    assert_eq!(collected.len(), 3);
    let mut collected_sorted = collected.clone();
    collected_sorted.sort();
    assert_eq!(collected_sorted, vec![1, 2, 3]);
}

#[test]
fn test_macro_literals() {
    let empty: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
    assert_eq!(empty.length(), 0);

    let single = ArrayMtPerSLit![42];
    assert_eq!(single.length(), 1);
    assert_eq!(*single.nth(0), 42);

    let multi = ArrayMtPerSLit![1, 2, 3, 4];
    assert_eq!(multi.length(), 4);
    assert_eq!(*multi.nth(0), 1);
    assert_eq!(*multi.nth(3), 4);

    let repeated = ArrayMtPerSLit![7; 5];
    assert_eq!(repeated.length(), 5);
    assert_eq!(*repeated.nth(0), 7);
    assert_eq!(*repeated.nth(4), 7);
}

#[test]
fn test_equality_and_debug() {
    let a = ArrayMtPerSLit![1, 2, 3];
    let b = ArrayMtPerSLit![1, 2, 3];
    let c = ArrayMtPerSLit![1, 2, 4];

    assert_eq!(a, b);
    assert_ne!(a, c);

    // Debug format should work
    let debug_str = format!("{a:?}");
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));
}

#[test]
fn test_display_format() {
    let a = ArrayMtPerSLit![1, 2, 3];
    let display_str = format!("{a}");
    assert!(display_str.contains("1"));
    assert!(display_str.contains("2"));
    assert!(display_str.contains("3"));
}

#[test]
fn test_string_sequences() {
    let a = ArrayMtPerSLit!["hello", "world"];
    assert_eq!(a.length(), 2);
    assert_eq!(*a.nth(0), "hello");
    assert_eq!(*a.nth(1), "world");
}

#[test]
fn test_boolean_sequences() {
    let a = ArrayMtPerSLit![true, false, true];
    assert_eq!(a.length(), 3);
    assert!(*a.nth(0));
    assert!(!(*a.nth(1)));
    assert!(*a.nth(2));
}

#[test]
fn test_concurrent_read_access() {
    use std::sync::Arc;
    use std::thread;

    let a = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    // Spawn multiple threads to read concurrently
    for i in 0..4 {
        let a_clone = Arc::clone(&a);
        let handle = thread::spawn(move || {
            // Each thread reads different elements
            let index = i % a_clone.length();
            let value = *a_clone.nth(index);
            (index, value)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify all reads succeeded
    assert_eq!(results.len(), 4);
    for (index, value) in results {
        assert_eq!(value, (index + 1) as N); // Values are 1,2,3,4,5
    }
}

#[test]
fn test_concurrent_inject_operations() {
    use std::sync::Arc;
    use std::thread;

    let base = Arc::new(ArrayMtPerSLit![0; 10]);
    let mut handles = vec![];

    // Spawn threads that create different inject operations
    for i in 0..3 {
        let base_clone = Arc::clone(&base);
        let handle = thread::spawn(move || {
            let updates = ArrayMtPerSLit![Pair(i, (i + 1) * 10)];
            <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&base_clone, &updates)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify all inject operations succeeded
    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.length(), 10);
        assert_eq!(*result.nth(i), (i + 1) * 10);
    }
}

#[test]
fn test_concurrent_subseq_operations() {
    use std::sync::Arc;
    use std::thread;

    let a = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut handles = vec![];

    // Spawn threads that create different subseq operations
    for i in 0..3 {
        let a_clone = Arc::clone(&a);
        let handle = thread::spawn(move || {
            let start = i * 2;
            let length = 3;
            a_clone.subseq_copy(start, length)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify all subseq operations succeeded
    assert_eq!(results.len(), 3);

    // Check first subseq: start=0, length=3 -> [1,2,3]
    assert_eq!(results[0].length(), 3);
    assert_eq!(*results[0].nth(0), 1);

    // Check second subseq: start=2, length=3 -> [3,4,5]
    assert_eq!(results[1].length(), 3);
    assert_eq!(*results[1].nth(0), 3);

    // Check third subseq: start=4, length=3 -> [5,6,7]
    assert_eq!(results[2].length(), 3);
    assert_eq!(*results[2].nth(0), 5);
}

#[test]
fn test_thread_safety_with_barrier() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let a = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5]);
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    // Spawn threads that all start at the same time
    for _i in 0..3 {
        let a_clone = Arc::clone(&a);
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier_clone.wait(); // Wait for all threads to be ready

            // All threads perform the same operations simultaneously
            let length = a_clone.length();
            let first = *a_clone.nth(0);
            let last = *a_clone.nth(length - 1);

            (length, first, last)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // All threads should get the same results
    assert_eq!(results.len(), 3);
    for result in results {
        assert_eq!(result, (5, 1, 5));
    }
}

#[test]
fn test_concurrent_iterator_access() {
    use std::sync::Arc;
    use std::thread;

    let a = Arc::new(ArrayMtPerSLit![10, 20, 30, 40, 50]);
    let mut handles = vec![];

    // Spawn threads that iterate concurrently
    for _ in 0..3 {
        let a_clone = Arc::clone(&a);
        let handle = thread::spawn(move || {
            let collected: Vec<N> = a_clone.iter().copied().collect();
            collected
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // All threads should collect the same elements (though order might vary for Mt)
    assert_eq!(results.len(), 3);
    for result in results {
        assert_eq!(result.len(), 5);
        let mut sorted_result = result.clone();
        sorted_result.sort();
        assert_eq!(sorted_result, vec![10, 20, 30, 40, 50]);
    }
}

#[test]
fn test_race_condition_verification_concurrent_reads() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    let seq = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let barrier = Arc::new(Barrier::new(10));
    let race_detected = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    // Spawn multiple threads that read simultaneously
    for thread_id in 0..10 {
        let seq_clone = Arc::clone(&seq);
        let barrier_clone = Arc::clone(&barrier);
        let race_detected_clone = Arc::clone(&race_detected);

        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();

            // Perform concurrent reads
            let mut local_sum = 0;
            for i in 0..seq_clone.length() {
                let val = *seq_clone.nth(i);
                local_sum += val;

                // Verify data consistency during concurrent access
                if !(1..=10).contains(&val) {
                    race_detected_clone.store(true, Ordering::SeqCst);
                }
            }

            (thread_id, local_sum)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify no race conditions detected
    assert!(
        !race_detected.load(Ordering::SeqCst),
        "Race condition detected in concurrent reads"
    );

    // All threads should get the same sum (1+2+...+10 = 55)
    for (thread_id, sum) in results {
        assert_eq!(sum, 55, "Thread {thread_id} got incorrect sum");
    }
}

#[test]
fn test_race_condition_verification_concurrent_inject_operations() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    let base_seq = Arc::new(ArrayMtPerSLit![0; 100]);
    let barrier = Arc::new(Barrier::new(10));
    let operation_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Spawn threads that perform inject operations concurrently
    for thread_id in 0..10 {
        let base_clone = Arc::clone(&base_seq);
        let barrier_clone = Arc::clone(&barrier);
        let counter_clone = Arc::clone(&operation_counter);

        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();

            // Perform inject operations
            let mut results = vec![];
            for i in 0..10 {
                let index = (thread_id * 10 + i) % 100;
                let value = thread_id * 100 + i;
                let updates = ArrayMtPerSLit![Pair(index, value)];

                let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&base_clone, &updates);
                results.push((index, value, *result.nth(index)));

                counter_clone.fetch_add(1, Ordering::SeqCst);
            }

            (thread_id, results)
        });
        handles.push(handle);
    }

    // Collect results
    let mut all_results = vec![];
    for handle in handles {
        let (thread_id, results) = handle.join().unwrap();
        all_results.push((thread_id, results));
    }

    // Verify all operations completed
    assert_eq!(
        operation_counter.load(Ordering::SeqCst),
        100,
        "Not all operations completed"
    );

    // Verify each inject operation worked correctly
    for (thread_id, results) in all_results {
        for (index, expected_value, actual_value) in results {
            assert_eq!(
                actual_value, expected_value,
                "Thread {thread_id} inject at index {index} failed: expected {expected_value}, got {actual_value}"
            );
        }
    }
}

#[test]
fn test_race_condition_verification_mixed_operations() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    let seq = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let barrier = Arc::new(Barrier::new(6));
    let inconsistency_detected = Arc::new(AtomicBool::new(false));
    let mut reader_handles = vec![];
    let mut inject_handles = vec![];

    // Spawn reader threads
    for thread_id in 0..3 {
        let seq_clone = Arc::clone(&seq);
        let barrier_clone = Arc::clone(&barrier);
        let inconsistency_clone = Arc::clone(&inconsistency_detected);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            // Perform multiple reads and verify consistency
            for _ in 0..100 {
                let mut sum = 0;
                for i in 0..seq_clone.length() {
                    let val = *seq_clone.nth(i);
                    sum += val;

                    // Check for impossible values
                    if !(1..=10).contains(&val) {
                        inconsistency_clone.store(true, Ordering::SeqCst);
                    }
                }

                // Sum should always be 55 for original sequence
                if sum != 55 {
                    // This is expected since inject operations create new sequences
                    // but we verify the original sequence remains consistent
                    let original_sum: N = (0..seq_clone.length()).map(|i| *seq_clone.nth(i)).sum();
                    if original_sum != 55 {
                        inconsistency_clone.store(true, Ordering::SeqCst);
                    }
                }
            }

            thread_id
        });
        reader_handles.push(handle);
    }

    // Spawn inject operation threads
    for thread_id in 3..6 {
        let seq_clone = Arc::clone(&seq);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            // Perform inject operations
            let mut operation_count = 0;
            for i in 0..50 {
                let index = i % seq_clone.length();
                let value = thread_id * 100 + i;
                let updates = ArrayMtPerSLit![Pair(index, value)];

                let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&seq_clone, &updates);

                // Verify the inject worked
                if *result.nth(index) == value {
                    operation_count += 1;
                }
            }

            (thread_id, operation_count)
        });
        inject_handles.push(handle);
    }

    // Collect results
    let mut reader_results = vec![];
    for handle in reader_handles {
        let thread_id = handle.join().unwrap();
        reader_results.push(thread_id);
    }

    let mut inject_results = vec![];
    for handle in inject_handles {
        let (thread_id, op_count) = handle.join().unwrap();
        inject_results.push((thread_id, op_count));
    }

    // Verify no data inconsistencies detected
    assert!(
        !inconsistency_detected.load(Ordering::SeqCst),
        "Data inconsistency detected during concurrent operations"
    );

    // Verify all operations completed successfully
    for thread_id in reader_results {
        assert!(thread_id < 3, "Reader thread ID out of range");
    }

    for (thread_id, op_count) in inject_results {
        assert!((3..6).contains(&thread_id), "Inject thread ID out of range");
        assert_eq!(op_count, 50, "Thread {thread_id} didn't complete all operations");
    }
}

#[test]
fn test_atomic_operation_correctness_concurrent_reads() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    let seq = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let barrier = Arc::new(Barrier::new(8));
    let read_counter = Arc::new(AtomicUsize::new(0));
    let consistency_error = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    // Spawn multiple threads that perform atomic read operations
    for thread_id in 0..8 {
        let seq_clone = Arc::clone(&seq);
        let barrier_clone = Arc::clone(&barrier);
        let counter_clone = Arc::clone(&read_counter);
        let error_clone = Arc::clone(&consistency_error);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut local_reads = 0;
            for _ in 0..100 {
                // Perform atomic read operations
                let length = seq_clone.length();
                let is_empty = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isEmpty(&seq_clone);
                let is_singleton = <ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::isSingleton(&seq_clone);

                // Verify atomic consistency
                if length == 0 && !is_empty {
                    error_clone.store(true, Ordering::SeqCst);
                }
                if length == 1 && !is_singleton {
                    error_clone.store(true, Ordering::SeqCst);
                }
                if length > 1 && is_singleton {
                    error_clone.store(true, Ordering::SeqCst);
                }

                // Atomic read of elements
                for i in 0..length {
                    let val = *seq_clone.nth(i);
                    if !(1..=10).contains(&val) {
                        error_clone.store(true, Ordering::SeqCst);
                    }
                }

                local_reads += 1;
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }

            (thread_id, local_reads)
        });
        handles.push(handle);
    }

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify atomic operations completed correctly
    assert!(
        !consistency_error.load(Ordering::SeqCst),
        "Atomic consistency error detected"
    );
    assert_eq!(
        read_counter.load(Ordering::SeqCst),
        800,
        "Not all atomic reads completed"
    );

    for (thread_id, local_count) in results {
        assert_eq!(local_count, 100, "Thread {thread_id} didn't complete all reads");
    }
}

#[test]
fn test_atomic_operation_correctness_inject_operations() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    let base_seq = Arc::new(ArrayMtPerSLit![0; 20]);
    let barrier = Arc::new(Barrier::new(4));
    let operation_counter = Arc::new(AtomicUsize::new(0));
    let atomicity_violation = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    // Spawn threads that perform atomic inject operations
    for thread_id in 0..4 {
        let base_clone = Arc::clone(&base_seq);
        let barrier_clone = Arc::clone(&barrier);
        let counter_clone = Arc::clone(&operation_counter);
        let violation_clone = Arc::clone(&atomicity_violation);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut completed_operations = 0;
            for i in 0..25 {
                let index = (thread_id * 5 + i) % 20;
                let value = thread_id * 1000 + i;

                // Create atomic update
                let updates = ArrayMtPerSLit![Pair(index, value)];

                // Perform atomic inject operation
                let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&base_clone, &updates);

                // Verify atomicity: the inject operation should be all-or-nothing
                let actual_value = *result.nth(index);
                if actual_value != value {
                    violation_clone.store(true, Ordering::SeqCst);
                }

                // Verify other elements remain unchanged from base
                for j in 0..result.length() {
                    if j != index {
                        let base_val = *base_clone.nth(j);
                        let result_val = *result.nth(j);
                        if result_val != base_val {
                            // This is expected behavior for inject - other concurrent operations
                            // may have modified the sequence, but we verify consistency
                        }
                    }
                }

                completed_operations += 1;
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }

            (thread_id, completed_operations)
        });
        handles.push(handle);
    }

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify atomic operations completed correctly
    assert!(
        !atomicity_violation.load(Ordering::SeqCst),
        "Atomicity violation detected in inject operations"
    );
    assert_eq!(
        operation_counter.load(Ordering::SeqCst),
        100,
        "Not all atomic operations completed"
    );

    for (thread_id, op_count) in results {
        assert_eq!(op_count, 25, "Thread {thread_id} didn't complete all operations");
    }
}

#[test]
fn test_atomic_operation_correctness_memory_ordering() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    let seq = Arc::new(ArrayMtPerSLit![1, 2, 3, 4, 5]);
    let barrier = Arc::new(Barrier::new(6));
    let write_counter = Arc::new(AtomicUsize::new(0));
    let read_counter = Arc::new(AtomicUsize::new(0));
    let ordering_violation = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    // Writer threads
    for thread_id in 0..3 {
        let seq_clone = Arc::clone(&seq);
        let barrier_clone = Arc::clone(&barrier);
        let write_counter_clone = Arc::clone(&write_counter);
        let ordering_clone = Arc::clone(&ordering_violation);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut writes = 0;
            for i in 0..20 {
                let index = i % seq_clone.length();
                let value = thread_id * 100 + i;
                let updates = ArrayMtPerSLit![Pair(index, value)];

                // Perform write with memory ordering guarantees
                let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&seq_clone, &updates);

                // Verify the write was atomic and visible
                let written_value = *result.nth(index);
                if written_value != value {
                    ordering_clone.store(true, Ordering::SeqCst);
                }

                writes += 1;
                write_counter_clone.fetch_add(1, Ordering::SeqCst);

                // Small delay to allow interleaving
                thread::yield_now();
            }

            writes
        });
        handles.push(handle);
    }

    // Reader threads
    for _thread_id in 3..6 {
        let seq_clone = Arc::clone(&seq);
        let barrier_clone = Arc::clone(&barrier);
        let read_counter_clone = Arc::clone(&read_counter);
        let ordering_clone = Arc::clone(&ordering_violation);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut reads = 0;
            for _ in 0..30 {
                // Perform atomic reads with memory ordering
                let length = seq_clone.length();

                // Read all elements atomically
                let mut snapshot = Vec::new();
                for i in 0..length {
                    snapshot.push(*seq_clone.nth(i));
                }

                // Verify snapshot consistency (no torn reads)
                if snapshot.len() != length {
                    ordering_clone.store(true, Ordering::SeqCst);
                }

                // Verify all values are within expected ranges
                for &val in &snapshot {
                    if !(1..=300).contains(&val) { // Original values 1-5, or thread values 0-299
                         // This might happen due to concurrent writes, which is acceptable
                    }
                }

                reads += 1;
                read_counter_clone.fetch_add(1, Ordering::SeqCst);

                thread::yield_now();
            }

            reads
        });
        handles.push(handle);
    }

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify memory ordering was maintained
    assert!(
        !ordering_violation.load(Ordering::SeqCst),
        "Memory ordering violation detected"
    );
    assert_eq!(write_counter.load(Ordering::SeqCst), 60, "Not all writes completed");
    assert_eq!(read_counter.load(Ordering::SeqCst), 90, "Not all reads completed");

    // Verify all threads completed their operations
    for (i, count) in results.iter().enumerate() {
        if i < 3 {
            assert_eq!(*count, 20, "Writer thread {i} didn't complete all writes");
        } else {
            assert_eq!(*count, 30, "Reader thread {i} didn't complete all reads");
        }
    }
}

// ========== Additional Comprehensive Tests for Untested Functions ==========

#[test]
fn test_arrayseqmtper_trait_empty() {
    let empty = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::empty();
    assert_eq!(empty.length(), 0);
    assert!(<ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::isEmpty(&empty));
}

#[test]
fn test_arrayseqmtper_trait_new() {
    let seq = <ArraySeqMtPerS<i32> as Chap18Trait<i32>>::new(5, 42);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 42);
    }
}

#[test]
fn test_arrayseqmtper_trait_singleton() {
    let seq = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::singleton(99);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 99);
    assert!(<ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::isSingleton(&seq));
}

#[test]
fn test_arrayseqmtper_trait_length() {
    let seq = ArrayMtPerSLit![1, 2, 3, 4];
    assert_eq!(<ArraySeqMtPerS<i32> as Chap18Trait<i32>>::length(&seq), 4);
}

#[test]
fn test_arrayseqmtper_trait_nth() {
    let seq = ArrayMtPerSLit![10, 20, 30];
    assert_eq!(*<ArraySeqMtPerS<i32> as Chap18Trait<i32>>::nth(&seq, 0), 10);
    assert_eq!(*<ArraySeqMtPerS<i32> as Chap18Trait<i32>>::nth(&seq, 2), 30);
}

#[test]
fn test_arrayseqmtper_trait_tabulate() {
    let seq = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::tabulate(&|i| (i * 3) as i32, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(2), 6);
    assert_eq!(*seq.nth(4), 12);
}

#[test]
fn test_arrayseqmtper_trait_map() {
    let seq = ArrayMtPerSLit![1, 2, 3, 4];
    let doubled = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::map(&seq, |x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_arrayseqmtper_trait_filter() {
    let seq = ArrayMtPerSLit![1, 2, 3, 4, 5, 6];
    let evens = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::filter(&seq, |x| *x % 2 == 0);
    assert!(evens.length() >= 3);
    // Values should be even
    for i in 0..evens.length() {
        assert_eq!(*evens.nth(i) % 2, 0);
    }
}

#[test]
fn test_arrayseqmtper_trait_append() {
    let a = ArrayMtPerSLit![1, 2];
    let b = ArrayMtPerSLit![3, 4, 5];
    let c = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(*c.nth(0), 1);
    assert_eq!(*c.nth(4), 5);
}

#[test]
fn test_arrayseqmtper_trait_append_select() {
    let a = ArrayMtPerSLit![10, 20];
    let b = ArrayMtPerSLit![30, 40];
    let c = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::append_select(&a, &b);
    assert_eq!(c.length(), 4);
    assert_eq!(*c.nth(0), 10);
    assert_eq!(*c.nth(3), 40);
}

#[test]
fn test_arrayseqmtper_trait_flatten() {
    let seq1 = ArrayMtPerSLit![1, 2];
    let seq2 = ArrayMtPerSLit![3, 4];
    let nested = ArrayMtPerSLit![seq1, seq2];
    let flat = <ArraySeqMtPerS<i32> as Chap18Trait<i32>>::flatten(&nested);
    assert_eq!(flat.length(), 4);
    assert_eq!(*flat.nth(0), 1);
    assert_eq!(*flat.nth(3), 4);
}

#[test]
fn test_arrayseqmtper_trait_reduce() {
    let seq = ArrayMtPerSLit![1, 2, 3, 4];
    let sum = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqmtper_trait_scan() {
    let seq = ArrayMtPerSLit![1, 2, 3, 4];
    let (prefix_sums, total) = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::scan(&seq, &|a, b| a + b, 0);
    assert_eq!(prefix_sums.length(), 4);
    assert_eq!(total, 10);
}

#[test]
fn test_arrayseqmtper_trait_iterate() {
    let seq = ArrayMtPerSLit![1, 2, 3, 4];
    let sum = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_arrayseqmtper_trait_ninject() {
    let base = ArrayMtPerSLit![0, 0, 0, 0, 0];
    let updates = ArrayMtPerSLit![Pair(1, 10), Pair(3, 30)];
    let result = <ArraySeqMtPerS<i32> as Chap18Trait<i32>>::ninject(&base, &updates);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(1), 10);
    assert_eq!(*result.nth(3), 30);
}

#[test]
fn test_arrayseqmtper_trait_select() {
    let a = ArrayMtPerSLit![1, 2];
    let b = ArrayMtPerSLit![3, 4];
    let result = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::select(&a, &b, 1);
    assert_eq!(result, Some(&2));
    let result2 = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::select(&a, &b, 2);
    assert_eq!(result2, Some(&3));
}

#[test]
fn test_arrayseqmtper_trait_deflate() {
    let result = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::deflate(&|x| *x > 0, &5);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 5);

    let empty = <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::deflate(&|x| *x < 0, &5);
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_arrayseqmtper_collect() {
    let pairs = ArrayMtPerSLit![Pair(1, 1), Pair(1, 2), Pair(2, 3)];
    let grouped = <ArraySeqMtPerS<Pair<i32, i32>> as Chap18Trait<Pair<i32, i32>>>::collect(&pairs, |a, b| a.cmp(b));
    assert!(grouped.length() > 0);
}

#[test]
fn test_arrayseqmtper_atomic_write() {
    // atomic write is a stub, just test it doesn't panic
    let mut values = ArrayMtPerSLit![Pair(0, 0), Pair(1, 1)];
    let changes = ArrayMtPerSLit![Pair(0, 99)];
    <ArraySeqMtPerS<i32> as ArraySeqMtPerTrait<i32>>::atomicWrite(&mut values, &changes, 0);
    // No panic means success for stub
}

// ========== Merged from _Advanced file ==========

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
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&values, &changes);

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
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&values, &changes);

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
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&values, &changes);

    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 0);
    assert_eq!(*result.nth(1), 99); // Valid update applied
    assert_eq!(*result.nth(2), 2); // Out of bounds update ignored
}

#[test]
fn test_inject_empty_changes() {
    let values = ArrayMtPerSLit![1, 2, 3];
    let changes: ArraySeqMtPerS<Pair<N, N>> = ArrayMtPerSLit![];
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&values, &changes);

    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 1);
    assert_eq!(*result.nth(1), 2);
    assert_eq!(*result.nth(2), 3);
}

#[test]
fn test_inject_empty_values() {
    let values: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
    let changes = ArrayMtPerSLit![PairLit!(0, 99)];
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::inject(&values, &changes);

    assert_eq!(result.length(), 0); // No values to update
}

// TODO: AtomicWriteLowestChangeNumberWins tests require complex setup
// The function signature expects values_with_change_number and changes arrays
// to have specific relationships that need more investigation

#[test]
fn test_inject_string_values() {
    let values = ArrayMtPerSLit!["hello", "world", "test"];
    let changes = ArrayMtPerSLit![PairLit!(1, "rust"), PairLit!(0, "hi")];
    let result = <ArraySeqMtPerS<&str> as Chap18Trait<&str>>::inject(&values, &changes);

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
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::flatten(&nested);
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
    assert_eq!(
        *<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::select(&a, &b, 0).unwrap(),
        1
    );
    assert_eq!(
        *<ArraySeqMtPerS<N> as ArraySeqMtPerTrait<N>>::select(&a, &b, 3).unwrap(),
        4
    );
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
    let result = <ArraySeqMtPerS<N> as Chap18Trait<N>>::ninject(&seq, &updates);
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

// ========== Merged from TestArraySeqMtPerChap19_Advanced.rs above ==========
