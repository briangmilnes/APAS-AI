//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Types::Types::Pair;
use apas_ai::Types::Types::*; // macro import

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            | 0 => 0,
            | 1 => 1,
            | _ => fib(n - 1) + fib(n - 2),
        }
    }
    let a: ArraySeqStPerS<N> = ArraySeqStPerS::tabulate(&fib, 10);
    let fib10_head = ArraySeqStPerSLit![
        *a.nth(0),
        *a.nth(1),
        *a.nth(2),
        *a.nth(3),
        *a.nth(4),
        *a.nth(5),
        *a.nth(6),
        *a.nth(7),
        *a.nth(8),
        *a.nth(9)
    ];
    assert_eq!(fib10_head, ArraySeqStPerSLit![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let b = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&a, &|x| x + 1);
    assert_eq!(b, ArraySeqStPerSLit![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = ArraySeqStPerSLit![10, 20, 30, 40, 50];
    let b = a.subseq_copy(1, 3);
    assert_eq!(b, ArraySeqStPerSLit![20, 30, 40]);
    let c = a.subseq_copy(2, 0);
    assert_eq!(c.length(), 0);
    let d = a.subseq_copy(0, 1);
    assert_eq!(d, ArraySeqStPerSLit![10]);
}

#[test]
fn test_append() {
    let a = ArraySeqStPerSLit![1, 2, 3];
    let b = ArraySeqStPerSLit![4, 5, 6];
    let c = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&a, &b);
    assert_eq!(c, ArraySeqStPerSLit![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySeqStPerSLit![1, 2, 3];
    let b = ArraySeqStPerSLit![4, 5];
    let c = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&a, &b);
    assert_eq!(c, ArraySeqStPerSLit![1, 2, 3, 4, 5]);
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    let d = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::append(&ArraySeqStPerSLit![1, 2, 3], &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::append(&empty, &ArraySeqStPerSLit![1, 2, 3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&numbers, &|&x| {
        if x % 2 == 0 {
            true
        } else {
            false
        }
    });
    assert_eq!(evens, ArraySeqStPerSLit![2, 4, 6, 8, 10]);
    let odds_only = ArraySeqStPerSLit![1, 3, 5, 7];
    let no_evens = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(&odds_only, &|&x| {
        if x % 2 == 0 {
            true
        } else {
            false
        }
    });
    assert_eq!(no_evens.length(), 0);
}

// TODO: Fix flatten type inference - ArraySeqStPerS<ArraySeqStPerS<N>> trait bound issues
#[test]
fn test_flatten() {
    let a: ArraySeqStPerS<N> = ArraySeqStPerSLit![1, 2];
    let b: ArraySeqStPerS<N> = ArraySeqStPerSLit![3, 4, 5];
    let c: ArraySeqStPerS<N> = ArraySeqStPerSLit![6];
    let sequences: ArraySeqStPerS<ArraySeqStPerS<N>> = ArraySeqStPerSLit![a, b, c];
    let flattened = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&sequences);
    assert_eq!(flattened, ArraySeqStPerSLit![1, 2, 3, 4, 5, 6]);
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    let mixed: ArraySeqStPerS<ArraySeqStPerS<N>> =
        ArraySeqStPerSLit![ArraySeqStPerSLit![1, 2], empty, ArraySeqStPerSLit![3]];
    let mixed_flat = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySeqStPerSLit![1, 2, 3]);
}

#[test]
fn test_inject_and_ninject() {
    let a = ArraySeqStPerSLit!["the", "cat", "in", "the", "hat"];
    let updates = ArraySeqStPerSLit![Pair(0, "a"), Pair(2, "on"), Pair(4, "mat")];
    let injected = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, ArraySeqStPerSLit!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = ArraySeqStPerSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")];
    let result_first = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::inject(&a, &conflicting_updates);
    assert_eq!(result_first, ArraySeqStPerSLit!["first", "updated", "in", "the", "hat"]);

    let ninjected = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::ninject(
        &a,
        &ArraySeqStPerSLit![Pair(1, "dog"), Pair(3, "big"), Pair(6, "hog")],
    );
    assert_eq!(ninjected, ArraySeqStPerSLit!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArraySeqStPerS<&str> as ArraySeqStPerTrait<&str>>::ninject(
        &a,
        &ArraySeqStPerSLit![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")],
    );
    assert_eq!(result_last, ArraySeqStPerSLit!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
    let empty_result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySeqStPerSLit![100];
    let single_result = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::iterate(&numbers, &sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArraySeqStPerSLit![1, 2, 3];
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    // Use N, N pairs since collect requires same types
    let pairs = ArraySeqStPerSLit![
        Pair(1_usize, 10_usize),
        Pair(2_usize, 20_usize),
        Pair(1_usize, 30_usize)
    ];
    let grouped: ArraySeqStPerS<Pair<N, ArraySeqStPerS<N>>> =
        <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let pair0 = grouped.nth(0);
    assert_eq!(pair0.0, 1);
    assert_eq!(pair0.1, ArraySeqStPerSLit![10, 30]);
    let pair1 = grouped.nth(1);
    assert_eq!(pair1.0, 2);
    assert_eq!(pair1.1, ArraySeqStPerSLit![20]);
}

#[test]
fn test_arrayseqstper_iter() {
    let seq = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let collected: Vec<i32> = seq.iter().cloned().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);

    let empty: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();
    let empty_collected: Vec<i32> = empty.iter().cloned().collect();
    assert_eq!(empty_collected, Vec::<i32>::new());
}

#[test]
#[should_panic]
fn test_arrayseqstper_nth_out_of_bounds() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    let _ = seq.nth(5); // Should panic - index out of bounds
}

#[test]
fn test_arrayseqstper_subseq_invalid_start() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    let result = seq.subseq_copy(5, 1); // Start index out of bounds - returns empty
    assert_eq!(result.length(), 0);
}

#[test]
fn test_arrayseqstper_subseq_invalid_length() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    let result = seq.subseq_copy(1, 10); // Length extends beyond bounds - clamps to available
    assert_eq!(result.length(), 2); // Should return elements from index 1 to end
    assert_eq!(result, ArraySeqStPerSLit![2, 3]);
}

#[test]
fn test_arrayseqstper_inject_invalid_index() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    let updates = ArraySeqStPerSLit![Pair(5, 99)]; // Index 5 is out of bounds
    let result = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::inject(&seq, &updates);
    // Should silently skip out-of-bounds indices, leaving sequence unchanged
    assert_eq!(result, seq);
}

#[test]
#[should_panic]
fn test_arrayseqstper_nth_panic_outofbounds() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    let _ = seq.nth(10); // Index 10 is way out of bounds - should panic
}

