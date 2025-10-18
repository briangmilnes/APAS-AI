//! M2JustStructAndTrait: Explicitly imports struct and trait to access both methods

use crate::M1::{M1, M1Trait};

pub fn use_both_methods(m: &M1) -> (i32, i32) {
    (m.method1(), m.method2())
}

