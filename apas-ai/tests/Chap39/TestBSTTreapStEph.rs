//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTTreapStEph.

use apas_ai::BSTTreapStEphLit;
use apas_ai::Chap39::BSTTreapStEph::BSTTreapStEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bsttreapstephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTTreapStEph<i32> = BSTTreapStEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTTreapStEph<i32> = BSTTreapStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}