#[test]
fn test_arrayseqstper_subseq_overflow_graceful() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    // APAS style: handles overflow gracefully by returning empty or clamping
    let result = seq.subseq_copy(1, usize::MAX);
    // Should either return empty or clamp to available elements
    assert!(result.length() <= 2); // At most 2 elements from index 1 onwards
}

#[test]
#[should_panic]
fn test_arrayseqstper_nth_panic_empty() {
    let empty: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();
    let _ = empty.nth(0); // Any index on empty sequence should panic
}

#[test]
fn test_arrayseqstper_empty_operations_comprehensive() {
    let empty: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();

    // Basic properties
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), true);
    assert_eq!(empty.isSingleton(), false);

    // Operations on empty sequence should return empty or appropriate defaults
    let empty_subseq = empty.subseq_copy(0, 0);
    assert_eq!(empty_subseq.length(), 0);

    let empty_subseq2 = empty.subseq_copy(0, 10);
    assert_eq!(empty_subseq2.length(), 0);

    // Map on empty sequence should return empty
    let mapped = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&empty, &|x| x * 2);
    assert_eq!(mapped.length(), 0);

    // Filter on empty sequence should return empty
    let filtered =
        <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::filter(&empty, &|x| if *x > 0 { true } else { false });
    assert_eq!(filtered.length(), 0);

    // Reduce on empty sequence should return base value
    let reduced = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::reduce(&empty, &|a, b| a + b, 42);
    assert_eq!(reduced, 42);

    // Scan on empty sequence should return sequence with base value
    let scanned = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::scan(&empty, &|a, b| a + b, 0);
    assert_eq!(scanned.0.length(), 1); // scan returns (sequence with base, final_value)
    assert_eq!(*scanned.0.nth(0), 0); // Should contain the base value

    // Append with empty should work
    let non_empty = ArraySeqStPerSLit![1, 2, 3];
    let appended1 = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&empty, &non_empty);
    assert_eq!(appended1.length(), 3);
    assert_eq!(appended1, non_empty);

    let appended2 = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&non_empty, &empty);
    assert_eq!(appended2.length(), 3);
    assert_eq!(appended2, non_empty);

    // Inject on empty sequence with empty updates should return empty
    let empty_updates: ArraySeqStPerS<Pair<usize, i32>> = ArraySeqStPerS::empty();
    let injected = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::inject(&empty, &empty_updates);
    assert_eq!(injected.length(), 0);

    // Iterator on empty sequence should be empty
    let collected: Vec<i32> = empty.iter().cloned().collect();
    assert_eq!(collected.len(), 0);
}

