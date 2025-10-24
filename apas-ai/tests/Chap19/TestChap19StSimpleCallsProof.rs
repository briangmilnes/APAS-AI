//! Proof that simple calls work for St (single-threaded) files when ONLY Chap19 is imported
use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_st_simple_calls_work_without_chap18_redefinable() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    
    // Simple Type::function calls - NO UFCS needed!
    // These resolve to Chap19's ArraySeqStEphTrait
    let sum = ArraySeqStEphS::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 10);
    
    let doubled = ArraySeqStEphS::map(&seq, &|x| x * 2);
    assert_eq!(doubled.nth(0), &2);
    assert_eq!(doubled.nth(3), &8);
    
    let filtered = ArraySeqStEphS::filter(&seq, &|x| *x % 2 == 0);
    assert_eq!(filtered.length(), 2);
}

#[test]
fn test_st_type_prefix_required() {
    let seq = ArraySeqStEphSLit![1, 2, 3, 4];
    
    // reduce() is a TRAIT METHOD (associated function), not a free function
    // Must use Type::method syntax - cannot call as bare reduce(...)
    let sum = ArraySeqStEphS::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 10);
}

