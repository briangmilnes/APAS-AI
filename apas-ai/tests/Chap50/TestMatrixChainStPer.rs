//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainStPer.

use apas_ai::Chap50::MatrixChainStPer::MatrixChainStPer::*;
use apas_ai::MatrixChainStPerLit;
use apas_ai::Types::Types::*;

#[test]
fn test_matrixchainstperlit_macro_functionality() {
    let chain = MatrixChainStPerLit![dims: [(10, 20), (20, 30), (30, 40)]];
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_new() {
    let chain: MatrixChainStPerS = MatrixChainStPerTrait::new();
    assert_eq!(chain.num_matrices(), 0);
}

#[test]
fn test_from_dimensions() {
    let dims = vec![MatrixDim{rows:10,cols:20}, MatrixDim{rows:20,cols:30}];
    let chain = MatrixChainStPerS::from_dimensions(dims);
    assert_eq!(chain.num_matrices(), 2);
}

#[test]
fn test_from_dim_pairs() {
    let pairs = vec![Pair(10,20), Pair(20,30), Pair(30,40)];
    let chain = MatrixChainStPerS::from_dim_pairs(pairs);
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_optimal_cost_two_matrices() {
    let chain = MatrixChainStPerLit![dims: [(10,20), (20,30)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 10*20*30);
}

#[test]
fn test_optimal_cost_three_matrices() {
    let chain = MatrixChainStPerLit![dims: [(10,20), (20,30), (30,40)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_dimensions() {
    let chain = MatrixChainStPerLit![dims: [(5,10), (10,15)]];
    let dims = chain.dimensions();
    assert_eq!(dims.len(), 2);
}

#[test]
fn test_compute_multiplication() {
    let chain = MatrixChainStPerLit![dims: [(2,3), (3,4)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 2*3*4);
}

#[test]
fn test_empty_chain() {
    let chain: MatrixChainStPerS = MatrixChainStPerTrait::new();
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_single_matrix() {
    let chain = MatrixChainStPerLit![dims: [(5,10)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_large_chain() {
    let pairs = vec![Pair(10,20), Pair(20,5), Pair(5,30), Pair(30,10), Pair(10,25)];
    let chain = MatrixChainStPerS::from_dim_pairs(pairs);
    assert_eq!(chain.num_matrices(), 5);
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}
