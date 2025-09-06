//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::FooArraySeqPer::{FooArrayPerS, FooArraySeqPerTrait, foo_new2};

#[test]
fn test_new_and_set() {
    // 1) Inherent method (no UFCS, no type ascription): works because Self is known from call site
    let a = FooArrayPerS::new(3, 7);
    // 2) Trait method returning Self using method-call syntax (no UFCS): receiver provides Self
    let _b2 = a.set2(1, 11).unwrap();
    // 3) Free function returning concrete type (no UFCS, no type ascription): works due to concrete return
    let _c = foo_new2(3, 7);
}

#[test]
fn test_singleton() {
    let _s = <FooArrayPerS<N> as FooArraySeqPerTrait<N>>::singleton(42);
}

#[test]
fn test_is_empty_and_is_singleton() {
    let _e: FooArrayPerS<N> = <FooArrayPerS<N> as FooArraySeqPerTrait<N>>::empty();
}


