//! Tests for M1 module demonstrating trait method visibility

use experiments::M1::{M1, M1Trait};

#[test]
fn test_struct_only_import_has_inherent() {
    // When importing only the struct, inherent methods are available
    use experiments::M1::M1;
    
    let m = M1 { value: 10 };
    assert_eq!(m.method2(), 30); // Inherent method works
}

#[test]
fn test_wildcard_import_has_both() {
    // When importing with *, both trait and inherent methods are available
    use experiments::M1::*;
    
    let m = M1 { value: 10 };
    assert_eq!(m.method1(), 20); // Trait method works
    assert_eq!(m.method2(), 30); // Inherent method works
}

#[test]
fn test_explicit_struct_and_trait_has_both() {
    // When importing struct and trait explicitly, both methods work
    use experiments::M1::{M1, M1Trait};
    
    let m = M1 { value: 10 };
    assert_eq!(m.method1(), 20); // Trait method works
    assert_eq!(m.method2(), 30); // Inherent method works
}

