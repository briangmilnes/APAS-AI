#![allow(dead_code)]

/// A cleaner pattern for functional module interfaces in Rust
pub mod FunctionalInterface {
    
    // Define the interface/trait that modules should implement
    pub trait Calculator {
        fn add(&self, a: u32, b: u32) -> u32;
        fn multiply(&self, a: u32, b: u32) -> u32;
        fn identity() -> u32;
    }
    
    // Strategy 1: Zero-sized type (ZST) implementation
    // This is the most idiomatic for "functional modules"
    pub struct BasicCalc;
    
    impl Calculator for BasicCalc {
        fn add(&self, a: u32, b: u32) -> u32 { a + b }
        fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
        fn identity() -> u32 { 0 }
    }
    
    // Strategy 2: Alternative implementation
    pub struct ModuloCalc;
    
    impl Calculator for ModuloCalc {
        fn add(&self, a: u32, b: u32) -> u32 { (a + b) % 100 }
        fn multiply(&self, a: u32, b: u32) -> u32 { (a * b) % 100 }
        fn identity() -> u32 { 1 }
    }
    
    // Generic function that works with any Calculator implementation
    pub fn compute_with<C: Calculator>(calc: &C, x: u32, y: u32) -> u32 {
        let sum = calc.add(x, y);
        let prod = calc.multiply(sum, 2);
        prod + C::identity()
    }
    
    // Convenience functions using default implementation
    pub fn compute(x: u32, y: u32) -> u32 {
        compute_with(&BasicCalc, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionalInterface::*;
    
    #[test]
    fn test_basic_calc() {
        let calc = BasicCalc;
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.multiply(2, 3), 6);
    }
    
    #[test]
    fn test_compute() {
        assert_eq!(compute(5, 10), 30); // (5+10)*2 + 0 = 30
    }
    
    #[test]
    fn test_with_different_impl() {
        let result = compute_with(&ModuloCalc, 50, 60);
        // (50+60=110 % 100 = 10) * 2 = 20 % 100 = 20, + 1 = 21
        assert_eq!(result, 21);
    }
}



