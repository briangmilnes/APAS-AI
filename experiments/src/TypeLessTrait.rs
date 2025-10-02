#![allow(dead_code)]
pub mod TypeLessTrait {

    trait MyInterface {
        fn do_something(&self);
        fn get_value(&self) -> u32;
    }

    fn do_something(_x: u32) { () }
    fn get_value(_x: u32) -> u32 { 1 }
}
