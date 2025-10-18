//! M2JustStruct: Imports only the struct to test inherent method availability

use crate::M1::M1;

pub fn use_inherent_method(m: &M1) -> i32 {
    m.method2()  // Inherent method - should work
}

// This function would NOT compile if uncommented:
// pub fn use_trait_method(m: &M1) -> i32 {
//     m.method1()  // ERROR: M1Trait not in scope
// }