#[test]
fn test_arrayseqstper_single_element_boundary() {
    // Test single element sequence operations
    let single = ArraySeqStPerSLit![42];

    // Basic properties
    assert_eq!(single.length(), 1);
    assert_eq!(single.isEmpty(), false);
    assert_eq!(single.isSingleton(), true);

    // Access operations
    assert_eq!(*single.nth(0), 42);

    // Subseq operations
    let full_subseq = single.subseq_copy(0, 1);
    assert_eq!(full_subseq.length(), 1);
    assert_eq!(*full_subseq.nth(0), 42);

    let empty_subseq = single.subseq_copy(1, 1);
    assert_eq!(empty_subseq.length(), 0);

    let zero_length_subseq = single.subseq_copy(0, 0);
    assert_eq!(zero_length_subseq.length(), 0);

    // Map operations
    let mapped = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&single, &|x| x * 2);
    assert_eq!(mapped.length(), 1);
    assert_eq!(*mapped.nth(0), 84);

    // Filter operations
    let filtered_true =
        <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::filter(&single, &|x| if *x > 0 { true } else { false });
    assert_eq!(filtered_true.length(), 1);
    assert_eq!(*filtered_true.nth(0), 42);

    let filtered_false =
        <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::filter(&single, &|x| if *x > 100 { true } else { false });
    assert_eq!(filtered_false.length(), 0);

    // Reduce operations
    let reduced = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::reduce(&single, &|a, b| a + b, 0);
    assert_eq!(reduced, 42);

    // Scan operations
    let scanned = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::scan(&single, &|a, b| a + b, 0);
    assert_eq!(scanned.0.length(), 1); // Just the prefix sum result
    assert_eq!(*scanned.0.nth(0), 0); // Base value is returned as first element

    // Append operations
    let empty: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();
    let appended1 = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&single, &empty);
    assert_eq!(appended1.length(), 1);
    assert_eq!(appended1, single);

    let appended2 = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&empty, &single);
    assert_eq!(appended2.length(), 1);
    assert_eq!(appended2, single);

    // Inject operations
    let updates = ArraySeqStPerSLit![Pair(0, 99)];
    let injected = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::inject(&single, &updates);
    assert_eq!(injected.length(), 1);
    assert_eq!(*injected.nth(0), 99);

    // Iterator operations
    let collected: Vec<i32> = single.iter().cloned().collect();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0], 42);

    // Equality operations
    let single_same = ArraySeqStPerSLit![42];
    assert_eq!(single, single_same);

    let single_diff = ArraySeqStPerSLit![43];
    assert_ne!(single, single_diff);

    assert_ne!(single, empty);
}

