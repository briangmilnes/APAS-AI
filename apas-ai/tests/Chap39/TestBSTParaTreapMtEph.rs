//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTParaTreapMtEph.

use apas_ai::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
use apas_ai::ParamTreapLit;
use apas_ai::Types::Types::*;

#[test]
fn test_paramtreaplit_macro_functionality() {
    // Test empty tree creation
    let empty: ParamTreap<i32> = ParamTreapLit!();
    assert_eq!(empty.size(), 0);
    
    // Test tree creation with elements
    let with_data: ParamTreap<i32> = ParamTreapLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert_eq!(with_data.find(&5), Some(5));
    assert_eq!(with_data.find(&3), Some(3));
    assert_eq!(with_data.find(&7), Some(7));
    assert_eq!(with_data.find(&10), None);
}
