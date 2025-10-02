#![allow(dead_code)]

//! Pattern: Traits as Documentation for Free Function Modules
//!
//! The trait serves as a formal interface specification.
//! No impl, no dummy structs - just documentation that the compiler can parse.

/// Math operations module
pub mod math {
    /// Interface specification for this module
    /// 
    /// This trait documents what operations are available.
    /// It's never implemented - it's just a formal comment.
    #[allow(dead_code)]
    pub trait MathInterface {
        /// Add two numbers
        fn add(x: u32, y: u32) -> u32;
        
        /// Multiply two numbers  
        fn multiply(x: u32, y: u32) -> u32;
        
        /// Compute factorial
        fn factorial(n: u32) -> u32;
    }
    
    // The actual implementation - free functions
    
    pub fn add(x: u32, y: u32) -> u32 {
        x + y
    }
    
    pub fn multiply(x: u32, y: u32) -> u32 {
        x * y
    }
    
    pub fn factorial(n: u32) -> u32 {
        match n {
            0 | 1 => 1,
            n => n * factorial(n - 1),
        }
    }
}

/// String manipulation module
pub mod strings {
    /// Interface specification for string operations
    #[allow(dead_code)]
    pub trait StringInterface {
        /// Reverse a string
        fn reverse(s: &str) -> String;
        
        /// Convert to uppercase
        fn uppercase(s: &str) -> String;
        
        /// Count words in a string
        fn word_count(s: &str) -> usize;
    }
    
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    
    pub fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }
    
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }
}

/// File operations module
pub mod files {
    /// Interface for file operations
    /// 
    /// This documents the contract without enforcement.
    /// In ML, this would be a proper module signature.
    #[allow(dead_code)]
    pub trait FileInterface {
        /// Read file contents
        fn read(path: &str) -> Result<String, String>;
        
        /// Write contents to file
        fn write(path: &str, contents: &str) -> Result<(), String>;
        
        /// Check if file exists
        fn exists(path: &str) -> bool;
    }
    
    pub fn read(path: &str) -> Result<String, String> {
        std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
    }
    
    pub fn write(path: &str, contents: &str) -> Result<(), String> {
        std::fs::write(path, contents)
            .map_err(|e| e.to_string())
    }
    
    pub fn exists(path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
}

/// Usage examples
pub mod usage {
    use super::math;
    use super::strings;
    
    pub fn demo() {
        // Math operations
        let sum = math::add(5, 3);
        let product = math::multiply(4, 2);
        let fact = math::factorial(5);
        
        println!("sum: {}, product: {}, factorial: {}", sum, product, fact);
        
        // String operations
        let reversed = strings::reverse("hello");
        let upper = strings::uppercase("world");
        let count = strings::word_count("the quick brown fox");
        
        println!("reversed: {}, upper: {}, words: {}", reversed, upper, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_math() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!(math::multiply(4, 5), 20);
        assert_eq!(math::factorial(5), 120);
    }
    
    #[test]
    fn test_strings() {
        assert_eq!(strings::reverse("abc"), "cba");
        assert_eq!(strings::uppercase("hello"), "HELLO");
        assert_eq!(strings::word_count("one two three"), 3);
    }
}



