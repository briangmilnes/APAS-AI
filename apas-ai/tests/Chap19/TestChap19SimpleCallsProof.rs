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

#[test]
fn test_delegated_methods_work_without_ufcs() {
    // These methods are DELEGATED to Chap18 but should work with simple calls
    // when importing only Chap19::*

    // empty() - delegated to Chap18
    let empty = ArraySeqMtEphS::<i32>::empty();
    assert_eq!(empty.length(), 0);

    // singleton() - delegated to Chap18
    let single = ArraySeqMtEphS::singleton(42);
    assert_eq!(single.length(), 1);
    assert_eq!(single.nth_cloned(0), 42);

    // tabulate() - delegated to Chap18
    let tab = ArraySeqMtEphS::tabulate(&|i| (i * 2) as i32, 5);
    assert_eq!(tab.length(), 5);
    assert_eq!(tab.nth_cloned(0), 0);
    assert_eq!(tab.nth_cloned(4), 8);

    // map() - delegated to Chap18
    let seq = ArraySeqMtEphSLit![1, 2, 3];
    let mapped = ArraySeqMtEphS::map(&seq, |x| x * 10);
    assert_eq!(mapped.nth_cloned(0), 10);
    assert_eq!(mapped.nth_cloned(2), 30);

    // append() - delegated to Chap18
    let a = ArraySeqMtEphSLit![1, 2];
    let b = ArraySeqMtEphSLit![3, 4];
    let appended = ArraySeqMtEphS::append(&a, &b);
    assert_eq!(appended.length(), 4);

    // filter() - delegated to Chap18
    let seq = ArraySeqMtEphSLit![1, 2, 3, 4, 5];
    let evens = ArraySeqMtEphS::filter(&seq, &|x: &i32| *x % 2 == 0);
    assert_eq!(evens.length(), 2);

    // isEmpty() - delegated to Chap18
    assert!(ArraySeqMtEphS::isEmpty(&empty));
    assert!(!ArraySeqMtEphS::isEmpty(&seq));

    // isSingleton() - delegated to Chap18
    assert!(ArraySeqMtEphS::isSingleton(&single));
    assert!(!ArraySeqMtEphS::isSingleton(&seq));

    // iterate() - delegated to Chap18
    let sum = ArraySeqMtEphS::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 15);
}
