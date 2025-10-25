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

#[test]
fn test_st_delegated_and_redefined_methods() {
    // Test methods that are re-declared in Chap19's trait
    // Some are delegated to Chap18, some are redefined by Chap19

    // empty() - defined in Chap19
    let empty = ArraySeqStEphS::<i32>::empty();
    assert_eq!(empty.length(), 0);

    // singleton() - defined in Chap19
    let single = ArraySeqStEphS::singleton(42);
    assert_eq!(*single.nth(0), 42);

    // tabulate() - defined in Chap19
    let tab = ArraySeqStEphS::tabulate(&|i| (i * 2) as i32, 5);
    assert_eq!(*tab.nth(4), 8);

    // append() - defined in Chap19
    let a = ArraySeqStEphSLit![1, 2];
    let b = ArraySeqStEphSLit![3, 4];
    let appended = ArraySeqStEphS::append(&a, &b);
    assert_eq!(appended.length(), 4);

    // isEmpty() - defined in Chap19
    assert!(ArraySeqStEphS::isEmpty(&empty));

    // isSingleton() - defined in Chap19
    assert!(ArraySeqStEphS::isSingleton(&single));

    // iterate() - defined in Chap19
    let seq = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    let sum = ArraySeqStEphS::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 15);

    // inject() - defined in Chap19 (was moved to RedefinableTrait)
    let base = ArraySeqStEphSLit![10, 20, 30, 40];
    let updates = vec![(0, 100), (2, 300)];
    let injected = ArraySeqStEphS::inject(&base, &updates);
    assert_eq!(*injected.nth(0), 100);
    assert_eq!(*injected.nth(1), 20);
    assert_eq!(*injected.nth(2), 300);
}
