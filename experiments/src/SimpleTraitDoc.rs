#![allow(dead_code)]

/// Simple example: Trait as documentation for free functions
pub mod math {
    
    /// The trait defines the interface/contract for this module
    /// It serves as documentation showing what operations are available
    pub trait MathOps {
        fn add(x: u32, y: u32) -> u32;
        fn multiply(x: u32, y: u32) -> u32;
    }
    
    /// Implementation on a zero-sized type
    pub struct Math;
    
    impl MathOps for Math {
        fn add(x: u32, y: u32) -> u32 {
            x + y
        }
        
        fn multiply(x: u32, y: u32) -> u32 {
            x * y
        }
    }
    
    // ============================================================
    // FREE FUNCTIONS at module top level
    // These are what users actually call
    // ============================================================
    
    /// Add two numbers
    pub fn add(x: u32, y: u32) -> u32 {
        Math::add(x, y)
    }
    
    /// Multiply two numbers
    pub fn multiply(x: u32, y: u32) -> u32 {
        Math::multiply(x, y)
    }
}

// Usage example
pub fn example() {
    use math::*;
    
    // Users just call the free functions
    let sum = add(5, 3);        // 8
    let product = multiply(4, 2); // 8
    
    println!("sum: {}, product: {}", sum, product);
}

#[cfg(test)]
mod tests {
    use super::math::*;
    
    #[test]
    fn test_math_functions() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(add(10, 20), 30);
    }
}



