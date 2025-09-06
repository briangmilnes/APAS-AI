//! Demonstrate wildcard import from FooArraySeqPer and when names still need qualification.

use apas_ai::Types::N;
use apas_ai::FooArraySeqPer::*;

#[test]
fn test_wildcard_import_usage() {
    // Inherent constructor: available directly via type name (in scope via wildcard)
    let a = FooArrayPerS::new(3, 7);

    // Trait method returning Self using trait name (no UFCS, receiver provides Self)
    let b = FooArraySeqPerTrait::set2(&a, 1, 11).unwrap();
    assert_eq!(b.data[1], 11);

    // Free function from module: available directly
    let c = foo_new2(2, 5);
    assert_eq!(c.data.len(), 2);
    assert_eq!(c.data[0], 5);

    // Trait associated fn with no receiver still needs the trait name
    let d: FooArrayPerS<N> = FooArraySeqPerTrait::<N>::new2(4, 1);
    assert_eq!(d.data.len(), 4);
}
