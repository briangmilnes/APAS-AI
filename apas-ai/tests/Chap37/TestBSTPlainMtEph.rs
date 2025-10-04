//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTPlainMtEph.

use apas_ai::BSTPlainMtEphLit;
use apas_ai::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstplainmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTPlainMtEph<i32> = BSTPlainMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTPlainMtEph<i32> = BSTPlainMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

