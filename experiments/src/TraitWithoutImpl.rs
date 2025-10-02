#![allow(dead_code)]

/// What happens without the impl?
pub mod math_no_impl {
    
    /// Trait defines the interface
    pub trait MathOps {
        fn add(x: u32, y: u32) -> u32;
        fn multiply(x: u32, y: u32) -> u32;
    }
    
    // NO IMPL! Just the trait and free functions
    
    // Free functions at top level
    pub fn add(x: u32, y: u32) -> u32 {
        x + y
    }
    
    pub fn multiply(x: u32, y: u32) -> u32 {
        x * y
    }
    
    // ⚠️ The trait does NOT type check these functions!
    // I can even make them wrong:
    
    pub fn wrong_add(x: u32, y: u32) -> String {  // Different return type!
        format!("{} + {}", x, y)
    }
    
    // The trait doesn't enforce anything without an impl
}

/// Making a caller module
pub mod caller {
    use super::math_no_impl::*;
    
    pub fn use_math() -> u32 {
        let sum = add(5, 3);
        let product = multiply(4, 2);
        sum + product
    }
    
    // Can we use the trait? Let's try:
    pub fn use_trait_generic<T: MathOps>() -> u32 {
        // ❌ This won't work - there's no type that implements MathOps!
        // T::add(5, 3)  // Error: no implementations found
        0  // Have to return something
    }
}

/// Now let's compare with the impl version
pub mod math_with_impl {
    
    pub trait MathOps {
        fn add(x: u32, y: u32) -> u32;
        fn multiply(x: u32, y: u32) -> u32;
    }
    
    pub struct Math;
    
    impl MathOps for Math {
        fn add(x: u32, y: u32) -> u32 {
            x + y
        }
        
        fn multiply(x: u32, y: u32) -> u32 {
            x * y
        }
    }
    
    // Free functions delegate to the impl
    pub fn add(x: u32, y: u32) -> u32 {
        Math::add(x, y)
    }
    
    pub fn multiply(x: u32, y: u32) -> u32 {
        Math::multiply(x, y)
    }
}

/// Caller using the impl version
pub mod caller_with_impl {
    use super::math_with_impl::*;
    
    pub fn use_math() -> u32 {
        let sum = add(5, 3);
        let product = multiply(4, 2);
        sum + product
    }
    
    // Now we CAN use the trait!
    pub fn use_trait_generic<T: MathOps>() -> u32 {
        T::add(5, 3) + T::multiply(4, 2)
    }
    
    // And call it with the impl
    pub fn call_generic() -> u32 {
        use_trait_generic::<Math>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_no_impl() {
        // Free functions work fine
        assert_eq!(math_no_impl::add(2, 3), 5);
        assert_eq!(caller::use_math(), 16);
    }
    
    #[test]
    fn test_with_impl() {
        assert_eq!(math_with_impl::add(2, 3), 5);
        assert_eq!(caller_with_impl::use_math(), 16);
        assert_eq!(caller_with_impl::call_generic(), 16);
    }
}



