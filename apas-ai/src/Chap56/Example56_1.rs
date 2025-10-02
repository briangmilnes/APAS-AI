//! Copyright © 2025 APAS-VERUS. All rights reserved.
//!
//! Example 56.1 - Path Weight Computation
//!
//! Demonstrates computing the weight of a path in a weighted graph.
//! Shows path weight calculation for simple paths with both positive and negative weights.

pub mod Example56_1 {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;
    use crate::Chap56::PathWeightUtilsStEph::PathWeightUtilsStEph::{path_weight_float, path_weight_int};
    use ordered_float::OrderedFloat;
    
    /// Example demonstrating path weight computation with integer weights.
    pub fn example_path_weight_int() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 5, 10, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 3, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0, 1]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, i64::MAX, 0]),
        ]);
    
        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2, 3]);
        match path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2→3 has weight: {}", w),
            | None => println!("Invalid path"),
        }
    }
    
    /// Example demonstrating path weight computation with floating-point weights.
    pub fn example_path_weight_float() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(0.0),
                OrderedFloat(2.5),
                OrderedFloat(5.0),
                OrderedFloat(f64::INFINITY),
            ]),
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(f64::INFINITY),
                OrderedFloat(0.0),
                OrderedFloat(1.5),
                OrderedFloat(f64::INFINITY),
            ]),
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(f64::INFINITY),
                OrderedFloat(f64::INFINITY),
                OrderedFloat(0.0),
                OrderedFloat(0.5),
            ]),
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(f64::INFINITY),
                OrderedFloat(f64::INFINITY),
                OrderedFloat(f64::INFINITY),
                OrderedFloat(0.0),
            ]),
        ]);
    
        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2, 3]);
        match path_weight_float(&path, &weights) {
            | Some(w) => println!("Path 0→1→2→3 has weight: {:.1}", w.0),
            | None => println!("Invalid path"),
        }
    }
    
    /// Example with negative edge weights.
    pub fn example_negative_weights() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 10, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, -5]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0]),
        ]);
    
        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
        match path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2 with negative weight has total: {}", w),
            | None => println!("Invalid path"),
        }
    }
}
