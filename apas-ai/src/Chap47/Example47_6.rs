//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Example 47.6 - Double Hashing and Collision Handling

pub mod Example47_6 {

    use crate::Types::Types::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap47::AdvancedDoubleHashing::AdvancedDoubleHashing::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Chap47::LinearProbing::LinearProbing::*;
    use crate::Chap47::ProbeSequenceExamples::ProbeSequenceExamples::*;
    use crate::ArraySeqStPerSLit;

    /// Trait for Example 47.6 operations
    pub trait Example47_6Trait {
        /// Example 47.6: Double hashing optimal distribution
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn example_47_6_double_hashing_optimal() -> TextbookExampleResults;
        
        /// Example 47.6: Collision handling with linear probing
        /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
        fn example_47_6_collision_handling() -> LinearProbingHashTable<String, String, StringPositionHashFunction>;
        
        /// Run comprehensive demonstration of Example 47.6
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn run_example_47_6() -> String;
        
        /// Analyze double hashing vs other probing methods
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn analyze_double_hashing_advantages() -> String;
        
        /// Demonstrate collision handling strategies
        /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
        fn demonstrate_collision_handling() -> String;
    }

    /// Example 47.6: Double hashing optimal distribution
    /// Demonstrates how double hashing achieves near-optimal probe sequence distribution
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn example_47_6_double_hashing_optimal() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 13; // Prime table size
        let keys = ArraySeqStPerSLit!["hash1".to_string(), "hash2".to_string(), "hash3".to_string()];

        let mut probe_sequences = ArraySeqStPerS::empty();
        for i in 0..keys.length() {
            let key = keys.nth(i);
            let probe_viz = analyzer.analyze_double_hashing(&key, table_size);
            probe_sequences = probe_sequences.append(&ArraySeqStPerS::singleton(probe_viz));
        }

        let clustering_analysis = "Double hashing uses h_i(k) = (h₁(k) + i·h₂(k)) mod m where h₁ and h₂ are \
             independent hash functions. When gcd(h₂(k), m) = 1, the probe sequence visits \
             all table positions before repeating, eliminating both primary and secondary clustering."
            .to_string();

        let performance_summary = "Double hashing achieves performance close to uniform hashing: expected probes \
             for unsuccessful search ≈ 1/(1-α), and for successful search ≈ (1/α)ln(1/(1-α)). \
             This is optimal among open addressing schemes."
            .to_string();

