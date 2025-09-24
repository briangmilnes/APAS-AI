//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use rand::{Rng, rng};

use apas_ai::Chapter36MtSlice::Chapter36MtSlice::Chapter36MtSliceTrait;
use apas_ai::*;

fn to_vec<T: StT>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> { a.to_vec() }

fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }

fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> { ArraySeqMtEphSliceS::from_vec(data.to_vec()) }

#[test]
fn quick_sort_slice_variants_produce_sorted_output() {
    let base = mk_seq(&[5, 3, 1, 4, 2, 2, 3]);
    let expected = vec![1, 2, 2, 3, 3, 4, 5];

    let first = base.clone();
    first.quick_sort_mt_first();
    assert_eq!(to_vec(&first), expected);

    let median3 = base.clone();
    median3.quick_sort_mt_median3();
    assert_eq!(to_vec(&median3), expected);

    let random = base.clone();
    random.quick_sort_mt_random();
    assert_eq!(to_vec(&random), expected);
}

#[test]
fn quick_sort_slice_edge_cases() {
    let empty = ArraySeqMtEphSliceS::from_vec(Vec::<i32>::new());
    empty.quick_sort_mt_first();
    assert!(to_vec(&empty).is_empty());

    let single = mk_seq(&[42]);
    single.quick_sort_mt_median3();
    assert_eq!(to_vec(&single), vec![42]);

    let sorted = mk_seq(&[1, 2, 3, 4, 5]);
    sorted.quick_sort_mt_random();
    assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);

    let reversed = mk_seq(&[5, 4, 3, 2, 1]);
    reversed.quick_sort_mt_first();
    assert_eq!(to_vec(&reversed), vec![1, 2, 3, 4, 5]);

    let pair = mk_seq(&[2, 1]);
    pair.quick_sort_mt_median3();
    assert_eq!(to_vec(&pair), vec![1, 2]);
}

#[test]
fn quick_sort_slice_large_inputs() {
    let descending = ArraySeqMtEphSliceS::from_vec((0..4_096).rev().collect());
    descending.quick_sort_mt_first();
    assert!(is_sorted(&to_vec(&descending)));

    let mut rng = rng();
    let random_data: Vec<i32> = (0..4_096).map(|_| rng.random_range(-10_000..10_000)).collect();
    let random_seq = ArraySeqMtEphSliceS::from_vec(random_data);
    random_seq.quick_sort_mt_random();
    assert!(is_sorted(&to_vec(&random_seq)));
}

#[test]
fn slice_pivot_strategies_match_expectations() {
    let seq = mk_seq(&[9, 1, 5, 3, 7]);
    assert_eq!(seq.pivot_mt_first(0, seq.length()), 9);

    let median_case = mk_seq(&[3, 8, 5, 6, 7]);
    assert_eq!(median_case.pivot_mt_median3(0, median_case.length()), 5);

    let random_seq = mk_seq(&[10, 20, 30, 40, 50]);
    let pivot = random_seq.pivot_mt_random(1, 4);
    let mut within = false;
    for idx in 1..4 {
        if random_seq.nth_cloned(idx) == pivot {
            within = true;
            break;
        }
    }
    assert!(within, "random pivot should be drawn from the requested sub-range");
}

#[test]
fn quick_sort_slice_small_inputs_use_shared_pivots() {
    let seq = mk_seq(&[4, 1, 3]);
    assert_eq!(seq.pivot_mt_first(0, seq.length()), 4);
    seq.quick_sort_mt_first();
    assert_eq!(to_vec(&seq), vec![1, 3, 4]);

    let seq_med = mk_seq(&[8, 2, 7, 1, 5]);
    assert_eq!(seq_med.pivot_mt_median3(0, seq_med.length()), 7);
    seq_med.quick_sort_mt_median3();
    assert_eq!(to_vec(&seq_med), vec![1, 2, 5, 7, 8]);
}
