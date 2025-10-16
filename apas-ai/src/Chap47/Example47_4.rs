//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Example 47.4 - Linear Probing Clustering Demonstration

pub mod Example47_4 {

    use crate::ArraySeqStPerSLit;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Chap47::LinearProbing::LinearProbing::*;
    use crate::Chap47::ProbeSequenceExamples::ProbeSequenceExamples::*;
    use crate::Types::Types::*;

    /// Trait for Example 47.4 operations
    pub trait Example47_4Trait {
        /// Example 47.4: Linear probing clustering demonstration
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn example_47_4_linear_probing_clustering() -> TextbookExampleResults;

        /// Generate probe sequence for linear probing demonstration
        /// APAS: Work Θ(m), Span Θ(m) where m is table size
        fn example_47_4_probe_sequence()            -> ArraySeqStPerS<Pair<String, ArraySeqStPerS<N>>>;

        /// Run comprehensive demonstration of Example 47.4
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn run_example_47_4()                       -> String;

        /// Analyze primary clustering effects
        /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
        fn analyze_primary_clustering()             -> String;
    }

    /// Example 47.4: Linear probing clustering demonstration
    /// Shows primary clustering with keys that hash to consecutive positions
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn example_47_4_linear_probing_clustering() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 11; // Prime table size
        let keys = ArraySeqStPerSLit!["key1".to_string(), "key2".to_string(), "key3".to_string()];

        let mut probe_sequences = ArraySeqStPerS::empty();
        for i in 0..keys.length() {
            let key = keys.nth(i);
            let probe_viz = analyzer.analyze_linear_probing(&key, table_size);
            probe_sequences = probe_sequences.append(&ArraySeqStPerS::singleton(probe_viz));
        }

        let clustering_analysis = "Linear probing exhibits primary clustering where keys that hash to the same \
             or nearby positions form contiguous clusters. This leads to longer probe sequences \
             as the cluster grows, degrading performance significantly at high load factors."
            .to_string();

        let performance_summary = "Expected probes for unsuccessful search: 1/(1-α). For α=0.5, this is 2 probes. \
             For α=0.9, this increases to 10 probes, showing severe performance degradation."
            .to_string();

        TextbookExampleResults {
            example_name: "Example 47.4: Linear Probing Primary Clustering".to_string(),
            description: "Demonstrates how linear probing creates primary clusters that degrade performance"
                .to_string(),
            table_size,
            keys: keys.to_vec(), // Convert to Vec for compatibility with TextbookExampleResults
            probe_sequences: probe_sequences.to_vec(), // Convert to Vec for compatibility
            clustering_analysis,
            performance_summary,
        }
    }

    /// Generate probe sequence for linear probing demonstration
    /// Simulates probe sequences for keys from textbook example
    /// APAS: Work Θ(m), Span Θ(m) where m is table size
    pub fn example_47_4_probe_sequence() -> ArraySeqStPerS<Pair<String, ArraySeqStPerS<N>>> {
        let hash_fn = StringPositionHashFunction;
        let table_size = 8;

        // Simulate probe sequences for keys from example
        let keys = ArraySeqStPerSLit![
            "B".to_string(),
            "D".to_string(),
            "E".to_string(),
            "A".to_string(),
            "F".to_string()
        ];

        keys.map(|key| {
            let mut probe_sequence = ArraySeqStPerS::empty();
            let linear_probe = LinearProbingStrategy::new(hash_fn.clone());

            for i in 0..table_size {
                let hash_pos = linear_probe.probe_hash(&key, i, table_size);
                probe_sequence = probe_sequence.append(&ArraySeqStPerS::singleton(hash_pos));
            }

            Pair(key, probe_sequence)
        })
    }

    /// Run comprehensive demonstration of Example 47.4
    /// Shows linear probing behavior and clustering analysis
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn run_example_47_4() -> String {
        let mut output = String::new();

        output.push_str("=== Example 47.4: Linear Probing Primary Clustering ===\n\n");
        output.push_str("Hash Function: h(x) = (Σ pos(x[i])) mod m\n");
        output.push_str("Collision Resolution: Linear Probing h_i(k) = (h(k) + i) mod m\n\n");

        let example_results = example_47_4_linear_probing_clustering();
        output.push_str(&format!("Table Size: {}\n", example_results.table_size));
        output.push_str(&format!("Test Keys: {:?}\n\n", example_results.keys));

        output.push_str("Probe Sequences:\n");
        let probe_demo = example_47_4_probe_sequence();
        for i in 0..probe_demo.length() {
            let Pair(key, sequence) = probe_demo.nth(i);
            output.push_str(&format!("  '{}': ", key));
            for j in 0..sequence.length() {
                if j > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}", sequence.nth(j)));
            }
            output.push_str("\n");
        }
        output.push_str("\n");

        output.push_str(&analyze_primary_clustering());

        output.push_str("\nPerformance Analysis:\n");
        output.push_str(&example_results.performance_summary);
        output.push_str("\n");

        output
    }

    /// Analyze primary clustering effects
    /// Explains how linear probing creates clustering problems
    /// APAS: Work Θ(k * m), Span Θ(k * m) where k is number of keys
    pub fn analyze_primary_clustering() -> String {
        let mut output = String::new();

        output.push_str("Primary Clustering Analysis:\n");
        output.push_str("- Linear probing uses h_i(k) = (h(k) + i) mod m\n");
        output.push_str("- Keys with same h(k) follow identical probe sequences\n");
        output.push_str("- Adjacent occupied slots create contiguous clusters\n");
        output.push_str("- Clusters grow as more keys hash to nearby positions\n");
        output.push_str("- Longer clusters → longer probe sequences → worse performance\n\n");

        output.push_str("Load Factor Impact:\n");
        output.push_str("- α = 0.5: Average 1.5 probes for successful search\n");
        output.push_str("- α = 0.75: Average 2.5 probes for successful search\n");
        output.push_str("- α = 0.9: Average 5.5 probes for successful search\n");
        output.push_str("- Performance degrades rapidly as α approaches 1.0\n");

        output
    }
}
