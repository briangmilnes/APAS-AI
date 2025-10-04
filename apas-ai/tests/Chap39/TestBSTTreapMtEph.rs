//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTTreapMtEph.

use apas_ai::BSTTreapMtEphLit;
use apas_ai::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bsttr eapmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTTreapMtEph<i32> = BSTTreapMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTTreapMtEph<i32> = BSTTreapMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

