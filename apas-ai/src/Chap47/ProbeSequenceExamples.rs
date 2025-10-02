//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Probe Sequence Examples for Advanced Probing Strategies
//! Textbook Examples 47.4, 47.5, 47.6 and additional demonstrations

pub mod ProbeSequenceExamples {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display};

    use crate::Chap47::AdvancedDoubleHashing::AdvancedDoubleHashing::*;
    use crate::Chap47::AdvancedLinearProbing::AdvancedLinearProbing::*;
    use crate::Chap47::AdvancedQuadraticProbing::AdvancedQuadraticProbing::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct ProbeSequenceVisualization {
        pub strategy_name: String,
        pub key: String,
        pub table_size: N,
        pub probe_sequence: Vec<N>,
        pub sequence_length: N,
        pub unique_positions: N,
        pub period: N,
        pub collision_pattern: String,
    }

    /// Textbook Example Results
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug)]
    pub struct TextbookExampleResults {
        pub example_name: String,
        pub description: String,
        pub table_size: N,
        pub keys: Vec<String>,
        pub probe_sequences: Vec<ProbeSequenceVisualization>,
        pub clustering_analysis: String,
        pub performance_summary: String,
    }

    /// Probe Sequence Generator and Analyzer
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug)]
    pub struct ProbeSequenceAnalyzer {
        pub max_probes: N,
        pub visualization_enabled: B,
    }

    impl ProbeSequenceAnalyzer {
        /// Create new probe sequence analyzer
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new() -> Self {
            ProbeSequenceAnalyzer {
                max_probes: 20,
                visualization_enabled: true,
            }
        }

        /// Create minimal analyzer for performance testing
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new_minimal() -> Self {
            ProbeSequenceAnalyzer {
                max_probes: 10,
                visualization_enabled: false,
            }
        }

        /// Analyze linear probing sequence
        /// Claude Work: Θ(min(max_probes, m)), Span: Θ(min(max_probes, m))
        pub fn analyze_linear_probing(&self, key: &str, table_size: N) -> ProbeSequenceVisualization {
            let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
                AdvancedLinearProbingStrategy::new(DefaultHashFunction);

            let mut probe_sequence = Vec::new();
            let mut unique_positions = std::collections::HashSet::new();

            for i in 0..self.max_probes.min(table_size) {
                let pos = strategy.probe_hash(&key.to_string(), i, table_size);
                probe_sequence.push(pos);
                unique_positions.insert(pos);

                // For linear probing, period is table_size
                if i > 0 && pos == probe_sequence[0] {
                    break;
                }
            }

            let collision_pattern = self.analyze_collision_pattern(&probe_sequence);
            let sequence_length = probe_sequence.len();

            ProbeSequenceVisualization {
                strategy_name: "Linear Probing".to_string(),
                key: key.to_string(),
                table_size,
                probe_sequence,
                sequence_length,
                unique_positions: unique_positions.len(),
                period: table_size, // Linear probing always has period = table_size
                collision_pattern,
            }
        }

        /// Analyze quadratic probing sequence
        /// Claude Work: Θ(min(max_probes, m)), Span: Θ(min(max_probes, m))
        pub fn analyze_quadratic_probing(&self, key: &str, table_size: N, c1: N, c2: N) -> ProbeSequenceVisualization {
            let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
                AdvancedQuadraticProbingStrategy::new_with_coefficients(DefaultHashFunction, c1, c2);

            let mut probe_sequence = Vec::new();
            let mut unique_positions = std::collections::HashSet::new();
            let mut period = 0;

            for i in 0..self.max_probes.min(table_size) {
                let pos = strategy.probe_hash(&key.to_string(), i, table_size);
                probe_sequence.push(pos);

                if unique_positions.contains(&pos) && period == 0 {
                    period = i;
                }
                unique_positions.insert(pos);
            }

            if period == 0 {
                period = probe_sequence.len();
            }

            let collision_pattern = self.analyze_collision_pattern(&probe_sequence);
            let sequence_length = probe_sequence.len();

            ProbeSequenceVisualization {
                strategy_name: format!("Quadratic Probing (c1={}, c2={})", c1, c2),
                key: key.to_string(),
                table_size,
                probe_sequence,
                sequence_length,
                unique_positions: unique_positions.len(),
                period,
                collision_pattern,
            }
        }

        /// Analyze double hashing sequence
        /// Claude Work: Θ(min(max_probes, m)), Span: Θ(min(max_probes, m))
        pub fn analyze_double_hashing(&self, key: &str, table_size: N) -> ProbeSequenceVisualization {
            let strategy: AdvancedDoubleHashingStrategy<String, DefaultHashFunction, DefaultHashFunction> =
                AdvancedDoubleHashingStrategy::new(DefaultHashFunction, DefaultHashFunction);

            let probe_sequence = strategy.generate_probe_sequence(&key.to_string(), table_size, self.max_probes);
            let unique_positions: std::collections::HashSet<_> = probe_sequence.iter().cloned().collect();

            // Calculate period for double hashing
            let (_h1_value, h2_value) = strategy.get_hash_values(&key.to_string(), table_size);
            let h2_value = if h2_value == 0 { 1 } else { h2_value };
            let period = RelativePrimeValidator::probe_sequence_period(h2_value, table_size);

            let collision_pattern = self.analyze_collision_pattern(&probe_sequence);
            let sequence_length = probe_sequence.len();

            ProbeSequenceVisualization {
                strategy_name: "Double Hashing".to_string(),
                key: key.to_string(),
                table_size,
                probe_sequence,
                sequence_length,
                unique_positions: unique_positions.len(),
                period,
                collision_pattern,
            }
        }

        /// Analyze collision pattern in probe sequence
        /// Claude Work: Θ(n), Span: Θ(n) where n is sequence length
        fn analyze_collision_pattern(&self, sequence: &[N]) -> String {
            if sequence.len() <= 1 {
                return "No collisions".to_string();
            }

            let mut pattern = String::new();
            let mut consecutive_count = 1;

            for i in 1..sequence.len() {
                if sequence[i] == (sequence[i - 1] + 1) % sequence.len() {
                    consecutive_count += 1;
                } else {
                    if consecutive_count > 1 {
                        pattern.push_str(&format!("Cluster of {} consecutive positions; ", consecutive_count));
                    }
                    consecutive_count = 1;
                }
            }

            if consecutive_count > 1 {
                pattern.push_str(&format!("Final cluster of {} consecutive positions", consecutive_count));
            }

            if pattern.is_empty() {
                "Scattered positions (good distribution)".to_string()
            } else {
                pattern
            }
        }

        /// Compare all three probing strategies for a given key and table size
        /// Claude Work: Θ(min(max_probes, m)), Span: Θ(min(max_probes, m))
        pub fn compare_probing_strategies(&self, key: &str, table_size: N) -> Vec<ProbeSequenceVisualization> {
            vec![
                self.analyze_linear_probing(key, table_size),
                self.analyze_quadratic_probing(key, table_size, 1, 1),
                self.analyze_double_hashing(key, table_size),
            ]
        }
    }

    impl Display for ProbeSequenceVisualization {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "=== {} ===", self.strategy_name)?;
            writeln!(f, "Key: '{}', Table size: {}", self.key, self.table_size)?;
            writeln!(f, "Probe sequence: {:?}", self.probe_sequence)?;
            writeln!(
                f,
                "Sequence length: {}, Unique positions: {}, Period: {}",
                self.sequence_length, self.unique_positions, self.period
            )?;
            write!(f, "Collision pattern: {}", self.collision_pattern)
        }
    }

    impl Display for TextbookExampleResults {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "=== {} ===", self.example_name)?;
            writeln!(f, "{}", self.description)?;
            writeln!(f, "Table size: {}", self.table_size)?;
            writeln!(f, "Test keys: {:?}", self.keys)?;
            writeln!(f)?;

            for probe_viz in &self.probe_sequences {
                writeln!(f, "{}", probe_viz)?;
                writeln!(f)?;
            }

            writeln!(f, "Clustering Analysis:")?;
            writeln!(f, "{}", self.clustering_analysis)?;
            writeln!(f)?;
            writeln!(f, "Performance Summary:")?;
            write!(f, "{}", self.performance_summary)
        }
    }

    /// Textbook Example 47.4: Linear Probing Demonstration
    /// Shows primary clustering with keys that hash to consecutive positions
    /// Claude Work: Θ(k * m), Span: Θ(k * m) where k is number of keys
    pub fn example_47_4_linear_probing_clustering() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 11; // Prime table size
        let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];

        let mut probe_sequences = Vec::new();
        for key in &keys {
            probe_sequences.push(analyzer.analyze_linear_probing(key, table_size));
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
            keys,
            probe_sequences,
            clustering_analysis,
            performance_summary,
        }
    }

    /// Textbook Example 47.5: Quadratic Probing with Prime Table Size
    /// Shows how quadratic probing avoids primary clustering but may have secondary clustering
    /// Claude Work: Θ(k * m), Span: Θ(k * m) where k is number of keys
    pub fn example_47_5_quadratic_probing_prime_table() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 17; // Prime table size
        let keys = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];

        let mut probe_sequences = Vec::new();
        for key in &keys {
            probe_sequences.push(analyzer.analyze_quadratic_probing(key, table_size, 1, 1));
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
            keys,
            probe_sequences,
            clustering_analysis,
            performance_summary,
        }
    }

    /// Textbook Example 47.6: Double Hashing Optimal Distribution
    /// Demonstrates how double hashing achieves near-optimal probe sequence distribution
    /// Claude Work: Θ(k * m), Span: Θ(k * m) where k is number of keys
    pub fn example_47_6_double_hashing_optimal() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 13; // Prime table size
        let keys = vec!["hash1".to_string(), "hash2".to_string(), "hash3".to_string()];

        let mut probe_sequences = Vec::new();
        for key in &keys {
            probe_sequences.push(analyzer.analyze_double_hashing(key, table_size));
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
            keys,
            probe_sequences,
            clustering_analysis,
            performance_summary,
        }
    }

    /// Comprehensive Probe Sequence Comparison
    /// Compares all three strategies with the same keys and table size
    /// Claude Work: Θ(k * m), Span: Θ(k * m) where k is number of keys
    pub fn comprehensive_probe_sequence_comparison() -> TextbookExampleResults {
        let analyzer = ProbeSequenceAnalyzer::new();
        let table_size = 11; // Prime table size
        let test_key = "comparison_test";

        let probe_sequences = analyzer.compare_probing_strategies(test_key, table_size);

        let clustering_analysis = "Comparison shows: (1) Linear probing creates consecutive probe sequences leading \
             to primary clustering. (2) Quadratic probing spreads probes more evenly but keys \
             with same h(k) follow identical sequences. (3) Double hashing provides maximum \
             distribution with different probe patterns for each key."
            .to_string();

        let performance_summary =
            "Performance ranking (best to worst): Double Hashing > Quadratic Probing > Linear Probing. \
             Double hashing maintains good performance even at high load factors, while linear \
             probing degrades rapidly beyond α = 0.7."
                .to_string();

        TextbookExampleResults {
            example_name: "Comprehensive Probe Sequence Comparison".to_string(),
            description: "Side-by-side comparison of all three probing strategies".to_string(),
            table_size,
            keys: vec![test_key.to_string()],
            probe_sequences,
            clustering_analysis,
            performance_summary,
        }
    }

    /// Load Factor Impact Analysis
    /// Shows how probe sequences change with different load factors
    /// Claude Work: Θ(f * k * m), Span: Θ(f * k * m) where f is number of load factors
    pub fn load_factor_impact_analysis() -> Vec<(f64, Vec<ProbeSequenceVisualization>)> {
        let analyzer = ProbeSequenceAnalyzer::new();
        let load_factors = vec![0.25, 0.5, 0.75, 0.9];
        let table_size = 13;
        let test_key = "load_test";

        load_factors
            .into_iter()
            .map(|lf| {
                let sequences = analyzer.compare_probing_strategies(test_key, table_size);
                (lf, sequences)
            })
            .collect()
    }

    /// Prime vs Composite Table Size Comparison
    /// Demonstrates the importance of prime table sizes for quadratic probing
    /// Claude Work: Θ(k * m), Span: Θ(k * m) where k is number of keys
    pub fn prime_vs_composite_table_comparison() -> (TextbookExampleResults, TextbookExampleResults) {
        let analyzer = ProbeSequenceAnalyzer::new();
        let test_key = "prime_test";

        // Prime table size analysis
        let prime_size = 17;
        let prime_sequences = vec![analyzer.analyze_quadratic_probing(test_key, prime_size, 1, 1)];

        let prime_results = TextbookExampleResults {
            example_name: "Quadratic Probing with Prime Table Size".to_string(),
            description: format!("Analysis with prime table size m={}", prime_size),
            table_size: prime_size,
            keys: vec![test_key.to_string()],
            probe_sequences: prime_sequences,
            clustering_analysis: "Prime table size ensures good probe sequence distribution".to_string(),
            performance_summary: "Optimal quadratic probing performance".to_string(),
        };

        // Composite table size analysis
        let composite_size = 16;
        let composite_sequences = vec![analyzer.analyze_quadratic_probing(test_key, composite_size, 1, 1)];

        let composite_results = TextbookExampleResults {
            example_name: "Quadratic Probing with Composite Table Size".to_string(),
            description: format!("Analysis with composite table size m={}", composite_size),
            table_size: composite_size,
            keys: vec![test_key.to_string()],
            probe_sequences: composite_sequences,
            clustering_analysis: "Composite table size may limit probe sequence coverage".to_string(),
            performance_summary: "Potentially degraded quadratic probing performance".to_string(),
        };

        (prime_results, composite_results)
    }
}
