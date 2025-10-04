//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BinaryHeapPQ.

use apas_ai::BinaryHeapPQLit;
use apas_ai::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
use apas_ai::Types::Types::*;

#[test]
fn test_binaryheappqlit_macro_functionality() {
    // Test empty heap creation
    let empty: BinaryHeapPQ<i32> = BinaryHeapPQLit![];
    assert_eq!(empty.size(), 0);
    
    // Test heap creation with elements
    let with_data: BinaryHeapPQ<i32> = BinaryHeapPQLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
}

