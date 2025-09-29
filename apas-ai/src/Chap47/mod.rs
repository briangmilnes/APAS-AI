//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Tables Module

pub mod HashFunctionTraits;
pub mod NestedHashTable;
pub mod SeparateChaining;
pub mod FlatHashTable;
pub mod LinearProbing;
pub mod QuadraticProbing;
pub mod DoubleHashing;
pub mod HashExamples;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Types::Types::*;

    #[test]
    fn test_hash_function_traits_basic() {
        use HashFunctionTraits::HashFunctionTraits::*;
        
        let hash_fn = StringPositionHashFunction;
        let table_size = 5;
        
        // Test Example 47.1 cases
        assert_eq!(hash_fn.hash(&"aa".to_string(), table_size), 0);
        assert_eq!(hash_fn.hash(&"bb".to_string(), table_size), 2);
        assert_eq!(hash_fn.hash(&"cc".to_string(), table_size), 4);
    }

    #[test]
    fn test_separate_chaining_basic() {
        use SeparateChaining::SeparateChaining::*;
        use HashFunctionTraits::HashFunctionTraits::*;
        
        let mut table = SeparateChainingHashTable::create_table(
            DefaultKeyEquality, 
            StringPositionHashFunction, 
            5
        );
        
        table = table.insert("key1".to_string(), "value1".to_string());
        table = table.insert("key2".to_string(), "value2".to_string());
        
        assert_eq!(table.lookup(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(table.lookup(&"key2".to_string()), Some(&"value2".to_string()));
        assert_eq!(table.lookup(&"nonexistent".to_string()), None);
    }

    #[test]
    fn test_nested_hash_table_basic() {
        use NestedHashTable::NestedHashTable::*;
        
        let mut table: NestedHashTable<String, i32> = NestedHashTable::create_table(8);
        
        table = table.insert("key1".to_string(), 100);
        table = table.insert("key2".to_string(), 200);
        
        assert_eq!(table.lookup(&"key1".to_string()), Some(&100));
        assert_eq!(table.lookup(&"key2".to_string()), Some(&200));
        assert_eq!(table.lookup(&"nonexistent".to_string()), None);
        
        let (load, size) = table.load_and_size();
        assert_eq!(load, 2);
        assert_eq!(size, 8);
    }

    #[test]
    fn test_flat_hash_table_basic() {
        use FlatHashTable::FlatHashTable::*;
        use LinearProbing::LinearProbing::*;
        use HashFunctionTraits::HashFunctionTraits::*;
        
        let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
        let mut table = FlatHashTable::create_table(probe_strategy, 8);
        
        table = table.insert("key1".to_string(), 100);
        table = table.insert("key2".to_string(), 200);
        
        assert_eq!(table.lookup(&"key1".to_string()), Some(&100));
        assert_eq!(table.lookup(&"key2".to_string()), Some(&200));
        assert_eq!(table.lookup(&"nonexistent".to_string()), None);
    }

    #[test]
    fn test_example_47_2() {
        use SeparateChaining::SeparateChaining::*;
        
        let table = create_example_47_2_table();
        
        // Test that all example data is present
        assert_eq!(table.lookup(&"aa".to_string()), Some(&"a".to_string()));
        assert_eq!(table.lookup(&"bb".to_string()), Some(&"b".to_string()));
        assert_eq!(table.lookup(&"jj".to_string()), Some(&"j".to_string()));
        
        let (load, size) = table.load_and_size();
        assert_eq!(load, 10);
        assert_eq!(size, 5);
    }

    #[test]
    fn test_hash_examples() {
        use HashExamples::HashExamples::*;
        
        let hash_demo = example_47_1_demonstration();
        assert_eq!(hash_demo.len(), 10);
        
        let output = run_all_examples();
        assert!(output.contains("Chapter 47 Hash Table Examples"));
        assert!(output.len() > 100);
    }

    #[test]
    fn test_load_factor_management() {
        use HashFunctionTraits::HashFunctionTraits::*;
        
        let manager = LoadFactorManager::new(0.75, 0.25);
        
        assert_eq!(manager.load_factor(5, 10), 0.5);
        assert!(!manager.should_grow(5, 10));
        assert!(manager.should_grow(8, 10));
        assert!(!manager.should_shrink(5, 10));
        assert!(manager.should_shrink(2, 10));
    }

    #[test]
    fn test_collision_handling() {
        use SeparateChaining::SeparateChaining::*;
        use HashFunctionTraits::HashFunctionTraits::*;
        
        let mut table = SeparateChainingHashTable::create_table(
            DefaultKeyEquality, 
            StringPositionHashFunction, 
            5
        );
        
        // These keys should collide: "aa" and "ff" both hash to 0
        table = table.insert("aa".to_string(), "value_aa".to_string());
        table = table.insert("ff".to_string(), "value_ff".to_string());
        
        // Both should be retrievable despite collision
        assert_eq!(table.lookup(&"aa".to_string()), Some(&"value_aa".to_string()));
        assert_eq!(table.lookup(&"ff".to_string()), Some(&"value_ff".to_string()));
        
        let stats = table.statistics();
        assert!(stats.num_collisions >= 1);
    }
}
