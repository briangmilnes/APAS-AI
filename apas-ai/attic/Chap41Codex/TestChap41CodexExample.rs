use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41Codex::ArraySetEnumMtEph::ArraySetEnumMtEph::{
    ArraySetEnumMtEph, ArraySetEnumMtEphTrait,
};
use apas_ai::Chap41Codex::AVLTreeSetMtEph::AVLTreeSetMtEph::{
    AVLTreeSetMtEph, AVLTreeSetMtEphTrait,
};
use apas_ai::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{
    AVLTreeSetStEph, AVLTreeSetStEphTrait,
};
use apas_ai::Chap41Codex::Example41_3::Example41_3::{
    array_enum_from_seq, avl_from_seq, avl_mt_from_seq,
};
use apas_ai::Types::Types::*;

#[test]
fn example_41_3_equivalence() {
    let universe = 32;
    let seq_numbers = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4, 5, 3]);
    let direct = ArraySetEnumMtEph::from_seq(universe, &seq_numbers);
    let reduced = array_enum_from_seq(universe, &seq_numbers);
    assert_eq!(direct.to_seq(), reduced.to_seq());

    let seq_tree = ArraySeqStEphS::from_vec(vec!['a', 'b', 'a', 'c']);
    let direct_tree = AVLTreeSetStEph::from_seq(&seq_tree);
    let reduced_tree = avl_from_seq(&seq_tree);
    assert_eq!(direct_tree.to_seq(), reduced_tree.to_seq());

    let seq_tree_mt = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'b', 'd']);
    let direct_mt = AVLTreeSetMtEph::from_seq(&seq_tree_mt);
    let reduced_mt = avl_mt_from_seq(&seq_tree_mt);
    assert_eq!(direct_mt.size(), reduced_mt.size());
    for value in ['a', 'b', 'd'] {
        assert!(direct_mt.find(&value));
        assert!(reduced_mt.find(&value));
    }
}
