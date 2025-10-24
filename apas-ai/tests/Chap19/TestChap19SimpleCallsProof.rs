//! Proof that simple calls work when ONLY Chap19 is imported
use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_simple_calls_work_without_chap18_redefinable() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    
    // Simple Type::function calls - NO UFCS needed!
    // These resolve to Chap19's ArraySeqMtEphTrait (parallel versions)
    let sum = ArraySeqMtEphS::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);
    
    let nested = ArraySeqMtEphSLit![ArraySeqMtEphSLit![1, 2], ArraySeqMtEphSLit![3, 4]];
    let flat = ArraySeqMtEphS::flatten(&nested);
    assert_eq!(flat.length(), 4);
    
    let updates = ArraySeqMtEphSLit![Pair(0, 10), Pair(2, 30)];
    let injected = ArraySeqMtEphS::inject(&seq, &updates);
    assert_eq!(injected.nth_cloned(0), 10);
    assert_eq!(injected.nth_cloned(2), 30);
}

#[test]
fn test_type_prefix_required() {
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4];
    
    // reduce() is a TRAIT METHOD (associated function), not a free function
    // Must use Type::method syntax - cannot call as bare reduce(...)
    let sum = ArraySeqMtEphS::reduce(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 10);
}

