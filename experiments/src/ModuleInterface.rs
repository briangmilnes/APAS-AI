#![allow(dead_code)]

/// Another pattern: Module-level interface using trait objects
pub mod ModuleInterface {
    
    // The interface
    pub trait DataProcessor {
        fn process(&self, data: &str) -> String;
        fn validate(&self, data: &str) -> bool;
    }
    
    // Implementation 1: Uppercase processor
    pub struct UppercaseProcessor;
    
    impl DataProcessor for UppercaseProcessor {
        fn process(&self, data: &str) -> String {
            data.to_uppercase()
        }
        
        fn validate(&self, data: &str) -> bool {
            !data.is_empty()
        }
    }
    
    // Implementation 2: Reverse processor
    pub struct ReverseProcessor;
    
    impl DataProcessor for ReverseProcessor {
        fn process(&self, data: &str) -> String {
            data.chars().rev().collect()
        }
        
        fn validate(&self, data: &str) -> bool {
            data.len() > 3
        }
    }
    
    // A function that accepts any processor
    pub fn process_data(processor: &dyn DataProcessor, data: &str) -> Result<String, &'static str> {
        if processor.validate(data) {
            Ok(processor.process(data))
        } else {
            Err("Validation failed")
        }
    }
    
    // Module can have a default processor
    static DEFAULT_PROCESSOR: UppercaseProcessor = UppercaseProcessor;
    
    pub fn process_default(data: &str) -> Result<String, &'static str> {
        process_data(&DEFAULT_PROCESSOR, data)
    }
}

#[cfg(test)]
mod tests {
    use super::ModuleInterface::*;
    
    #[test]
    fn test_uppercase() {
        let proc = UppercaseProcessor;
        assert_eq!(process_data(&proc, "hello").unwrap(), "HELLO");
    }
    
    #[test]
    fn test_reverse() {
        let proc = ReverseProcessor;
        assert_eq!(process_data(&proc, "rust").unwrap(), "tsur");
    }
    
    #[test]
    fn test_default() {
        assert_eq!(process_default("world").unwrap(), "WORLD");
    }
}



