//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OptBinSearchTreeStPer.

use apas_ai::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::*;
use apas_ai::Chap50::Probability::Probability::Probability;
use apas_ai::prob;

#[test]
fn test_new_empty_obst() {
    let obst: OBSTStPerS<i32> = OBSTStPerTrait::new();
    assert_eq!(obst.num_keys(), 0);
}

#[test]
fn test_from_keys_probs() {
    let keys = vec![1, 2, 3];
    let probs = vec![prob!(0.2), prob!(0.3), prob!(0.5)];
    let obst = OBSTStPerS::from_keys_probs(keys.clone(), probs.clone());

    assert_eq!(obst.num_keys(), 3);
    let kp = obst.keys();
    assert_eq!(kp[0].key, 1);
    assert_eq!(kp[0].prob, prob!(0.2));
}

#[test]
fn test_from_key_probs() {
    let key_probs = vec![
        KeyProb { key: 10, prob: prob!(0.1) },
        KeyProb { key: 20, prob: prob!(0.4) },
    ];
    let obst = OBSTStPerS::from_key_probs(key_probs.clone());

    assert_eq!(obst.num_keys(), 2);
    assert_eq!(obst.keys()[0].key, 10);
}

#[test]
fn test_optimal_cost_single_key() {
    let obst = OBSTStPerS::from_keys_probs(vec![5], vec![prob!(1.0)]);
    let cost = obst.optimal_cost();

    assert_eq!(cost, prob!(1.0));
}

#[test]
fn test_optimal_cost_two_keys() {
    let obst = OBSTStPerS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let cost = obst.optimal_cost();

    assert!(cost.value() >= 1.0);
    assert!(cost.value() <= 2.0);
}

#[test]
fn test_optimal_cost_three_keys() {
    let obst = OBSTStPerS::from_keys_probs(
        vec![1, 2, 3],
        vec![prob!(0.2), prob!(0.5), prob!(0.3)],
    );
    let cost = obst.optimal_cost();

    assert!(cost.value() > 0.0);
    assert!(cost.value() >= 1.0);
}

#[test]
fn test_keys_method() {
    let key_probs = vec![
        KeyProb { key: 5, prob: prob!(0.3) },
        KeyProb { key: 10, prob: prob!(0.7) },
    ];
    let obst = OBSTStPerS::from_key_probs(key_probs);

    let keys = obst.keys();
    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0].key, 5);
}

#[test]
fn test_num_keys() {
    let obst = OBSTStPerS::from_keys_probs(vec![1, 2, 3, 4, 5], vec![
        prob!(0.1), prob!(0.2), prob!(0.3), prob!(0.2), prob!(0.2)
    ]);

    assert_eq!(obst.num_keys(), 5);
}

#[test]
fn test_memo_size() {
    let obst = OBSTStPerS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    // Before computation
    let _cost = obst.optimal_cost();

    // Memo size method should work without panicking
    let _ = obst.memo_size();
}

#[test]
fn test_empty_keys() {
    let obst: OBSTStPerS<i32> = OBSTStPerTrait::new();
    let cost = obst.optimal_cost();

    assert_eq!(cost, prob!(0.0));
}

#[test]
fn test_string_keys() {
    let obst = OBSTStPerS::from_keys_probs(
        vec!["a".to_string(), "b".to_string()],
        vec![prob!(0.4), prob!(0.6)],
    );

    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_skewed_probabilities() {
    let obst = OBSTStPerS::from_keys_probs(
        vec![1, 2, 3],
        vec![prob!(0.9), prob!(0.05), prob!(0.05)],
    );

    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_clone() {
    let obst1 = OBSTStPerS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let obst2 = obst1.clone();

    assert_eq!(obst1.num_keys(), obst2.num_keys());
    assert_eq!(obst1.keys(), obst2.keys());
}

#[test]
fn test_equality() {
    let obst1 = OBSTStPerS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let obst2 = OBSTStPerS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);

    assert_eq!(obst1, obst2);
}

#[test]
fn test_persistent_nature() {
    let obst1 = OBSTStPerS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);
    let obst2 = obst1.clone();

    // Original should be unchanged
    assert_eq!(obst1.num_keys(), obst2.num_keys());
}
