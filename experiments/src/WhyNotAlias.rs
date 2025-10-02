#![allow(dead_code)]

/// Demonstrating why type aliases DON'T work for traits
pub mod WhyNotAlias {
    
    pub trait Logger {
        fn log(&self, msg: &str);
    }
    
    // ❌ THIS DOESN'T COMPILE:
    // pub type ConsoleLogger = ();
    // impl Logger for ConsoleLogger {
    //     fn log(&self, msg: &str) {
    //         println!("{}", msg);
    //     }
    // }
    // Error: type aliases cannot have an impl block
    
    // ✅ THIS WORKS:
    pub struct ConsoleLogger;
    impl Logger for ConsoleLogger {
        fn log(&self, msg: &str) {
            println!("[CONSOLE] {}", msg);
        }
    }
    
    // ============================================================
    // The REAL problem with type aliases:
    // ============================================================
    
    // If you implement on the underlying type...
    pub type FileLogger = ();
    
    impl Logger for () {
        fn log(&self, msg: &str) {
            println!("[FILE] {}", msg);
        }
    }
    
    // Now BOTH of these do the same thing:
    pub fn use_file_logger() {
        let logger: FileLogger = ();
        logger.log("Using FileLogger");  // Prints "[FILE]"
    }
    
    pub fn use_unit_type() {
        let unit = ();
        unit.log("Using ()");  // ALSO prints "[FILE]" !
    }
    
    // ⚠️ Problem: You've polluted the unit type ()!
    // Anyone in your codebase using () now gets .log()
    
    // ============================================================
    // With structs, each is independent:
    // ============================================================
    
    pub struct JsonLogger;
    impl Logger for JsonLogger {
        fn log(&self, msg: &str) {
            println!(r#"{{"level":"info","msg":"{}"}}"#, msg);
        }
    }
    
    pub struct DebugLogger;
    impl Logger for DebugLogger {
        fn log(&self, msg: &str) {
            println!("[DEBUG] {} (line: {})", msg, line!());
        }
    }
    
    // Each struct is a DISTINCT type
    // No confusion, no pollution
    
    // ============================================================
    // Type aliases ARE useful for:
    // ============================================================
    
    // 1. Making complex types easier to read
    pub type Result<T> = std::result::Result<T, String>;
    
    // 2. Creating synonyms for existing types
    pub type UserId = u32;
    pub type Email = String;
    
    // 3. Simplifying generic types
    pub type Cache<K, V> = std::collections::HashMap<K, V>;
    
    // But NOT for creating new trait implementations!
}

#[cfg(test)]
mod tests {
    use super::WhyNotAlias::*;
    
    #[test]
    fn test_struct_loggers() {
        let console = ConsoleLogger;
        console.log("Hello from console");
        
        let json = JsonLogger;
        json.log("Hello from JSON");
        
        let debug = DebugLogger;
        debug.log("Hello from debug");
        
        // Each is a distinct type with its own behavior
    }
    
    #[test]
    fn test_unit_type_pollution() {
        // This works but is confusing - () has .log() method now!
        ().log("This is weird");
    }
}



