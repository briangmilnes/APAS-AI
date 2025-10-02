#![allow(dead_code)]
pub mod TypeLessTraitWithImpl {

  pub trait TypeLessTraitWithImplTrait {
        fn do_something(&self);
        fn get_value(&self) -> u32;
        fn one1() -> u32;
    }

  impl<T> TypeLessTraitWithImplTrait for T {
    fn do_something(&self) { () }
    fn get_value(&self) -> u32 { 1 }
    fn one1() -> u32 { 1 }
  }
}

