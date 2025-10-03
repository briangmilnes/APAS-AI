//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Types::Types::*;

#[test]
fn arrayseq_steph_basic_macros() {
    let mut seq = ArraySeqStEphSLit![1, 2, 3];
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::length(&seq), 3);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&seq, 0), &1);
    let _ = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::set(&mut seq, 1, 42);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&seq, 1), &42);
}

#[test]
fn arrayseq_steph_full_pipeline() {
    let seq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::tabulate(&|i| (i + 1) as i32, 5);
    let mapped = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::map(&seq, &|value| value * 2);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&mapped, 4), &10);

    let subseq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::subseq(&mapped, 1, 3);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&subseq, 2), &8);

    let appended = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::append(&mapped, &ArraySeqStEphSLit![99]);
    let evens = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::filter(&appended, &|value| {
        if *value % 2 == 0 { true } else { false }
    });
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::length(&evens), 5);

    let nested = ArraySeqStEphSLit![
        ArraySeqStEphSLit![1, 2],
        ArraySeqStEphSLit![3],
        ArraySeqStEphSLit![4, 5]
    ];
    let flattened = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::flatten(&nested);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&flattened, 3), &4);

    let mut writable = flattened.clone();
    let _ = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::update(&mut writable, Pair(0, 100));
    assert_eq!(
        <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&writable, 0),
        &100
    );

    let _ = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::inject(
        &mut writable,
        &ArraySeqStEphSLit![Pair(0, 7), Pair(0, 9), Pair(4, 11)],
    );
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&writable, 0), &9);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&writable, 4), &11);

    let collected = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::collect(
        &ArraySeqStEphSLit![Pair("x", 1), Pair("y", 2), Pair("x", 3)],
        |lhs, rhs| lhs.cmp(rhs),
    );
    assert_eq!(<ArraySeqStEphS<Pair<&str, ArraySeqStEphS<i32>>> as ArraySeqStEphTrait<Pair<&str, ArraySeqStEphS<i32>>>>::length(&collected), 2);

    let iter_sum = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::iterate(&flattened, &|acc, item| acc + item, 0);
    assert_eq!(iter_sum, 15);

    let reduced = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::reduce(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(reduced, 15);

    let (prefixes, total) =
        <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::scan(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(total, 15);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&prefixes, 2), &3);
}

#[test]
fn test_arrayseqsteph_iter() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let collected: Vec<i32> = seq.iter().cloned().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);

    let empty: ArraySeqStEphS<i32> = ArraySeqStEphS::empty();
    let empty_collected: Vec<i32> = empty.iter().cloned().collect();
    assert_eq!(empty_collected, vec![]);
}

#[test]
fn test_arrayseqsteph_outofbounds_graceful() {
    let mut seq = ArraySeqStEphSLit![1, 2, 3];

    // APAS style: bad arguments produce empty sequences or are ignored, not panics
    // Out-of-bounds set should return error but not panic
    let result = seq.set(10, 99); // Index 10 is out of bounds
    assert!(result.is_err()); // Should return an error, not panic

    // Out-of-bounds update should be silently ignored (APAS style)
    let original_values = [*seq.nth(0), *seq.nth(1), *seq.nth(2)];
    let _ = seq.update(Pair(10, 99)); // Index 10 is out of bounds - should be ignored
    // Verify sequence is unchanged
    assert_eq!(*seq.nth(0), original_values[0]);
    assert_eq!(*seq.nth(1), original_values[1]);
    assert_eq!(*seq.nth(2), original_values[2]);
}

#[test]
fn test_arrayseqsteph_subseq_graceful() {
    let seq = ArraySeqStEphSLit![1, 2, 3];

    // APAS style: invalid subseq parameters return empty sequences
    let result1 = seq.subseq(10, 1); // Start index out of bounds
    assert_eq!(result1.length(), 0);

    let result2 = seq.subseq(1, 10); // Length extends beyond bounds - should clamp
    assert_eq!(result2.length(), 2); // Should return elements from index 1 to end
    assert_eq!(*result2.nth(0), 2);
    assert_eq!(*result2.nth(1), 3);
}

#[test]
fn test_arrayseqsteph_subseq_overflow_graceful() {
    let seq = ArraySeqStEphSLit![1, 2, 3];
    // APAS style: handles overflow gracefully
    let result = seq.subseq(usize::MAX, usize::MAX);
    // Should return empty sequence for out-of-bounds start
    assert_eq!(result.length(), 0);
}

#[test]
#[should_panic]
fn test_arrayseqsteph_nth_panic_outofbounds() {
    let seq = ArraySeqStEphSLit![1, 2, 3];
    let _ = seq.nth(10); // Index 10 is way out of bounds - should panic
}

#[test]
#[should_panic]
fn test_arrayseqsteph_nth_panic_empty() {
    let empty: ArraySeqStEphS<i32> = ArraySeqStEphS::empty();
    let _ = empty.nth(0); // Any index on empty sequence should panic
}

#[test]
fn test_arrayseqsteph_iterator_boundaries() {
    // Test iterator at beginning/end boundaries
    let seq = ArraySeqStEphSLit![10, 20, 30, 40, 50];

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
    let single = ArraySeqStEphSLit![99];
    let mut single_iter = single.iter();
    assert_eq!(single_iter.next(), Some(&99)); // Beginning = end
    assert_eq!(single_iter.next(), None); // Past end

    // Test iterator on empty sequence - no boundaries
    let empty: ArraySeqStEphS<i32> = ArraySeqStEphS::empty();
    let mut empty_iter = empty.iter();
    assert_eq!(empty_iter.next(), None); // No beginning

    // Test iterator exhaustion and reset
    let seq_reset = ArraySeqStEphSLit![1, 2];
    let mut iter1 = seq_reset.iter();
    // Exhaust iterator by consuming all elements
    assert_eq!(iter1.next(), Some(&1));
    assert_eq!(iter1.next(), Some(&2));
    assert_eq!(iter1.next(), None); // Should be exhausted

    // New iterator should start fresh at beginning
    let mut iter2 = seq_reset.iter();
    assert_eq!(iter2.next(), Some(&1)); // Fresh start at beginning

    // Test iterator with functional operations at boundaries
    let seq_func = ArraySeqStEphSLit![100, 200, 300];

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
    let seq1 = ArraySeqStEphSLit![1, 2];
    let seq2 = ArraySeqStEphSLit![3, 4];
    let chained: Vec<i32> = seq1.iter().chain(seq2.iter()).cloned().collect();
    assert_eq!(chained.len(), 4);
    assert_eq!(chained[0], 1); // First from seq1
    assert_eq!(chained[3], 4); // Last from seq2

    // Test iterator skip/take boundaries
    let seq_skip = ArraySeqStEphSLit![10, 20, 30, 40, 50];
    let skipped: Vec<i32> = seq_skip.iter().skip(2).cloned().collect();
    assert_eq!(skipped.len(), 3);
    assert_eq!(skipped[0], 30); // First after skip

    let taken: Vec<i32> = seq_skip.iter().take(3).cloned().collect();
    assert_eq!(taken.len(), 3);
    assert_eq!(taken[2], 30); // Last taken element
}
