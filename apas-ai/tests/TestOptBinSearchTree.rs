//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Optimal Binary Search Tree implementations using Vec-based data structures.

use apas_ai::{
    Chap50::{
        OptBinSearchTreeStEph::OptBinSearchTreeStEph::{KeyProb as OBSTStEphKeyProb, OBSTStEphS, OBSTStEphTrait},
        OptBinSearchTreeStPer::OptBinSearchTreeStPer::{KeyProb as OBSTStPerKeyProb, OBSTStPerS, OBSTStPerTrait},
        Probability::Probability,
    },
    OBSTStPerLit, prob,
};

#[cfg(test)]
mod obst_tests {
    use super::*;

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
        // Cost should be sum of probabilities + optimal arrangement cost
        let cost = obst.optimal_cost();
        assert!(cost.0 >= 1.0); // At least sum of probabilities
        assert!(cost.0 <= 3.0); // Reasonable upper bound
    }

    #[test]
    fn test_obst_st_per_three_keys() {
        let keys = vec!['A', 'B', 'C'];
        let probs = vec![prob!(0.1), prob!(0.2), prob!(0.4)];
        let obst = OBSTStPerS::from_keys_probs(keys, probs);

        assert_eq!(obst.num_keys(), 3);
        let cost = obst.optimal_cost();
        // Should find optimal arrangement
        assert!(cost.0 > 0.0);
        assert!(cost.0 < 10.0); // Reasonable upper bound
    }

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
        let keys = vec!['A', 'B'];
        let probs = vec![prob!(0.3), prob!(0.7)];
        let mut obst = OBSTStEphS::from_keys_probs(keys, probs);

        let initial_cost = obst.optimal_cost();

        // Update probability and verify cost changes
        obst.update_prob(0, prob!(0.8));
        let updated_cost = obst.optimal_cost();

        assert_ne!(initial_cost, updated_cost);
        assert_eq!(obst.num_keys(), 2);
    }

    #[test]
    fn test_obst_key_prob_display() {
        let key_prob = OBSTStPerKeyProb {
            key: 'A',
            prob: prob!(0.125),
        };
        let display_str = format!("{}", key_prob);
        assert!(display_str.contains("A"));
        assert!(display_str.contains("0.125"));
    }

    #[test]
    fn test_obst_iteration() {
        let keys = vec!['A', 'B', 'C'];
        let probs = vec![prob!(0.1), prob!(0.2), prob!(0.3)];
        let obst = OBSTStPerS::from_keys_probs(keys, probs);

        let collected: Vec<_> = obst.into_iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0].key, 'A');
        assert_eq!(collected[1].key, 'B');
        assert_eq!(collected[2].key, 'C');
    }

    #[test]
    fn test_obst_macro() {
        let obst = OBSTStPerLit!(keys: ['A', 'B'], probs: [0.3, 0.7]);
        assert_eq!(obst.num_keys(), 2);
        let cost = obst.optimal_cost();
        assert!(cost.0 > 0.0);
    }

    #[test]
    fn test_obst_probability_arithmetic() {
        let p1 = prob!(0.3);
        let p2 = prob!(0.7);
        let sum = p1 + p2;
        assert!((sum.0 - 1.0).abs() < 1e-10);

        let diff = p2 - p1;
        assert!((diff.0 - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_obst_large_example() {
        let keys = vec!['A', 'B', 'C', 'D', 'E'];
        let probs = vec![prob!(0.1), prob!(0.2), prob!(0.3), prob!(0.2), prob!(0.2)];
        let obst = OBSTStPerS::from_keys_probs(keys, probs);

        assert_eq!(obst.num_keys(), 5);
        let cost = obst.optimal_cost();

        // Verify cost is reasonable (sum of probabilities + optimal arrangement cost)
        assert!(cost.0 >= 1.0); // At least sum of probabilities
        assert!(cost.0 <= 15.0); // Reasonable upper bound
    }
}
