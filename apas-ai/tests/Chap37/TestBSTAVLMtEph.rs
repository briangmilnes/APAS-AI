//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTAVLMtEph.

use apas_ai::BSTAVLMtEphLit;
use apas_ai::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstavlmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTAVLMtEph<i32> = BSTAVLMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTAVLMtEph<i32> = BSTAVLMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

