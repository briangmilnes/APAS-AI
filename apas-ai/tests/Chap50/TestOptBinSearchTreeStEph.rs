//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OptBinSearchTreeStEph.

use apas_ai::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::{KeyProb as OBSTStEphKeyProb, OBSTStEphS, OBSTStEphTrait};
use apas_ai::Chap50::Probability::Probability::*;
use apas_ai::prob;

#[test]
fn test_obst_st_eph_empty() {
    let mut obst: OBSTStEphS<char> = OBSTStEphS::new();
    assert_eq!(obst.optimal_cost(), Probability::zero());
    assert_eq!(obst.num_keys(), 0);
}

#[test]
fn test_obst_st_eph_single_key() {
    let keys = vec!['A'];
    let probs = vec![prob!(0.5)];
    let mut obst = OBSTStEphS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 1);
    assert_eq!(obst.optimal_cost(), prob!(0.5));
}

#[test]
fn test_obst_st_eph_mutation() {
    let keys1 = vec!['A', 'B'];
    let probs1 = vec![prob!(0.3), prob!(0.7)];
    let mut obst1 = OBSTStEphS::from_keys_probs(keys1, probs1);
    let cost1 = obst1.optimal_cost();

    let keys2 = vec!['A', 'B'];
    let probs2 = vec![prob!(0.5), prob!(0.5)];
    let mut obst2 = OBSTStEphS::from_keys_probs(keys2, probs2);
    let cost2 = obst2.optimal_cost();

    assert!(cost1 != cost2);
}
