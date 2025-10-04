//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OBSTMtEph (Optimal Binary Search Tree).

use apas_ai::Chap50::OptBinSearchTreeMtEph::*;
use apas_ai::OBSTMtEphLit;
use apas_ai::Types::Types::*;

#[test]
fn test_obstmtephlit_macro_functionality() {
    // Test with sample keys and frequencies
    let data = OBSTMtEphLit![
        (1, 0.1),
        (2, 0.2),
        (3, 0.3)
    ];
    assert_eq!(data.len(), 3);
}

