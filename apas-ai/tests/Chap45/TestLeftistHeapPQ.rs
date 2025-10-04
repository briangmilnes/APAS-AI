//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for LeftistHeapPQ.

use apas_ai::Chap45::LeftistHeapPQ::LeftistHeapPQ::*;
use apas_ai::LeftistHeapPQLit;
use apas_ai::Types::Types::*;

#[test]
fn test_leftistheappqlit_macro_functionality() {
    // Test empty heap creation
    let empty: LeftistHeapPQ<i32> = LeftistHeapPQLit![];
    assert_eq!(empty.size(), 0);
    
    // Test heap creation with elements
    let with_data: LeftistHeapPQ<i32> = LeftistHeapPQLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
}

