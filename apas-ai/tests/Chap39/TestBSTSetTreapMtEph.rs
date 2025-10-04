//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetTreapMtEph.

use apas_ai::BSTSetTreapMtEphLit;
use apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_bstsettreapmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSetTreapMtEph<i32> = BSTSetTreapMtEphLit![];
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: BSTSetTreapMtEph<i32> = BSTSetTreapMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert_eq!(with_data.find(&5), Some(5));
    assert_eq!(with_data.find(&3), Some(3));
    assert_eq!(with_data.find(&7), Some(7));
    assert_eq!(with_data.find(&10), None);
}
