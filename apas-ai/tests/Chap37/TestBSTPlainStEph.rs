//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTPlainStEph.

use apas_ai::BSTPlainStEphLit;
use apas_ai::Chap37::BSTPlainStEph::BSTPlainStEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstplainstephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTPlainStEph<i32> = BSTPlainStEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTPlainStEph<i32> = BSTPlainStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

