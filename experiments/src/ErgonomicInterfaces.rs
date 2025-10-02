#![allow(dead_code)]

/// Different ways to make trait-based interfaces ergonomic
pub mod ErgonomicInterfaces {
    
    // ============================================================
    // Pattern 1: Direct struct usage (verbose)
    // ============================================================
    
    pub mod verbose {
        pub trait Math {
            fn add(&self, a: u32, b: u32) -> u32;
            fn multiply(&self, a: u32, b: u32) -> u32;
        }
        
        pub struct Calculator;
        
        impl Math for Calculator {
            fn add(&self, a: u32, b: u32) -> u32 { a + b }
            fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
        }
        
        // Usage: Must create struct every time
        pub fn example() -> u32 {
            Calculator.add(2, 3)  // Have to mention Calculator each time
        }
    }
    
    // ============================================================
    // Pattern 2: Module-level convenience functions (RECOMMENDED)
    // ============================================================
    
    pub mod convenient {
        pub trait Math {
            fn add(&self, a: u32, b: u32) -> u32;
            fn multiply(&self, a: u32, b: u32) -> u32;
        }
        
        pub struct Calculator;
        
        impl Math for Calculator {
            fn add(&self, a: u32, b: u32) -> u32 { a + b }
            fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
        }
        
        // Provide free functions that hide the struct!
        pub fn add(a: u32, b: u32) -> u32 {
            Calculator.add(a, b)
        }
        
        pub fn multiply(a: u32, b: u32) -> u32 {
            Calculator.multiply(a, b)
        }
        
        // Usage: Clean and simple!
        pub fn example() -> u32 {
            add(2, 3) + multiply(4, 5)  // No struct needed!
        }
    }
    
    // ============================================================
    // Pattern 3: Static instance (for stateless implementations)
    // ============================================================
    
    pub mod with_static {
        pub trait Math {
            fn add(&self, a: u32, b: u32) -> u32;
            fn multiply(&self, a: u32, b: u32) -> u32;
        }
        
        pub struct Calculator;
        
        impl Math for Calculator {
            fn add(&self, a: u32, b: u32) -> u32 { a + b }
            fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
        }
        
        // Provide a static instance
        pub static CALC: Calculator = Calculator;
        
        // Usage: Use the static instance
        pub fn example() -> u32 {
            CALC.add(2, 3) + CALC.multiply(4, 5)
        }
    }
    
    // ============================================================
    // Pattern 4: Just free functions (no trait needed)
    // ============================================================
    
    pub mod simple {
        // If you don't need swappable implementations,
        // just use regular functions!
        
        pub fn add(a: u32, b: u32) -> u32 {
            a + b
        }
        
        pub fn multiply(a: u32, b: u32) -> u32 {
            a * b
        }
        
        // Usage: Simple as it gets
        pub fn example() -> u32 {
            add(2, 3) + multiply(4, 5)
        }
        
        // Note: Can't swap implementations, but often you don't need to!
    }
    
    // ============================================================
    // Pattern 5: Impl block on module (closest to "namespaced functions")
    // ============================================================
    
    pub mod namespaced {
        pub struct Math;
        
        impl Math {
            // Associated functions (no trait needed)
            pub fn add(a: u32, b: u32) -> u32 {
                a + b
            }
            
            pub fn multiply(a: u32, b: u32) -> u32 {
                a * b
            }
        }
        
        // Usage: Like calling static methods
        pub fn example() -> u32 {
            Math::add(2, 3) + Math::multiply(4, 5)
        }
    }
    
    // ============================================================
    // Real-World Example: Best of Both Worlds
    // ============================================================
    
    pub mod best_practice {
        // Define trait for flexibility
        pub trait Logger {
            fn info(&self, msg: &str);
            fn error(&self, msg: &str);
        }
        
        // Default implementation
        pub struct ConsoleLogger;
        
        impl Logger for ConsoleLogger {
            fn info(&self, msg: &str) {
                println!("[INFO] {}", msg);
            }
            
            fn error(&self, msg: &str) {
                eprintln!("[ERROR] {}", msg);
            }
        }
        
        // Convenience functions for default implementation
        pub fn info(msg: &str) {
            ConsoleLogger.info(msg)
        }
        
        pub fn error(msg: &str) {
            ConsoleLogger.error(msg)
        }
        
        // Generic function for custom implementations
        pub fn log_with<L: Logger>(logger: &L, messages: &[&str]) {
            for msg in messages {
                logger.info(msg);
            }
        }
        
        // Usage examples:
        pub fn usage_examples() {
            // Easy default usage - no struct visible!
            info("Starting application");
            error("Something went wrong");
            
            // But can still swap implementations when needed
            let custom_logger = ConsoleLogger;
            log_with(&custom_logger, &["msg1", "msg2"]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ErgonomicInterfaces::*;
    
    #[test]
    fn test_convenient() {
        assert_eq!(convenient::add(2, 3), 5);
        assert_eq!(convenient::multiply(4, 5), 20);
    }
    
    #[test]
    fn test_with_static() {
        use with_static::Math; // Need trait in scope!
        assert_eq!(with_static::CALC.add(2, 3), 5);
    }
    
    #[test]
    fn test_simple() {
        assert_eq!(simple::add(2, 3), 5);
    }
    
    #[test]
    fn test_namespaced() {
        assert_eq!(namespaced::Math::add(2, 3), 5);
    }
    
    #[test]
    fn test_best_practice() {
        best_practice::info("test message");
        // No assertion needed, just checking it compiles and runs
    }
}

