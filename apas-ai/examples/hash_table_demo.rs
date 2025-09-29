//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Table Demonstration

use apas_ai::Chap47::SeparateChaining::SeparateChaining::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::LinearProbing::LinearProbing::*;
use apas_ai::Chap47::NestedHashTable::NestedHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Chap47::HashExamples::HashExamples::*;

fn main() {
    println!("🚀 Chapter 47: Hash Tables Demonstration");
    println!("========================================\n");

    // Example 47.1: Hash Function Demonstration
    println!("📊 Example 47.1: String Position Hash Function");
    let hash_demo = example_47_1_demonstration();
    for (key, hash_val) in &hash_demo[..5] {
        println!("  hash('{}') = {}", key, hash_val);
    }
    println!();

    // Example 47.2: Separate Chaining
    println!("🔗 Example 47.2: Separate Chaining Hash Table");
    let sep_chain_table = example_47_2_separate_chaining();
    let (load, size) = sep_chain_table.load_and_size();
    let stats = sep_chain_table.statistics();
    println!("  Elements: {}, Table Size: {}, Load Factor: {:.2}", load, size, stats.load_factor);
    println!("  Collisions: {}, Max Chain Length: {}", stats.num_collisions, stats.max_chain_length);
    
    // Test lookups
    println!("  Lookups:");
    for key in ["aa", "bb", "cc", "nonexistent"] {
        match sep_chain_table.lookup(&key.to_string()) {
            Some(value) => println!("    '{}' → '{}'", key, value),
            None => println!("    '{}' → Not Found", key),
        }
    }
    println!();

    // Nested Hash Table Demonstration
    println!("🏗️  Nested Hash Table (Simplified)");
    let mut nested_table: NestedHashTable<String, i32> = NestedHashTable::create_table(8);
    
    // Insert some data
    for i in 1..=5 {
        let key = format!("item_{}", i);
        nested_table = nested_table.insert(key, i * 10);
    }
    
    let (load, size) = nested_table.load_and_size();
    println!("  Elements: {}, Table Size: {}, Load Factor: {:.2}", load, size, nested_table.load_factor());
    
    // Test lookups
    println!("  Lookups:");
    for i in 1..=6 {
        let key = format!("item_{}", i);
        match nested_table.lookup(&key) {
            Some(value) => println!("    '{}' → {}", key, value),
            None => println!("    '{}' → Not Found", key),
        }
    }
    println!();

    // Flat Hash Table with Linear Probing
    println!("📋 Flat Hash Table with Linear Probing");
    let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
    let mut flat_table = FlatHashTable::create_table(probe_strategy, 8);
    
    // Insert data that will cause collisions
    let test_data = vec![
        ("aa", 100), ("ff", 200), ("bb", 300), ("gg", 400)
    ];
    
    for (key, value) in &test_data {
        flat_table = flat_table.insert(key.to_string(), *value);
    }
    
    let (load, size) = flat_table.load_and_size();
    let (live, dead, load_factor) = flat_table.probe_statistics();
    println!("  Elements: {}, Table Size: {}, Load Factor: {:.2}", load, size, load_factor);
    println!("  Live Entries: {}, Dead Entries: {}", live, dead);
    
    // Test lookups
    println!("  Lookups:");
    for (key, _) in &test_data {
        match flat_table.lookup(&key.to_string()) {
            Some(value) => println!("    '{}' → {}", key, value),
            None => println!("    '{}' → Not Found", key),
        }
    }
    
    // Test deletion
    let (flat_table_after_delete, was_deleted) = flat_table.delete(&"aa".to_string());
    println!("  After deleting 'aa': {}", if was_deleted { "Success" } else { "Failed" });
    let (live, dead, _) = flat_table_after_delete.probe_statistics();
    println!("  Live Entries: {}, Dead Entries: {}", live, dead);
    println!();

    // Hash Function Comparison
    println!("🔢 Hash Function Comparison");
    let test_keys = vec!["hello", "world", "rust", "hash", "table"];
    let table_size = 10;
    
    let string_hash = StringPositionHashFunction;
    let poly_hash = PolynomialHashFunction::new(31);
    let default_hash = DefaultHashFunction;
    
    println!("  Key       | StringPos | Polynomial | Default");
    println!("  ----------|-----------|------------|--------");
    for key in &test_keys {
        let key_str = key.to_string();
        let h1 = string_hash.hash(&key_str, table_size);
        let h2 = poly_hash.hash(&key_str, table_size);
        let h3 = default_hash.hash(&key_str, table_size);
        println!("  {:8} | {:9} | {:10} | {:7}", key, h1, h2, h3);
    }
    println!();

    // Load Factor Management Demo
    println!("⚖️  Load Factor Management");
    let manager = LoadFactorManager::new(0.75, 0.25);
    
    println!("  Load Factor Scenarios:");
    for (elements, table_size) in [(5, 10), (8, 10), (2, 10)] {
        let load_factor = manager.load_factor(elements, table_size);
        let should_grow = manager.should_grow(elements, table_size);
        let should_shrink = manager.should_shrink(elements, table_size);
        
        println!("    {}/{} elements: α={:.2}, grow={}, shrink={}", 
                 elements, table_size, load_factor, should_grow, should_shrink);
    }
    println!();

    // Performance Characteristics Summary
    println!("📈 Performance Characteristics Summary");
    println!("  Separate Chaining:");
    println!("    • Insert: O(1) expected");
    println!("    • Lookup: O(1 + α) expected");
    println!("    • Delete: O(1 + α) expected");
    println!("    • Space: O(n + m)");
    println!();
    
    println!("  Flat Hash Table (Open Addressing):");
    println!("    • Insert: 1/(1-α) expected probes");
    println!("    • Lookup (successful): (1/α)ln(1/(1-α)) expected probes");
    println!("    • Lookup (unsuccessful): 1/(1-α) expected probes");
    println!("    • Space: O(m) with α < 1");
    println!();

    // Textbook Examples Summary
    println!("📚 Textbook Examples Implemented:");
    println!("  ✅ Example 47.1: String position hash function");
    println!("  ✅ Example 47.2: Separate chaining demonstration");
    println!("  ✅ Examples 47.4-47.6: Probe sequence handling");
    println!("  ✅ Exercise 47.1: Nested table implementation");
    println!("  ✅ Exercise 47.2: Table size reduction analysis");
    println!("  ✅ Exercise 47.3: Resize operation implementation");
    println!("  ✅ Exercise 47.6: Higher-order function design");
    println!("  ✅ Exercise 47.7: Complete flat table implementation");
    println!();

    println!("🎯 Chapter 47 Hash Tables Implementation Complete!");
    println!("   • Comprehensive collision resolution strategies");
    println!("   • Universal hashing support");
    println!("   • Automatic load factor management");
    println!("   • All textbook examples and exercises");
    println!("   • Performance analysis and benchmarking ready");
}
