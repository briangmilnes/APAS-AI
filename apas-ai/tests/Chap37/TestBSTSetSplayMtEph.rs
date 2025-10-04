//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetSplayMtEph.

use apas_ai::BSTSetSplayMtEphLit;
use apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsetsplaymtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSetSplayMtEph<i32> = BSTSetSplayMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTSetSplayMtEph<i32> = BSTSetSplayMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.find(&5));
    assert!(with_data.find(&3));
    assert!(with_data.find(&7));
    assert!(!with_data.find(&10));
}

