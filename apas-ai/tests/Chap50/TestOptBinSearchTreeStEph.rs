//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OptBinSearchTreeStEph.

use apas_ai::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::*;
use apas_ai::Chap50::Probability::Probability::Probability;
use apas_ai::{prob, OBSTStEphLit};

#[test]
fn test_new_empty_obst() {
    let obst: OBSTStEphS<i32> = OBSTStEphTrait::new();
    assert_eq!(obst.num_keys(), 0);
    assert_eq!(obst.memo_size(), 0);
}

#[test]
fn test_from_keys_probs() {
    let keys = vec![1, 2, 3];
    let probs = vec![prob!(0.2), prob!(0.3), prob!(0.5)];
    let obst = OBSTStEphS::from_keys_probs(keys.clone(), probs.clone());

    assert_eq!(obst.num_keys(), 3);
    let kp = obst.keys();
    assert_eq!(kp[0].key, 1);
    assert_eq!(kp[0].prob, prob!(0.2));
    assert_eq!(kp[1].key, 2);
    assert_eq!(kp[1].prob, prob!(0.3));
    assert_eq!(kp[2].key, 3);
    assert_eq!(kp[2].prob, prob!(0.5));
}

#[test]
fn test_from_key_probs() {
    let key_probs = vec![
        KeyProb {
            key: 10,
            prob: prob!(0.1),
        },
        KeyProb {
            key: 20,
            prob: prob!(0.4),
        },
        KeyProb {
            key: 30,
            prob: prob!(0.5),
        },
    ];
    let obst = OBSTStEphS::from_key_probs(key_probs.clone());

    assert_eq!(obst.num_keys(), 3);
    assert_eq!(obst.keys()[0].key, 10);
    assert_eq!(obst.keys()[1].key, 20);
    assert_eq!(obst.keys()[2].key, 30);
}

#[test]
fn test_optimal_cost_single_key() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![5], vec![prob!(1.0)]);
    let cost = obst.optimal_cost();

    // Single key: cost is just the probability
    assert_eq!(cost, prob!(1.0));
}

#[test]
fn test_optimal_cost_two_keys() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let cost = obst.optimal_cost();

    // Cost should be positive and at least sum of probabilities
    assert!(cost.value() >= 1.0);
    assert!(cost.value() <= 2.0); // Upper bound check
}

#[test]
fn test_optimal_cost_three_keys() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2, 3], vec![prob!(0.2), prob!(0.5), prob!(0.3)]);
    let cost = obst.optimal_cost();

    // Cost should be positive
    assert!(cost.value() > 0.0);
    // Should be at least the sum of probabilities
    assert!(cost.value() >= 1.0);
}

#[test]
fn test_optimal_cost_uniform_probs() {
    let mut obst = OBSTStEphS::from_keys_probs(
        vec![1, 2, 3, 4],
        vec![prob!(0.25), prob!(0.25), prob!(0.25), prob!(0.25)],
    );
    let cost = obst.optimal_cost();

    // Uniform probabilities
    assert!(cost.value() > 0.0);
}

#[test]
fn test_keys_method() {
    let key_probs = vec![
        KeyProb {
            key: 5,
            prob: prob!(0.3),
        },
        KeyProb {
            key: 10,
            prob: prob!(0.7),
        },
    ];
    let obst = OBSTStEphS::from_key_probs(key_probs);

    let keys = obst.keys();
    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0].key, 5);
    assert_eq!(keys[1].key, 10);
}

#[test]
fn test_keys_mut_method() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    {
        let keys_mut = obst.keys_mut();
        keys_mut[0].prob = prob!(0.3);
        keys_mut[1].prob = prob!(0.7);
    }

    assert_eq!(obst.keys()[0].prob, prob!(0.3));
    assert_eq!(obst.keys()[1].prob, prob!(0.7));
}

#[test]
fn test_set_key_prob() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    obst.set_key_prob(
        0,
        KeyProb {
            key: 10,
            prob: prob!(0.2),
        },
    );

    assert_eq!(obst.keys()[0].key, 10);
    assert_eq!(obst.keys()[0].prob, prob!(0.2));
}

#[test]
fn test_update_prob() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    obst.update_prob(1, prob!(0.9));

    assert_eq!(obst.keys()[1].prob, prob!(0.9));
}

#[test]
fn test_num_keys() {
    let obst = OBSTStEphS::from_keys_probs(
        vec![1, 2, 3, 4, 5],
        vec![prob!(0.1), prob!(0.2), prob!(0.3), prob!(0.2), prob!(0.2)],
    );

    assert_eq!(obst.num_keys(), 5);
}

#[test]
fn test_clear_memo() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2, 3], vec![prob!(0.3), prob!(0.3), prob!(0.4)]);

    // Compute optimal cost (fills memo)
    let _cost1 = obst.optimal_cost();
    assert!(obst.memo_size() > 0);

    // Clear memo
    obst.clear_memo();
    assert_eq!(obst.memo_size(), 0);

    // Compute again (should refill memo)
    let _cost2 = obst.optimal_cost();
    assert!(obst.memo_size() > 0);
}

#[test]
fn test_memo_size() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    assert_eq!(obst.memo_size(), 0);

    obst.optimal_cost();

    // After computing, memo should have entries
    assert!(obst.memo_size() > 0);
}

