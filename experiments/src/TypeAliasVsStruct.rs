#![allow(dead_code)]

/// Comparing type aliases vs zero-sized structs for trait implementations
pub mod TypeAliasVsStruct {
    
    pub trait Calculator {
        fn add(&self, a: u32, b: u32) -> u32;
        fn multiply(&self, a: u32, b: u32) -> u32;
    }
    
    // ============================================================
    // Approach 1: Type Alias (DOESN'T WORK for new implementations)
    // ============================================================
    
    // This is just an alias for the unit type ()
    pub type AliasCalc = ();
    
    // ❌ PROBLEM: Can't implement traits on type aliases!
    // This would fail:
    // impl Calculator for AliasCalc {
    //     fn add(&self, a: u32, b: u32) -> u32 { a + b }
    //     fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
    // }
    // Error: "type aliases cannot be used as type constructors"
    
    // You'd have to implement it on the underlying type:
    impl Calculator for () {
        fn add(&self, a: u32, b: u32) -> u32 { a + b }
        fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
    }
    
    // But now ANYONE using () gets this implementation!
    // This pollutes the global namespace for the unit type.
    
    // ============================================================
    // Approach 2: Zero-Sized Struct (RECOMMENDED)
    // ============================================================
    
    pub struct StructCalc;  // New, distinct type
    
    impl Calculator for StructCalc {
        fn add(&self, a: u32, b: u32) -> u32 { a + b }
        fn multiply(&self, a: u32, b: u32) -> u32 { a * b }
    }
    
    // ✅ Benefits:
    // - Creates a NEW distinct type
    // - Doesn't pollute existing types
    // - Still zero runtime cost (ZST)
    // - Better type safety and clarity
    
    // ============================================================
    // Approach 3: Type Alias to Existing Type (WORKS but risky)
    // ============================================================
    
    // You could alias to a type you own:
    pub struct MyUnit;
    pub type AliasToMyUnit = MyUnit;
    
    impl Calculator for MyUnit {
        fn add(&self, a: u32, b: u32) -> u32 { a.wrapping_add(b) }
        fn multiply(&self, a: u32, b: u32) -> u32 { a.wrapping_mul(b) }
    }
    
    // Now AliasToMyUnit works because it points to MyUnit
    // But why not just use MyUnit directly?
    
    // ============================================================
    // Size Comparison
    // ============================================================
    
    pub fn show_sizes() {
        use std::mem::size_of;
        
        println!("Size of ():          {} bytes", size_of::<()>());
        println!("Size of AliasCalc:   {} bytes", size_of::<AliasCalc>());
        println!("Size of StructCalc:  {} bytes", size_of::<StructCalc>());
        println!("Size of MyUnit:      {} bytes", size_of::<MyUnit>());
        
        // All are 0 bytes! Zero-sized types!
    }
}

#[cfg(test)]
mod tests {
    use super::TypeAliasVsStruct::*;
    
    #[test]
    fn test_unit_calc() {
        let calc = (); // Using the unit type
        assert_eq!(calc.add(2, 3), 5);
    }
    
    #[test]
    fn test_struct_calc() {
        let calc = StructCalc;
        assert_eq!(calc.add(2, 3), 5);
    }
    
    #[test]
    fn test_alias_to_myunit() {
        let calc: AliasToMyUnit = MyUnit;
        assert_eq!(calc.add(2, 3), 5);
    }
    
    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<AliasCalc>(), 0);
        assert_eq!(size_of::<StructCalc>(), 0);
        assert_eq!(size_of::<MyUnit>(), 0);
    }
}



