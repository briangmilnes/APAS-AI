#![allow(dead_code)]

/// Function types without traits
pub mod function_types {
    
    // ============================================================
    // Declare function types directly!
    // ============================================================
    
    /// Type alias for binary operations
    pub type BinaryOp = fn(u32, u32) -> u32;
    
    /// Type alias for unary operations  
    pub type UnaryOp = fn(u32) -> u32;
    
    /// Type alias for predicates
    pub type Predicate = fn(u32) -> bool;
    
    // Now implement functions matching these types
    
    pub fn add(x: u32, y: u32) -> u32 {
        x + y
    }
    
    pub fn multiply(x: u32, y: u32) -> u32 {
        x * y
    }
    
    pub fn square(x: u32) -> u32 {
        x * x
    }
    
    pub fn is_even(x: u32) -> bool {
        x % 2 == 0
    }
    
    // ============================================================
    // Use them as parameters
    // ============================================================
    
    pub fn apply_binary(op: BinaryOp, x: u32, y: u32) -> u32 {
        op(x, y)
    }
    
    pub fn apply_unary(op: UnaryOp, x: u32) -> u32 {
        op(x)
    }
    
    pub fn filter_nums(nums: &[u32], pred: Predicate) -> Vec<u32> {
        nums.iter().copied().filter(|&n| pred(n)).collect()
    }
}

/// More complex example - function signatures as types
pub mod signatures {
    
    /// Math module with explicit type signatures
    pub mod math {
        // Declare the types first
        pub type AddFn = fn(u32, u32) -> u32;
        pub type MulFn = fn(u32, u32) -> u32;
        pub type FactFn = fn(u32) -> u32;
        
        // Implementations
        pub fn add(x: u32, y: u32) -> u32 { x + y }
        pub fn multiply(x: u32, y: u32) -> u32 { x * y }
        pub fn factorial(n: u32) -> u32 {
            match n {
                0 | 1 => 1,
                n => n * factorial(n - 1),
            }
        }
        
        // Can verify a function matches the type
        pub const ADD_FN: AddFn = add;
        pub const MUL_FN: MulFn = multiply;
        pub const FACT_FN: FactFn = factorial;
    }
}

/// Using function types as a "module signature"
pub mod with_signature {
    
    // Define all function types upfront (like a signature)
    pub type ReadFn = fn(&str) -> Result<String, String>;
    pub type WriteFn = fn(&str, &str) -> Result<(), String>;
    pub type ExistsFn = fn(&str) -> bool;
    
    /// File operations module
    pub mod files {
        use super::*;
        
        pub fn read(path: &str) -> Result<String, String> {
            std::fs::read_to_string(path).map_err(|e| e.to_string())
        }
        
        pub fn write(path: &str, contents: &str) -> Result<(), String> {
            std::fs::write(path, contents).map_err(|e| e.to_string())
        }
        
        pub fn exists(path: &str) -> bool {
            std::path::Path::new(path).exists()
        }
        
        // Verify they match the types
        pub const READ: ReadFn = read;
        pub const WRITE: WriteFn = write;
        pub const EXISTS: ExistsFn = exists;
    }
}

#[cfg(test)]
mod tests {
    use super::function_types::*;
    use super::signatures::math;
    
    #[test]
    fn test_function_types() {
        assert_eq!(apply_binary(add, 5, 3), 8);
        assert_eq!(apply_binary(multiply, 4, 2), 8);
        assert_eq!(apply_unary(square, 5), 25);
    }
    
    #[test]
    fn test_predicate() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let evens = filter_nums(&nums, is_even);
        assert_eq!(evens, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_signatures() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!((math::ADD_FN)(2, 3), 5);
    }
}

