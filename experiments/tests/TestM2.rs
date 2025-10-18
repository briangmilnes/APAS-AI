//! Tests for M2 modules showing both import styles work

use experiments::M1::M1;
use experiments::M2Asterix;
use experiments::M2JustStructAndTrait;
use experiments::M2JustStruct;

#[test]
fn test_m2_asterix_accesses_both_methods() {
    let m = M1 { value: 10 };
    let (result1, result2) = M2Asterix::use_both_methods(&m);
    assert_eq!(result1, 20); // method1 (trait)
    assert_eq!(result2, 30); // method2 (inherent)
}

#[test]
fn test_m2_just_struct_and_trait_accesses_both_methods() {
    let m = M1 { value: 10 };
    let (result1, result2) = M2JustStructAndTrait::use_both_methods(&m);
    assert_eq!(result1, 20); // method1 (trait)
    assert_eq!(result2, 30); // method2 (inherent)
}

#[test]
fn test_both_modules_give_same_results() {
    let m = M1 { value: 42 };
    
    let asterix_result = M2Asterix::use_both_methods(&m);
    let explicit_result = M2JustStructAndTrait::use_both_methods(&m);
    
    assert_eq!(asterix_result, explicit_result);
    assert_eq!(asterix_result, (84, 126));
}

#[test]
fn test_m2_just_struct_can_access_inherent_method() {
    let m = M1 { value: 10 };
    let result = M2JustStruct::use_inherent_method(&m);
    assert_eq!(result, 30); // inherent method works even without trait import!
}

