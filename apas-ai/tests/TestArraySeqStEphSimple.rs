//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test demonstrating: NO INHERENT METHODS NEEDED - trait methods work directly!

use apas_ai::Chap18::ArraySeqStEphSimple::*;
use apas_ai::Types::Types::*;

#[test]
fn test_direct_method_calls_work() {
    // Trait is in scope, so methods work like normal!
    let seq: ArraySeqStEphSimpleS<i32> = ArraySeqStEphSimpleS::empty();
    assert_eq!(seq.length(), 0);  // NO inherent method! Works via trait!
    
    let seq = ArraySeqStEphSimpleS::singleton(42);
    assert_eq!(seq.length(), 1);  // Works!
    assert_eq!(seq.nth(0), &42);  // Works!
}

#[test]
fn test_mutable_operations() {
    let mut seq = ArraySeqStEphSimpleS::new(3, 10);
    assert_eq!(seq.length(), 3);
    
    let _ = seq.set(1, 99);
    assert_eq!(seq.nth(1), &99);
    
    let _ = seq.update(Pair(2, 88));
    assert_eq!(seq.nth(2), &88);
}

#[test]
fn test_from_vec_and_iter() {
    let seq = ArraySeqStEphSimpleS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
    
    let sum: i32 = seq.iter().sum();
    assert_eq!(sum, 15);
}

#[test]
fn test_generic_function() {
    // This is the REAL benefit of traits - generic functions!
    fn double_first<T, S>(seq: &mut S) 
    where 
        T: StT + std::ops::Add<Output = T> + Copy,
        S: ArraySeqStEphSimpleTrait<T>
    {
        if seq.length() > 0 {
            let first = *seq.nth(0);
            let _ = seq.set(0, first + first);
        }
    }
    
    let mut seq = ArraySeqStEphSimpleS::from_vec(vec![5, 10, 15]);
    double_first(&mut seq);
    assert_eq!(seq.nth(0), &10);
}

#[test]
fn test_explicit_trait_call() {
    // You CAN call via explicit trait syntax if needed
    let seq = <ArraySeqStEphSimpleS<i32> as ArraySeqStEphSimpleTrait<i32>>::empty();
    assert_eq!(<ArraySeqStEphSimpleS<i32> as ArraySeqStEphSimpleTrait<i32>>::length(&seq), 0);
    
    // But normal method syntax works fine too!
    assert_eq!(seq.length(), 0);
}

