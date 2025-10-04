//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainMtEph.

use apas_ai::Chap50::MatrixChainMtEph::*;
use apas_ai::MatrixChainMtEphLit;
use apas_ai::Types::Types::*;

#[test]
fn test_matrixchainmtephlit_macro_functionality() {
    // Test with sample dimensions
    let dims = MatrixChainMtEphLit![10, 20, 30, 40];
    assert_eq!(dims.len(), 4);
}

