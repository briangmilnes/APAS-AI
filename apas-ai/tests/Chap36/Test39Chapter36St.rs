//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod Test39Chapter36St {
    use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::Chap36::QuickSortSt::Chapter36St::Chapter36StTrait;
    use apas_ai::Types::Types::*;
    use apas_ai::*;

trait ToVec<T: StT> {
    fn to_vec(&self) -> Vec<T>;
}
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {
    fn to_vec(&self) -> Vec<T> { (0..self.length()).map(|i| self.nth(i).clone()).collect() }
}

#[test]
fn quick_sort_variants_produce_sorted_output() {
    let base = ArraySeqStEphS::from_vec(vec![5, 3, 1, 4, 2, 2, 3]);
    let mut first = base.clone();
    first.quick_sort_st_first();
    let mut median3 = base.clone();
    median3.quick_sort_st_median3();
    let mut random = base.clone();
    random.quick_sort_st_random();
    let expected = ArraySeqStEphS::from_vec(vec![1, 2, 2, 3, 3, 4, 5]);

    assert_eq!(first, expected);
    assert_eq!(median3, expected);
    assert_eq!(random, expected);
}

#[test]
fn quick_sort_handles_edge_cases() {
    let mut empty: ArraySeqStEphS<i32> = ArraySeqStEphS::from_vec(vec![]);
    empty.quick_sort_st_first();
    assert_eq!(empty.to_vec(), vec![]);

    let mut single = ArraySeqStEphS::from_vec(vec![42]);
    single.quick_sort_st_median3();
    assert_eq!(single.to_vec(), vec![42]);

    let mut sorted = ArraySeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
    sorted.quick_sort_st_random();
    assert_eq!(sorted.to_vec(), vec![1, 2, 3, 4, 5]);

    let mut reversed = ArraySeqStEphS::from_vec(vec![5, 4, 3, 2, 1]);
    reversed.quick_sort_st_first();
    assert_eq!(reversed.to_vec(), vec![1, 2, 3, 4, 5]);

    let mut pair = ArraySeqStEphS::from_vec(vec![2, 1]);
    pair.quick_sort_st_median3();
    assert_eq!(pair.to_vec(), vec![1, 2]);
}

#[test]
fn pivot_strategies_match_expectations() {
    let seq = ArraySeqStEphS::from_vec(vec![9, 1, 5, 3, 7]);
    assert_eq!(seq.pivot_st_first(0, seq.length()), 9);
    assert_eq!(seq.pivot_st_median3(0, seq.length()), 7);

    let random_seq = ArraySeqStEphS::from_vec(vec![10, 20, 30, 40, 50]);
    let pivot = random_seq.pivot_st_random(1, 4);
    let mut within_range = false;
    for idx in 1..4 {
        if random_seq.nth(idx) == &pivot {
            within_range = true;
            break;
        }
    }
    assert!(
        within_range,
        "random pivot should be drawn from the requested sub-range"
    );

    let median_case = ArraySeqStEphS::from_vec(vec![3, 8, 5, 6, 7]);
    let expected_mid = median_case.pivot_st_median3(0, median_case.length());
    assert_eq!(expected_mid, 5);
}

#[test]
fn quick_sort_small_inputs_use_shared_pivots() {
    let mut seq = ArraySeqStEphS::from_vec(vec![4, 1, 3]);
    let expected_pivot = seq.pivot_st_first(0, seq.length());
    assert_eq!(expected_pivot, 4);
    seq.quick_sort_st_first();
    assert_eq!(seq.to_vec(), vec![1, 3, 4]);

    let mut seq_med = ArraySeqStEphS::from_vec(vec![8, 2, 7, 1, 5]);
    let expected_med = seq_med.pivot_st_median3(0, seq_med.length());
    assert_eq!(expected_med, 7);
    seq_med.quick_sort_st_median3();
    assert_eq!(seq_med.to_vec(), vec![1, 2, 5, 7, 8]);
}

}
