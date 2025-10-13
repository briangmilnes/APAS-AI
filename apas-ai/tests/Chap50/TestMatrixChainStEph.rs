//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainStEph.

use apas_ai::Chap50::MatrixChainStEph::MatrixChainStEph::*;
use apas_ai::MatrixChainStEphLit;
use apas_ai::Types::Types::*;

#[test]
fn test_matrixchainstephlit_macro_functionality() {
    // Test with sample matrix dimension pairs (rows, cols)
    let _chain = MatrixChainStEphLit![
        dims: [(10, 20), (20, 30), (30, 40)]
    ];
    // Macro creates a MatrixChain, not a simple vec
}

#[test]
fn test_new() {
    let chain = MatrixChainStEphS::new();
    assert_eq!(chain.num_matrices(), 0);
}

#[test]
fn test_from_dimensions() {
    let dims = vec![MatrixDim { rows: 10, cols: 20 }, MatrixDim { rows: 20, cols: 30 }];
    let chain = MatrixChainStEphS::from_dimensions(dims);
    assert_eq!(chain.num_matrices(), 2);
}

#[test]
fn test_from_dim_pairs() {
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let chain = MatrixChainStEphS::from_dim_pairs(pairs);
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_optimal_cost() {
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_dimensions() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let dims = chain.dimensions();
    assert_eq!(dims.len(), 2);
    assert_eq!(dims[0].rows, 10);
    assert_eq!(dims[0].cols, 20);
}

#[test]
fn test_dimensions_mut() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let dims = chain.dimensions_mut();
    dims.push(MatrixDim { rows: 30, cols: 40 });
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_set_dimension() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    chain.set_dimension(0, MatrixDim { rows: 5, cols: 15 });
    assert_eq!(chain.dimensions()[0].rows, 5);
    assert_eq!(chain.dimensions()[0].cols, 15);
}

#[test]
fn test_update_dimension() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    chain.update_dimension(1, 25, 35);
    assert_eq!(chain.dimensions()[1].rows, 25);
    assert_eq!(chain.dimensions()[1].cols, 35);
}

#[test]
fn test_num_matrices() {
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let chain = MatrixChainStEphS::from_dim_pairs(pairs);
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_clear_memo() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let _ = chain.optimal_cost();
    assert!(chain.memo_size() > 0);
    chain.clear_memo();
    assert_eq!(chain.memo_size(), 0);
}

#[test]
fn test_memo_size() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    assert_eq!(chain.memo_size(), 0);
    let _ = chain.optimal_cost();
    assert!(chain.memo_size() > 0);
}