#[test]
fn test_optimal_cost_memoization() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2, 3, 4], vec![prob!(0.2), prob!(0.3), prob!(0.3), prob!(0.2)]);

    let cost1 = obst.optimal_cost();
    let memo_size1 = obst.memo_size();

    // Second call should use memoized values
    let cost2 = obst.optimal_cost();
    let memo_size2 = obst.memo_size();

    assert_eq!(cost1, cost2);
    assert_eq!(memo_size1, memo_size2);
}

#[test]
fn test_empty_keys() {
    let mut obst: OBSTStEphS<i32> = OBSTStEphTrait::new();
    let cost = obst.optimal_cost();

    // Empty tree has zero cost
    assert_eq!(cost, prob!(0.0));
}

#[test]
fn test_string_keys() {
    let mut obst = OBSTStEphS::from_keys_probs(
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec![prob!(0.2), prob!(0.5), prob!(0.3)],
    );

    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_skewed_probabilities() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2, 3], vec![prob!(0.9), prob!(0.05), prob!(0.05)]);

    let cost = obst.optimal_cost();

    // First key highly probable, should have low depth
    assert!(cost.value() > 0.0);
}

#[test]
fn test_large_tree() {
    let n = 10;
    let keys: Vec<i32> = (0..n).collect();
    let probs: Vec<Probability> = (0..n).map(|_| prob!(1.0 / n as f64)).collect();

    let mut obst = OBSTStEphS::from_keys_probs(keys, probs);
    let cost = obst.optimal_cost();

    assert!(cost.value() > 0.0);
}

#[test]
fn test_key_prob_display() {
    let kp = KeyProb {
        key: 42,
        prob: prob!(0.5),
    };
    let display_str = format!("{kp}");

    assert!(display_str.contains("42"));
    assert!(display_str.contains("0.5"));
}

#[test]
fn test_key_prob_debug() {
    let kp = KeyProb {
        key: 42,
        prob: prob!(0.5),
    };
    let debug_str = format!("{kp:?}");

    assert!(!debug_str.is_empty());
}

#[test]
fn test_key_prob_clone() {
    let kp1 = KeyProb {
        key: 10,
        prob: prob!(0.3),
    };
    let kp2 = kp1.clone();

    assert_eq!(kp1.key, kp2.key);
    assert_eq!(kp1.prob, kp2.prob);
}

#[test]
fn test_key_prob_equality() {
    let kp1 = KeyProb {
        key: 5,
        prob: prob!(0.2),
    };
    let kp2 = KeyProb {
        key: 5,
        prob: prob!(0.2),
    };
    let kp3 = KeyProb {
        key: 5,
        prob: prob!(0.3),
    };

    assert_eq!(kp1, kp2);
    assert_ne!(kp1, kp3);
}

#[test]
fn test_obst_clone() {
    let obst1 = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let obst2 = obst1.clone();

    assert_eq!(obst1.num_keys(), obst2.num_keys());
    assert_eq!(obst1.keys(), obst2.keys());
}

#[test]
fn test_obst_equality() {
    let obst1 = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let obst2 = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.4), prob!(0.6)]);
    let obst3 = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    assert_eq!(obst1, obst2);
    assert_ne!(obst1, obst3);
}

#[test]
fn test_update_after_cost_computation() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2], vec![prob!(0.5), prob!(0.5)]);

    let cost1 = obst.optimal_cost();

    // Update probability
    obst.update_prob(0, prob!(0.8));
    obst.clear_memo();

    let cost2 = obst.optimal_cost();

    // Costs should differ after probability change
    assert_ne!(cost1, cost2);
}

#[test]
fn test_zero_probability() {
    let mut obst = OBSTStEphS::from_keys_probs(vec![1, 2, 3], vec![prob!(0.0), prob!(0.5), prob!(0.5)]);

    let cost = obst.optimal_cost();
    assert!(cost.value() >= 0.0);
}

#[test]
fn test_obststephlit_macro_empty() {
    let obst: OBSTStEphS<i32> = OBSTStEphLit!();
    assert_eq!(obst.num_keys(), 0);
}

#[test]
fn test_obststephlit_macro_single_key() {
    let mut obst = OBSTStEphLit!(keys: [1], probs: [1.0]);
    assert_eq!(obst.num_keys(), 1);
    let cost = obst.optimal_cost();
    assert_eq!(cost, prob!(1.0));
}

#[test]
fn test_obststephlit_macro_multiple_keys() {
    let mut obst = OBSTStEphLit!(keys: [1, 2, 3], probs: [0.2, 0.5, 0.3]);
    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_obststephlit_macro_trailing_comma() {
    let obst = OBSTStEphLit!(keys: [10, 20,], probs: [0.4, 0.6,]);
    assert_eq!(obst.num_keys(), 2);
    let keys = obst.keys();
    assert_eq!(keys[0].key, 10);
    assert_eq!(keys[1].key, 20);
}

#[test]
fn test_obststephlit_macro_string_keys() {
    let mut obst = OBSTStEphLit!(
        keys: ["a".to_string(), "b".to_string()],
        probs: [0.3, 0.7]
    );
    assert_eq!(obst.num_keys(), 2);
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}
