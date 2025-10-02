#![allow(dead_code)]
pub mod TypeLessTraitWithImplU32 {

    trait MyInterface {
        fn do_something(&self);
        fn get_value(&self) -> u32;
    }

  impl MyInterface for u32 {
    fn do_something(&self) { () }
    fn get_value(&self) -> u32 { 1 }
  }
}

