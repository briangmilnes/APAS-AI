//! M2Asterix: Uses wildcard import to access both trait and inherent methods

use crate::M1::*;

pub fn use_both_methods(m: &M1) -> (i32, i32) {
    (m.method1(), m.method2())
}

