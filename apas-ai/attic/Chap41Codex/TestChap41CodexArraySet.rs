use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41Codex::ArraySetEnumMtEph::ArraySetEnumMtEph::{
    ArraySetEnumMtEph, ArraySetEnumMtEphTrait,
};
use apas_ai::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::{
    ArraySetEnumStEph, ArraySetEnumStEphTrait,
};
use apas_ai::Types::Types::N;

fn seq_values_st(seq: &ArraySeqStEphS<N>) -> Vec<N> {
    let mut out = Vec::with_capacity(seq.length());
    for i in 0..seq.length() {
        out.push(*seq.nth(i));
    }
    out
}

fn seq_values_mt(seq: &ArraySeqMtEphS<N>) -> Vec<N> {
    seq.iter_cloned()
}

#[test]
fn array_set_st_example_41_1() {
    let universe = 32;
    let mut abc = ArraySetEnumStEph::empty(universe);
    abc.insert(1);
    abc.insert(2);
    abc.insert(3);
    assert_eq!(abc.size(), 3);

    let from_seq = ArraySetEnumStEph::from_seq(
        universe,
        &ArraySeqStEphS::from_vec(vec![4, 11, 2, 6]),
    );
    let filtered = from_seq.filter(|x| x < 7);
    assert_eq!(seq_values_st(&filtered.to_seq()), vec![2, 4, 6]);

    let finder = ArraySetEnumStEph::from_seq(
        universe,
        &ArraySeqStEphS::from_vec(vec![6, 2, 9, 11, 8]),
    );
    assert!(!finder.find(4));

    let left = ArraySetEnumStEph::from_seq(
        universe,
        &ArraySeqStEphS::from_vec(vec![2, 7, 8, 11]),
    );
    let right = ArraySetEnumStEph::from_seq(
        universe,
        &ArraySeqStEphS::from_vec(vec![7, 9, 11, 14, 17]),
    );
    let union = left.union(&right);
    assert_eq!(seq_values_st(&union.to_seq()), vec![2, 7, 8, 9, 11, 14, 17]);

    let to_seq = left.to_seq();
    assert_eq!(seq_values_st(&to_seq), vec![2, 7, 8, 11]);

    let from_seq_again = ArraySetEnumStEph::from_seq(
        universe,
        &ArraySeqStEphS::from_vec(vec![2, 7, 2, 8, 11, 2]),
    );
    assert_eq!(seq_values_st(&from_seq_again.to_seq()), vec![2, 7, 8, 11]);
}

#[test]
fn array_set_mt_parallel_filter() {
    let universe = 32;
    let original = ArraySetEnumMtEph::from_seq(
        universe,
        &ArraySeqMtEphS::from_vec(vec![1, 3, 5, 7, 9]),
    );
    let filtered = original.filter(|x| x % 2 == 1 && x < 7);
    let filtered_seq = filtered.to_seq();
    assert_eq!(seq_values_mt(&filtered_seq), vec![1, 3, 5]);
}