#[test]
fn test_arrayseqstper_zero_length_operations() {
    // Test zero-length subseq operations
    let seq = ArraySeqStPerSLit![1, 2, 3, 4, 5];

    // Zero-length subseq at start
    let zero_start = seq.subseq_copy(0, 0);
    assert_eq!(zero_start.length(), 0);
    assert_eq!(zero_start.isEmpty(), true);

    // Zero-length subseq in middle
    let zero_middle = seq.subseq_copy(2, 0);
    assert_eq!(zero_middle.length(), 0);
    assert_eq!(zero_middle.isEmpty(), true);

    // Zero-length subseq at end
    let zero_end = seq.subseq_copy(5, 0);
    assert_eq!(zero_end.length(), 0);
    assert_eq!(zero_end.isEmpty(), true);

    // Zero-length subseq beyond end should still return empty
    let zero_beyond = seq.subseq_copy(10, 0);
    assert_eq!(zero_beyond.length(), 0);
    assert_eq!(zero_beyond.isEmpty(), true);

    // Test with single element sequence
    let single = ArraySeqStPerSLit![42];
    let zero_single_start = single.subseq_copy(0, 0);
    assert_eq!(zero_single_start.length(), 0);
    assert_eq!(zero_single_start.isEmpty(), true);

    let zero_single_end = single.subseq_copy(1, 0);
    assert_eq!(zero_single_end.length(), 0);
    assert_eq!(zero_single_end.isEmpty(), true);

    // Test with empty sequence
    let empty: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();
    let zero_empty = empty.subseq_copy(0, 0);
    assert_eq!(zero_empty.length(), 0);
    assert_eq!(zero_empty.isEmpty(), true);

    // All zero-length subsequences should be equal to empty
    assert_eq!(zero_start, empty);
    assert_eq!(zero_middle, empty);
    assert_eq!(zero_end, empty);
    assert_eq!(zero_beyond, empty);
    assert_eq!(zero_single_start, empty);
    assert_eq!(zero_single_end, empty);
    assert_eq!(zero_empty, empty);

    // Test operations on zero-length subsequences
    let mapped_zero = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&zero_start, &|x| x * 2);
    assert_eq!(mapped_zero.length(), 0);

    let filtered_zero = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::filter(&zero_start, &|x| {
        if *x > 0 {
            true
        } else {
            false
        }
    });
    assert_eq!(filtered_zero.length(), 0);

    let reduced_zero = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::reduce(&zero_start, &|a, b| a + b, 100);
    assert_eq!(reduced_zero, 100); // Should return base value

    let scanned_zero = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::scan(&zero_start, &|a, b| a + b, 50);
    assert_eq!(scanned_zero.0.length(), 1); // Should contain base value
    assert_eq!(*scanned_zero.0.nth(0), 50);

    // Iterator on zero-length should be empty
    let collected: Vec<i32> = zero_start.iter().cloned().collect();
    assert_eq!(collected.len(), 0);
}

#[test]
fn test_arrayseqstper_iterator_boundaries() {
    // Test iterator at beginning/end boundaries
    let seq = ArraySeqStPerSLit![10, 20, 30, 40, 50];

    // Test iterator starting from beginning
    let mut iter = seq.iter();
    assert_eq!(iter.next(), Some(&10)); // First element
    assert_eq!(iter.next(), Some(&20)); // Second element

    // Test iterator ending at end
    let iter_end = seq.iter();
    let collected: Vec<i32> = iter_end.cloned().collect();
    assert_eq!(collected.len(), 5);
    assert_eq!(collected[0], 10); // First boundary
    assert_eq!(collected[4], 50); // Last boundary

    // Test iterator on single element - both beginning and end
    let single = ArraySeqStPerSLit![99];
    let mut single_iter = single.iter();
    assert_eq!(single_iter.next(), Some(&99)); // Beginning = end
    assert_eq!(single_iter.next(), None); // Past end

    // Test iterator on empty sequence - no boundaries
    let empty: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();
    let mut empty_iter = empty.iter();
    assert_eq!(empty_iter.next(), None); // No beginning

    // Test iterator exhaustion and reset
    let seq_reset = ArraySeqStPerSLit![1, 2];
    let mut iter1 = seq_reset.iter();
    // Exhaust iterator by consuming all elements
    assert_eq!(iter1.next(), Some(&1));
    assert_eq!(iter1.next(), Some(&2));
    assert_eq!(iter1.next(), None); // Should be exhausted

    // New iterator should start fresh at beginning
    let mut iter2 = seq_reset.iter();
    assert_eq!(iter2.next(), Some(&1)); // Fresh start at beginning

    // Test iterator with functional operations at boundaries
    let seq_func = ArraySeqStPerSLit![100, 200, 300];

    // First element via iterator
    let first = seq_func.iter().next();
    assert_eq!(first, Some(&100));

    // Last element via iterator
    let last = seq_func.iter().last();
    assert_eq!(last, Some(&300));

    // Count elements via iterator
    let count = seq_func.iter().count();
    assert_eq!(count, 3);

    // Test iterator chain boundaries
    let seq1 = ArraySeqStPerSLit![1, 2];
    let seq2 = ArraySeqStPerSLit![3, 4];
    let chained: Vec<i32> = seq1.iter().chain(seq2.iter()).cloned().collect();
    assert_eq!(chained.len(), 4);
    assert_eq!(chained[0], 1); // First from seq1
    assert_eq!(chained[3], 4); // Last from seq2

    // Test iterator skip/take boundaries
    let seq_skip = ArraySeqStPerSLit![10, 20, 30, 40, 50];
    let skipped: Vec<i32> = seq_skip.iter().skip(2).cloned().collect();
    assert_eq!(skipped.len(), 3);
    assert_eq!(skipped[0], 30); // First after skip

    let taken: Vec<i32> = seq_skip.iter().take(3).cloned().collect();
    assert_eq!(taken.len(), 3);
    assert_eq!(taken[2], 30); // Last taken element
}

