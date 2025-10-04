//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetPlainMtEph.

use apas_ai::BSTSetPlainMtEphLit;
use apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsetplainmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSetPlainMtEph<i32> = BSTSetPlainMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTSetPlainMtEph<i32> = BSTSetPlainMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.find(&5));
    assert!(with_data.find(&3));
    assert!(with_data.find(&7));
    assert!(!with_data.find(&10));
}

