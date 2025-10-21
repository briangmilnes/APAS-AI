//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Matrix Chain Multiplication StEph implementation.

use apas_ai::Chap50::MatrixChainStEph::MatrixChainStEph::{
    MatrixChainStEphS, MatrixChainStEphTrait, MatrixDim as MatrixChainStEphMatrixDim,
};

#[test]
fn test_matrix_chain_st_eph_empty() {
    let mut chain = MatrixChainStEphS::new();
    assert_eq!(chain.num_matrices(), 0);
    assert_eq!(chain.optimal_cost(), 0);
}

#[test]
fn test_matrix_chain_st_eph_single_matrix() {
    let dimensions = vec![MatrixChainStEphMatrixDim { rows: 10, cols: 20 }];
    let mut chain = MatrixChainStEphS::from_dimensions(dimensions);
    assert_eq!(chain.num_matrices(), 1);
    assert_eq!(chain.optimal_cost(), 0);
}

#[test]
fn test_matrix_chain_st_eph_mutation() {
    let dimensions = vec![
        MatrixChainStEphMatrixDim { rows: 10, cols: 20 },
        MatrixChainStEphMatrixDim { rows: 20, cols: 30 },
    ];
    let mut chain = MatrixChainStEphS::from_dimensions(dimensions);

    // Test mutation
    chain.set_dimension(0, MatrixChainStEphMatrixDim { rows: 15, cols: 25 });
    assert_eq!(chain.dimensions()[0].rows, 15);
    assert_eq!(chain.dimensions()[0].cols, 25);

    // Test mutable access
    {
        let dims_mut = chain.dimensions_mut();
        dims_mut[1] = MatrixChainStEphMatrixDim { rows: 25, cols: 35 };
    }
    assert_eq!(chain.dimensions()[1].rows, 25);
    assert_eq!(chain.dimensions()[1].cols, 35);
}

#[test]
fn test_matrix_chain_st_eph_iteration() {
    let dimensions = vec![
        MatrixChainStEphMatrixDim { rows: 5, cols: 10 },
        MatrixChainStEphMatrixDim { rows: 10, cols: 15 },
    ];
    let chain = MatrixChainStEphS::from_dimensions(dimensions);

    let collected: Vec<MatrixChainStEphMatrixDim> = chain.into_iter().collect();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].rows, 5);
    assert_eq!(collected[0].cols, 10);
    assert_eq!(collected[1].rows, 10);
    assert_eq!(collected[1].cols, 15);
}
