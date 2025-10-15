//! Copyright © 2025 Russ Eddington. All rights reserved.
//!
//! Test suite for PathWeightUtilsStPer.

use ordered_float::OrderedFloat;

use apas_ai::Types::Types::OrderedF64;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;
use apas_ai::Chap56::PathWeightUtilsStPer::PathWeightUtilsStPer::*;

#[test]
fn test_path_weight_int_simple() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![0, 1, i64::MAX]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, 0, 2]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(path_weight_int(&path, &weights), Some(3));
}

#[test]
fn test_path_weight_int_negative() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![0, 5, i64::MAX]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, 0, -3]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(path_weight_int(&path, &weights), Some(2));
}

#[test]
fn test_path_weight_float_simple() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![OrderedFloat(0.0), OrderedFloat(1.5), OrderedFloat(f64::INFINITY)]),
        ArraySeqStPerS::from_vec(vec![OrderedFloat(f64::INFINITY), OrderedFloat(0.0), OrderedFloat(2.5)]),
        ArraySeqStPerS::from_vec(vec![
            OrderedFloat(f64::INFINITY),
            OrderedFloat(f64::INFINITY),
            OrderedFloat(0.0),
        ]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(path_weight_float(&path, &weights), Some(OrderedFloat(4.0)));
}

#[test]
fn test_validate_subpath_int() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![0, 1, i64::MAX]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, 0, 2]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let distances = ArraySeqStPerS::from_vec(vec![0, 1, 3]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert!(validate_subpath_property_int(&path, &distances, &weights));
}

#[test]
fn test_validate_subpath_float() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![OrderedFloat(0.0), OrderedFloat(1.5), OrderedFloat(f64::INFINITY)]),
        ArraySeqStPerS::from_vec(vec![OrderedFloat(f64::INFINITY), OrderedFloat(0.0), OrderedFloat(2.5)]),
        ArraySeqStPerS::from_vec(vec![
            OrderedFloat(f64::INFINITY),
            OrderedFloat(f64::INFINITY),
            OrderedFloat(0.0),
        ]),
    ]);
    let distances = ArraySeqStPerS::from_vec(vec![OrderedFloat(0.0), OrderedFloat(1.5), OrderedFloat(4.0)]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert!(validate_subpath_property_float(&path, &distances, &weights));
}
