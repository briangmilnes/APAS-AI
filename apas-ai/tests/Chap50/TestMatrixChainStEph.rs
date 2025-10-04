//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainStEph.

use apas_ai::Chap50::MatrixChainStEph::*;
use apas_ai::MatrixChainStEphLit;
use apas_ai::Types::Types::*;

#[test]
fn test_matrixchainstephlit_macro_functionality() {
    // Test with sample dimensions
    let dims = MatrixChainStEphLit![10, 20, 30, 40];
    assert_eq!(dims.len(), 4);
}

