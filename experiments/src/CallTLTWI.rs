pub mod CallTLWI {
   use crate::TypeLessTraitWithImpl::TypeLessTraitWithImpl::*;
   // Need to specify a concrete type when calling associated function
   fn dummy() -> u32 { <u32>::one1() }
}    
