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

#[test]
fn test_empty_chain() {
    let mut chain = MatrixChainStEphS::new();
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_single_matrix() {
    let pairs = vec![Pair(10, 20)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0); // No multiplication needed for single matrix
}

#[test]
fn test_two_matrices() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert_eq!(cost, 10 * 20 * 30);
}

#[test]
fn test_classic_example() {
    // Classic DP example: matrices with dimensions 10x20, 20x30, 30x40, 40x30
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40), Pair(40, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert!(cost > 0);
    assert!(cost < 100000); // Should be optimized
}

#[test]
fn test_large_chain() {
    let pairs = vec![
        Pair(10, 15),
        Pair(15, 20),
        Pair(20, 25),
        Pair(25, 30),
        Pair(30, 35),
        Pair(35, 40),
        Pair(40, 45),
        Pair(45, 50),
    ];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_memo_reuse() {
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);

    let cost1 = chain.optimal_cost();
    let memo_size1 = chain.memo_size();

    let cost2 = chain.optimal_cost();
    let memo_size2 = chain.memo_size();

    assert_eq!(cost1, cost2);
    assert_eq!(memo_size1, memo_size2);
}

#[test]
fn test_update_and_recompute() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);

    let cost1 = chain.optimal_cost();

    chain.update_dimension(0, 5, 10);
    chain.clear_memo();

    let cost2 = chain.optimal_cost();

    assert_ne!(cost1, cost2);
}

#[test]
fn test_display_formatting() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let chain = MatrixChainStEphS::from_dim_pairs(pairs);

    let display = format!("{}", chain);
    assert!(display.contains("MatrixChain"));
}

#[test]
fn test_debug_formatting() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let chain = MatrixChainStEphS::from_dim_pairs(pairs);

    let debug = format!("{:?}", chain);
    assert!(debug.contains("MatrixChainStEphS"));
}

#[test]
fn test_clone() {
    let pairs = vec![Pair(10, 20), Pair(20, 30)];
    let chain = MatrixChainStEphS::from_dim_pairs(pairs);

    let cloned = chain.clone();
    assert_eq!(cloned.num_matrices(), 2);
}

#[test]
fn test_equality() {
    let pairs1 = vec![Pair(10, 20), Pair(20, 30)];
    let chain1 = MatrixChainStEphS::from_dim_pairs(pairs1);

    let pairs2 = vec![Pair(10, 20), Pair(20, 30)];
    let chain2 = MatrixChainStEphS::from_dim_pairs(pairs2);

    assert_eq!(chain1, chain2);
}

#[test]
fn test_varying_dimensions() {
    // Test with very different dimension sizes
    let pairs = vec![Pair(1, 100), Pair(100, 1), Pair(1, 100)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_square_matrices() {
    let pairs = vec![Pair(10, 10), Pair(10, 10), Pair(10, 10)];
    let mut chain = MatrixChainStEphS::from_dim_pairs(pairs);
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}
