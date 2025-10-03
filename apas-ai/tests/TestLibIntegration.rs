//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Integration tests for lib.rs module structure and exports.

use apas_ai::{ArraySeqStPerSLit};

#[test]
fn test_module_accessibility() {
    // Test that all major modules are accessible through lib.rs
    use apas_ai::Chap03::InsertionSortSt::InsertionSortSt::*;
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Types::Types::*;

    // Verify we can create instances through the module paths
    let _set: Set<i32> = Set::empty();
    let _seq: ArraySeqStPerS<i32> = ArraySeqStPerS::empty();
    let _pair = Pair(1, 2);
    let _bool = true;
}

#[test]
fn test_macro_accessibility() {
    // Test that macros are accessible through the crate
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::PairLit;
    use apas_ai::SetLit;

    let _set = SetLit![1, 2, 3];
    let _pair = PairLit!(1, "test");
    let _seq = ArraySeqStPerSLit![1, 2, 3];
}

#[test]
fn test_all_chapters_compile() {
    // Verify all chapter modules can be imported without compilation errors
    use apas_ai::Chap03;
    use apas_ai::Chap05;
    use apas_ai::Chap06;
    use apas_ai::Chap11;
    use apas_ai::Chap12;
    use apas_ai::Chap17;
    use apas_ai::Chap18;
    use apas_ai::Chap19;
    use apas_ai::Chap23;
    use apas_ai::Chap36;
    use apas_ai::Chap37;
    use apas_ai::Chap38;
    use apas_ai::Chap39;

    // If this compiles, the module structure is correct
    assert!(true);
}

#[test]
fn test_cross_chapter_compatibility() {
    // Test that types from different chapters work together
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;

    let set = SetLit![1, 2, 3];
    let seq = ArraySeqStPerSLit![1, 2, 3];

    // Verify cross-chapter type compatibility
    assert_eq!(set.size(), seq.length());
}
