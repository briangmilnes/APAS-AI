//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetRBMtEph.

use apas_ai::BSTSetRBMtEphLit;
use apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsetrbmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSetRBMtEph<i32> = BSTSetRBMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTSetRBMtEph<i32> = BSTSetRBMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.find(&5));
    assert!(with_data.find(&3));
    assert!(with_data.find(&7));
    assert!(!with_data.find(&10));
}

