//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Table Examples from Textbook

pub mod HashExamples {
    use crate::Chap47::FlatHashTable::FlatHashTable::ProbeSequence;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Chap47::LinearProbing::LinearProbing::*;
    use crate::Chap47::SeparateChaining::SeparateChaining::*;
    use crate::Types::Types::*;

    /// Example 47.1: String hash function demonstration
    pub fn example_47_1_hash_function() -> StringPositionHashFunction {
        StringPositionHashFunction
    }

    /// Example 47.1: Demonstrate hash function on sample strings
    pub fn example_47_1_demonstration() -> Vec<(String, N)> {
        let hash_fn = example_47_1_hash_function();
        let table_size = 5;

        let test_strings = vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "ee".to_string(),
            "ff".to_string(),
            "gg".to_string(),
            "hh".to_string(),
            "ii".to_string(),
            "jj".to_string(),
        ];

        test_strings
            .into_iter()
            .map(|s| {
                let hash_value = hash_fn.hash(&s, table_size);
                (s, hash_value)
            })
            .collect()
    }

    /// Example 47.2: Separate chaining demonstration
    pub fn example_47_2_separate_chaining() -> StringSeparateChaining<String> {
        create_example_47_2_table()
    }

    /// Example 47.4: Flat table probe sequence demonstration
    pub fn example_47_4_probe_sequence() -> Vec<(String, Vec<N>)> {
        let hash_fn = StringPositionHashFunction;
        let table_size = 8;

        // Simulate probe sequences for keys from example
        let keys = vec![
            "B".to_string(),
            "D".to_string(),
            "E".to_string(),
            "A".to_string(),
            "F".to_string(),
        ];

        keys.into_iter()
            .map(|key| {
                let mut probe_sequence = Vec::new();
                let linear_probe = LinearProbingStrategy::new(hash_fn.clone());

                for i in 0..table_size {
                    let hash_pos = linear_probe.probe_hash(&key, i, table_size);
                    probe_sequence.push(hash_pos);
                }

                (key, probe_sequence)
            })
            .collect()
    }

    /// Example 47.5: Deleted entry handling demonstration
    pub fn example_47_5_deleted_entries() -> LinearProbingHashTable<String, String, StringPositionHashFunction> {
        let mut table = LinearProbingFactory::create_string_table(8);

        // Insert initial entries
        table = table.insert("B".to_string(), "B_value".to_string());
        table = table.insert("D".to_string(), "D_value".to_string());
        table = table.insert("E".to_string(), "E_value".to_string());
        table = table.insert("A".to_string(), "A_value".to_string());
        table = table.insert("F".to_string(), "F_value".to_string());

        // Delete an entry to create a "Dead" slot
        let (table_after_delete, _) = table.delete(&"E".to_string());

        table_after_delete
    }

    /// Example 47.6: Insertion with collision handling
    pub fn example_47_6_collision_handling() -> LinearProbingHashTable<String, String, StringPositionHashFunction> {
        let mut table = LinearProbingFactory::create_string_table(8);

        // Insert entries that will cause collisions
        table = table.insert("B".to_string(), "B_value".to_string());
        table = table.insert("E".to_string(), "E_value".to_string());
        table = table.insert("A".to_string(), "A_value".to_string());
        table = table.insert("F".to_string(), "F_value".to_string());

        // Insert D which will probe through occupied slots
        table = table.insert("D".to_string(), "D_value".to_string());

        table
    }

    /// Exercise 47.1: Nested table implementation using Table ADT
    pub fn exercise_47_1_nested_implementation() -> String {
        r#"
Exercise 47.1: Nested Hash Table Implementation

The nested hash table can be implemented using the Table ADT as follows:

1. Outer Table: Use an array of size m to map hash codes to inner tables
2. Inner Tables: Each position contains a Table ADT instance
3. Hash Function: Maps keys to positions [0, m-1] in the outer array

Operations:
- createTable(eq_fn, hash_gen, size): Initialize array with empty Table instances
- insert(table, key, value): Hash key to find inner table, insert into that table
- lookup(table, key): Hash key to find inner table, lookup in that table
- resize(table, new_size): Create new table, rehash all key-value pairs

Cost Analysis:
- With universal hashing, expected inner table size is O(1 + α)
- All operations have expected cost O(1 + α) where α = n/m
- Resize operation costs O(n) to rehash all elements

This implementation is demonstrated in our NestedHashTable module.
        "#
        .to_string()
    }

    /// Exercise 47.2: Table size reduction analysis
    pub fn exercise_47_2_size_reduction() -> String {
        r#"
Exercise 47.2: Hash Table Size Reduction

Yes, it makes sense to reduce hash table size under certain conditions:

When to Reduce:
1. Load factor α falls below a threshold (e.g., α < 0.25)
2. Table size is significantly larger than minimum needed
3. Memory usage optimization is important

How to Reduce:
1. Create new table with size = current_size / 2
2. Rehash all existing elements into new table
3. Ensure minimum table size (e.g., never below 8)

Benefits:
- Reduced memory usage
- Better cache locality
- Maintained performance with appropriate load factor

Costs:
- O(n) work to rehash all elements
- Temporary memory overhead during resize
- Amortized analysis still holds with proper thresholds

Implementation:
- Monitor load factor after deletions
- Use hysteresis (different thresholds for grow/shrink)
- Example: grow at α > 0.75, shrink at α < 0.25

This is implemented in our LoadFactorManager.
        "#
        .to_string()
    }

    /// Exercise 47.3: Resize operation implementation and cost analysis
    pub fn exercise_47_3_resize_implementation() -> String {
        r#"
Exercise 47.3: Resize Operation Implementation

The resize operation for separate chaining:

Algorithm:
1. Create new hash table with target size
2. Iterate through all chains in old table
3. For each key-value pair, rehash and insert into new table
4. Return new table

Cost Analysis:
- Work: Θ(n) where n is number of elements
  - Must visit each element exactly once
  - Each rehash and insert is O(1) expected
- Span: Θ(n) sequential rehashing
  - Could be parallelized to O(log n) span

Amortized Analysis:
- Double when α > threshold (e.g., 0.75)
- Each element pays for future elements
- Amortized cost per operation remains O(1)

Implementation Details:
- Use insert_without_resize to avoid recursive resizing
- Handle both growth and shrinkage
- Maintain minimum table size
- Use appropriate load factor thresholds

This is fully implemented in our SeparateChaining module.
        "#
        .to_string()
    }

    /// Exercise 47.6: Higher-order function implementation
    pub fn exercise_47_6_higher_order() -> String {
        r#"
Exercise 47.6: Higher-Order Function for Flat Hash Tables

The parametric flat hash table operations can be unified with a single higher-order function:

```rust
fn probe_operation<K, V, F, R>(
    table: &FlatHashTable<K, V>,
    key: &K,
    operation: F
) -> R
where
    F: Fn(&Entry<K, V>, usize) -> Option<R>
{
    let mut probe_index = 0;
    while probe_index < table.size() {
        let pos = table.probe_hash(key, probe_index);
        let entry = &table[pos];
        
        if let Some(result) = operation(entry, pos) {
            return result;
        }
        
        probe_index += 1;
    }
    // Handle not found case
}
```

Usage:
- lookup: operation returns Some(value) on Live match, None to continue
- insert: operation returns Some(position) on Empty/Dead/matching Live
- delete: operation returns Some(position) on matching Live

This unifies the probe logic while allowing different behaviors.
        "#
        .to_string()
    }

    /// Exercise 47.7: Complete flat hash table implementation
    pub fn exercise_47_7_complete_implementation() -> String {
        r#"
Exercise 47.7: Complete Flat Hash Table Implementation

Additional operations for the parametric flat hash table:

1. Resize Operation:
   - Create new table with target size
   - Rehash all Live entries (skip Empty/Dead)
   - Cost: O(n) work, O(n) span

2. Load Factor Management:
   - Monitor (live + dead) / table_size
   - Resize when threshold exceeded
   - Use lower thresholds than separate chaining

3. Statistics Collection:
   - Track probe distances
   - Monitor clustering effects
   - Measure actual vs. theoretical performance

4. Probe Sequence Validation:
   - Ensure probe sequences visit all positions
   - Handle edge cases (h2 = 0 in double hashing)
   - Verify termination conditions

5. Memory Management:
   - Periodic cleanup of Dead entries
   - Optimize for cache performance
   - Consider Robin Hood hashing variants

All these features are implemented in our FlatHashTable module
with support for different probing strategies.
        "#
        .to_string()
    }

    /// Comprehensive demonstration of all examples
    pub fn run_all_examples() -> String {
        let mut output = String::new();

        output.push_str("=== Chapter 47 Hash Table Examples ===\n\n");

        // Example 47.1
        output.push_str("Example 47.1 - Hash Function:\n");
        let hash_demo = example_47_1_demonstration();
        for (key, hash_val) in hash_demo {
            output.push_str(&format!("  hash('{}') = {}\n", key, hash_val));
        }
        output.push_str("\n");

        // Example 47.2
        output.push_str("Example 47.2 - Separate Chaining:\n");
        let sep_chain = example_47_2_separate_chaining();
        let stats = sep_chain.statistics();
        output.push_str(&format!("  {}\n\n", stats));

        // Example 47.4
        output.push_str("Example 47.4 - Probe Sequences:\n");
        let probe_demo = example_47_4_probe_sequence();
        for (key, sequence) in probe_demo {
            output.push_str(&format!("  '{}': {:?}\n", key, sequence));
        }
        output.push_str("\n");

        // Exercises
        output.push_str("Exercise Solutions:\n");
        output.push_str(&exercise_47_1_nested_implementation());
        output.push_str(&exercise_47_2_size_reduction());

        output
    }
}