#[test]
fn test_arrayseqstper_maximum_size_boundary() {
    // Test maximum size collection boundary - use reasonably large size
    // to verify graceful handling without causing memory issues
    let large_size = 50_000usize;
    let large_vec: Vec<i32> = (0..large_size as i32).collect();
    let large_seq = ArraySeqStPerS::from_vec(large_vec);

    // Verify basic operations work on large sequence
    assert_eq!(large_seq.length(), large_size);
    assert_eq!(*large_seq.nth(0), 0);
    assert_eq!(*large_seq.nth(large_size - 1), (large_size - 1) as i32);
    assert_eq!(large_seq.isEmpty(), false);
    assert_eq!(large_seq.isSingleton(), false);

    // Test subseq_copy on large sequence
    let large_subseq = large_seq.subseq_copy(1000, 5000);
    assert_eq!(large_subseq.length(), 5000);
    assert_eq!(*large_subseq.nth(0), 1000);
    assert_eq!(*large_subseq.nth(4999), 5999);

    // Test map operation on large sequence (sample)
    let mapped_large = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::map(&large_seq, &|x| x * 2);
    assert_eq!(mapped_large.length(), large_size);
    assert_eq!(*mapped_large.nth(0), 0);
    assert_eq!(*mapped_large.nth(1), 2);
    assert_eq!(*mapped_large.nth(large_size - 1), ((large_size - 1) as i32) * 2);

    // Test filter operation on large sequence (sample)
    let filtered_large = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::filter(&large_seq, &|x| {
        if *x % 1000 == 0 {
            true
        } else {
            false
        }
    });
    assert_eq!(filtered_large.length(), 50); // 0, 1000, 2000, ..., 49000
    assert_eq!(*filtered_large.nth(0), 0);
    assert_eq!(*filtered_large.nth(1), 1000);

    // Test reduce operation on large sequence
    let sum_large = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::reduce(&large_seq, &|a, b| a + b, 0);
    let expected_sum = (large_size * (large_size - 1) / 2) as i32; // Sum of 0 to n-1
    assert_eq!(sum_large, expected_sum);

    // Test iterator on large sequence (sample check)
    let mut count = 0;
    for val in large_seq.iter() {
        if count < 10 {
            assert_eq!(*val, count as i32);
        }
        count += 1;
        if count > large_size + 100 {
            // Safety check
            break;
        }
    }
    assert_eq!(count, large_size);

    // Test append with large sequences
    let medium_seq = ArraySeqStPerS::from_vec((0..1000).collect());
    let appended_large = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&medium_seq, &large_seq);
    assert_eq!(appended_large.length(), 1000 + large_size);
    assert_eq!(*appended_large.nth(0), 0); // From medium_seq
    assert_eq!(*appended_large.nth(999), 999); // Last from medium_seq
    assert_eq!(*appended_large.nth(1000), 0); // First from large_seq

    // Test inject on large sequence (sample)
    let updates = ArraySeqStPerSLit![Pair(0, 999), Pair(large_size - 1, 888)];
    let injected_large = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::inject(&large_seq, &updates);
    assert_eq!(injected_large.length(), large_size);
    assert_eq!(*injected_large.nth(0), 999); // Updated
    assert_eq!(*injected_large.nth(large_size - 1), 888); // Updated
    assert_eq!(*injected_large.nth(1), 1); // Unchanged
}
