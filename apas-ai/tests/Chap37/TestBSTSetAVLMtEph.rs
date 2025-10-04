//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetAVLMtEph.

use apas_ai::BSTSetAVLMtEphLit;
use apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsetavlmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSetAVLMtEph<i32> = BSTSetAVLMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTSetAVLMtEph<i32> = BSTSetAVLMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.find(&5));
    assert!(with_data.find(&3));
    assert!(with_data.find(&7));
    assert!(!with_data.find(&10));
}