        TextbookExampleResults {
            example_name: "Example 47.6: Double Hashing Optimal Distribution".to_string(),
            description: "Demonstrates double hashing with h₁(k) = k mod m, h₂(k) = 1 + (k mod (m-1))".to_string(),
            table_size,
            keys: keys.to_vec(), // Convert to Vec for compatibility
            probe_sequences: probe_sequences.to_vec(), // Convert to Vec for compatibility
            clustering_analysis,
            performance_summary,
        }
    }

    /// Example 47.6: Collision handling with linear probing
    /// Shows insertion with collision handling using linear probing
    /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
    pub fn example_47_6_collision_handling() -> LinearProbingHashTable<String, String, StringPositionHashFunction> {
        let mut table = create_linear_probing_string_table(8);

        // Insert entries that will cause collisions
        table = table.insert("B".to_string(), "B_value".to_string());
        table = table.insert("E".to_string(), "E_value".to_string());
        table = table.insert("A".to_string(), "A_value".to_string());
        table = table.insert("F".to_string(), "F_value".to_string());

        // Insert D which will probe through occupied slots
        table = table.insert("D".to_string(), "D_value".to_string());

        table
    }

    /// Run comprehensive demonstration of Example 47.6
    /// Shows both double hashing and collision handling
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn run_example_47_6() -> String {
        let mut output = String::new();
        
        output.push_str("=== Example 47.6: Double Hashing and Collision Handling ===\n\n");
        
        // Part 1: Double Hashing
        output.push_str("PART 1: Double Hashing Optimal Distribution\n");
        output.push_str("Hash Functions: h₁(k) = k mod m, h₂(k) = 1 + (k mod (m-1))\n");
        output.push_str("Probe Function: h_i(k) = (h₁(k) + i·h₂(k)) mod m\n");
        output.push_str("Table Size: 13 (prime)\n\n");
        
        let double_results = example_47_6_double_hashing_optimal();
        output.push_str(&format!("Test Keys: {:?}\n", double_results.keys));
        output.push_str(&format!("Description: {}\n\n", double_results.description));
        
        output.push_str(&analyze_double_hashing_advantages());
        
        // Part 2: Collision Handling
        output.push_str("\nPART 2: Collision Handling with Linear Probing\n");
        output.push_str(&demonstrate_collision_handling());
        
        output.push_str("\nPerformance Summary:\n");
        output.push_str(&double_results.performance_summary);
        output.push_str("\n");
        
        output
    }

    /// Analyze double hashing vs other probing methods
    /// Compares double hashing with linear and quadratic probing
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn analyze_double_hashing_advantages() -> String {
        let mut output = String::new();
        
        output.push_str("Double Hashing vs Other Probing Methods:\n\n");
        
        output.push_str("Linear Probing:\n");
        output.push_str("- h_i(k) = (h(k) + i) mod m\n");
        output.push_str("- Primary clustering: adjacent slots form clusters\n");
        output.push_str("- Performance degrades with high load factors\n");
        output.push_str("- Good cache locality\n\n");
        
        output.push_str("Quadratic Probing:\n");
        output.push_str("- h_i(k) = (h(k) + c₁i + c₂i²) mod m\n");
        output.push_str("- Avoids primary clustering\n");
        output.push_str("- Secondary clustering: same h(k) → same sequence\n");
        output.push_str("- Requires prime table size\n\n");
        
        output.push_str("Double Hashing:\n");
        output.push_str("- h_i(k) = (h₁(k) + i·h₂(k)) mod m\n");
        output.push_str("- Eliminates both primary and secondary clustering\n");
        output.push_str("- Near-optimal performance (close to uniform hashing)\n");
        output.push_str("- Requires gcd(h₂(k), m) = 1 for full coverage\n");
        output.push_str("- More complex but best performance\n\n");
        
        output.push_str("Key Advantages of Double Hashing:\n");
        output.push_str("1. Different keys get different probe sequences\n");
        output.push_str("2. Uniform distribution of probe positions\n");
        output.push_str("3. Optimal expected performance: O(1/(1-α))\n");
        output.push_str("4. Works well even at high load factors\n");
        
        output
    }

    /// Demonstrate collision handling strategies
    /// Shows how collisions are resolved in open addressing
    /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
    pub fn demonstrate_collision_handling() -> String {
        let mut output = String::new();
        
        output.push_str("Collision Handling Demonstration:\n\n");
        
        output.push_str("Scenario: Insert keys B, E, A, F, D into table of size 8\n");
        output.push_str("Hash Function: h(k) = sum of ASCII values mod 8\n\n");
        
        let table = example_47_6_collision_handling();
        
        output.push_str("Step-by-step insertion:\n");
        output.push_str("1. Insert 'B': h('B') = 66 mod 8 = 2 → slot 2\n");
        output.push_str("2. Insert 'E': h('E') = 69 mod 8 = 5 → slot 5\n");
        output.push_str("3. Insert 'A': h('A') = 65 mod 8 = 1 → slot 1\n");
        output.push_str("4. Insert 'F': h('F') = 70 mod 8 = 6 → slot 6\n");
        output.push_str("5. Insert 'D': h('D') = 68 mod 8 = 4 → slot 4\n\n");
        
        output.push_str("Collision Resolution Process:\n");
        output.push_str("- If target slot is empty: insert directly\n");
        output.push_str("- If target slot is occupied: probe next slot\n");
        output.push_str("- Continue probing until empty slot found\n");
        output.push_str("- Linear probing: check slots (h(k) + i) mod m\n\n");
        
        output.push_str("Impact on Search:\n");
        output.push_str("- Must follow same probe sequence as insertion\n");
        output.push_str("- Cannot stop at first empty slot (might skip keys)\n");
        output.push_str("- Must probe until key found or original position revisited\n");
        
        output.push_str(&format!("Final table size: {}\n", table.size()));
        
        output
    }
}
