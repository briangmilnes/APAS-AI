//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OptBinSearchTreeStPer.

use apas_ai::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::{KeyProb as OBSTStPerKeyProb, OBSTStPerS, OBSTStPerTrait};
use apas_ai::Chap50::Probability::Probability::*;
use apas_ai::{OBSTStPerLit, prob};

#[test]
fn test_obst_st_per_empty() {
    let obst: OBSTStPerS<char> = OBSTStPerS::new();
    assert_eq!(obst.optimal_cost(), Probability::zero());
    assert_eq!(obst.num_keys(), 0);
}

#[test]
fn test_obst_st_per_single_key() {
    let keys = vec!['A'];
    let probs = vec![prob!(0.5)];
    let obst = OBSTStPerS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 1);
    assert_eq!(obst.optimal_cost(), prob!(0.5));
}

#[test]
fn test_obst_st_per_two_keys() {
    let keys = vec!['A', 'B'];
    let probs = vec![prob!(0.3), prob!(0.7)];
    let obst = OBSTStPerS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 2);
    let cost = obst.optimal_cost();
    assert!(cost.0 >= 1.0);
    assert!(cost.0 <= 3.0);
}

#[test]
fn test_obst_st_per_three_keys() {
    let keys = vec!['A', 'B', 'C'];
    let probs = vec![prob!(0.1), prob!(0.2), prob!(0.4)];
    let obst = OBSTStPerS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    assert!(cost.0 > 0.0);
    assert!(cost.0 < 10.0);
}

#[test]
fn test_obst_macro() {
    let obst = OBSTStPerLit![
        keys: ['A', 'B'],
        probs: [0.3, 0.7]
    ];
    assert_eq!(obst.num_keys(), 2);
}
