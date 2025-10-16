//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Example 47.5 - Quadratic Probing and Deleted Entry Handling

pub mod Example47_5 {

    use crate::ArraySeqStPerSLit;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap47::AdvancedQuadraticProbing::AdvancedQuadraticProbing::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Chap47::LinearProbing::LinearProbing::*;
    use crate::Chap47::ProbeSequenceExamples::ProbeSequenceExamples::*;
    use crate::Types::Types::*;

    /// Trait for Example 47.5 operations
    pub trait Example47_5Trait {
        /// Example 47.5: Quadratic probing with prime table size
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn example_47_5_quadratic_probing_prime_table() -> TextbookExampleResults;

        /// Example 47.5: Deleted entry handling demonstration
        /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
        fn example_47_5_deleted_entries()               -> LinearProbingHashTable<String, String, StringPositionHashFunction>;

        /// Run comprehensive demonstration of Example 47.5
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn run_example_47_5()                           -> String;

        /// Analyze quadratic probing vs linear probing
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn analyze_quadratic_vs_linear()                -> String;

        /// Demonstrate deleted entry handling
        /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
        fn demonstrate_deleted_entries()                -> String;
    }

    /// Example 47.5: Quadratic probing with prime table size
    /// Shows how quadratic probing avoids primary clustering but may have secondary clustering
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn example_47_5_quadratic_probing_prime_table() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 17; // Prime table size
        let keys = ArraySeqStPerSLit!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];

        let mut probe_sequences = ArraySeqStPerS::empty();
        for i in 0..keys.length() {
            let key = keys.nth(i);
            let probe_viz = analyzer.analyze_quadratic_probing(&key, table_size, 1, 1);
            probe_sequences = probe_sequences.append(&ArraySeqStPerS::singleton(probe_viz));
        }

        let clustering_analysis = "Quadratic probing with prime table size avoids primary clustering by using \
             h_i(k) = (h(k) + i + i²) mod m. Keys that hash to the same position will \
             follow the same probe sequence (secondary clustering), but different initial \
             positions lead to different probe patterns."
            .to_string();

        let performance_summary = "For prime table sizes with c₂ ≠ 0, quadratic probing guarantees that the probe \
             sequence will visit at least (m+1)/2 distinct positions before repeating, \
             providing better distribution than linear probing."
            .to_string();

        TextbookExampleResults {
            example_name: "Example 47.5: Quadratic Probing with Prime Table".to_string(),
            description: "Shows quadratic probing behavior with prime table size m=17".to_string(),
            table_size,
            keys: keys.to_vec(),                       // Convert to Vec for compatibility
            probe_sequences: probe_sequences.to_vec(), // Convert to Vec for compatibility
            clustering_analysis,
            performance_summary,
        }
    }

    /// Example 47.5: Deleted entry handling demonstration
    /// Shows how deleted entries are handled in linear probing hash tables
    /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
    pub fn example_47_5_deleted_entries() -> LinearProbingHashTable<String, String, StringPositionHashFunction> {
        let mut table = create_linear_probing_string_table(8);

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

    /// Run comprehensive demonstration of Example 47.5
    /// Shows both quadratic probing and deleted entry handling
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn run_example_47_5() -> String {
        let mut output = String::new();

        output.push_str("=== Example 47.5: Quadratic Probing and Deleted Entries ===\n\n");

        // Part 1: Quadratic Probing
        output.push_str("PART 1: Quadratic Probing with Prime Table Size\n");
        output.push_str("Hash Function: h_i(k) = (h(k) + i + i²) mod m\n");
        output.push_str("Table Size: 17 (prime)\n\n");

        let quad_results = example_47_5_quadratic_probing_prime_table();
        output.push_str(&format!("Test Keys: {:?}\n", quad_results.keys));
        output.push_str(&format!("Description: {}\n\n", quad_results.description));

        output.push_str(&analyze_quadratic_vs_linear());

        // Part 2: Deleted Entry Handling
        output.push_str("\nPART 2: Deleted Entry Handling\n");
        output.push_str(&demonstrate_deleted_entries());

        output.push_str("\nPerformance Summary:\n");
        output.push_str(&quad_results.performance_summary);
        output.push_str("\n");

        output
    }

    /// Analyze quadratic probing vs linear probing
    /// Compares the clustering behavior of both approaches
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn analyze_quadratic_vs_linear() -> String {
        let mut output = String::new();

        output.push_str("Quadratic vs Linear Probing Analysis:\n\n");

        output.push_str("Linear Probing h_i(k) = (h(k) + i) mod m:\n");
        output.push_str("- Creates primary clustering\n");
        output.push_str("- Adjacent occupied slots form contiguous clusters\n");
        output.push_str("- Performance degrades rapidly with high load factors\n");
        output.push_str("- Simple to implement\n\n");

        output.push_str("Quadratic Probing h_i(k) = (h(k) + c₁i + c₂i²) mod m:\n");
        output.push_str("- Avoids primary clustering\n");
        output.push_str("- May have secondary clustering (same h(k) → same sequence)\n");
        output.push_str("- Better performance at moderate load factors\n");
        output.push_str("- Requires prime table size for optimal coverage\n\n");

        output.push_str("Key Differences:\n");
        output.push_str("- Quadratic probing spreads keys more evenly\n");
        output.push_str("- Linear probing has better cache locality\n");
        output.push_str("- Quadratic probing needs careful parameter selection\n");

        output
    }

    /// Demonstrate deleted entry handling
    /// Shows the three-state system: Empty, Live, Dead
    /// APAS: Work Θ(n), Span Θ(n) where n is number of operations
    pub fn demonstrate_deleted_entries() -> String {
        let mut output = String::new();

        output.push_str("Deleted Entry Handling Demonstration:\n\n");

        output.push_str("Three-State System:\n");
        output.push_str("- EMPTY: Never contained a key-value pair\n");
        output.push_str("- LIVE: Currently contains a key-value pair\n");
        output.push_str("- DEAD: Previously contained a pair, now deleted\n\n");

        output.push_str("Operations:\n");
        output.push_str("1. Insert: Can use EMPTY or DEAD slots\n");
        output.push_str("2. Search: Must probe through DEAD slots\n");
        output.push_str("3. Delete: Changes LIVE slot to DEAD\n\n");

        let table = example_47_5_deleted_entries();
        output.push_str("Example Sequence:\n");
        output.push_str("- Insert B, D, E, A, F (creates LIVE slots)\n");
        output.push_str("- Delete E (creates DEAD slot)\n");
        output.push_str("- Search for F must probe through DEAD slot where E was\n\n");

        output.push_str("Why DEAD slots matter:\n");
        output.push_str("- Maintains probe sequence integrity\n");
        output.push_str("- Prevents search failures for keys inserted after deletions\n");
        output.push_str("- Can be reused for future insertions\n");

        output.push_str(&format!("Final table size: {}\n", table.size()));

        output
    }
}
