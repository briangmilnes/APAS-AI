//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainMtPer.

use apas_ai::Chap50::MatrixChainMtPer::*;
use apas_ai::MatrixChainMtPerLit;
use apas_ai::Types::Types::*;

#[test]
fn test_matrixchainmtperlit_macro_functionality() {
    // Test with sample dimensions
    let dims = MatrixChainMtPerLit![10, 20, 30, 40];
    assert_eq!(dims.len(), 4);
}

