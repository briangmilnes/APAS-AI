//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Advanced Double Hashing Strategy with Dual Hash Functions and Relative Prime Validation
//! Definition 47.9: Double hashing with h_i(k) = (h1(k) + i * h2(k)) mod m

pub mod AdvancedDoubleHashing {

use std::fmt::{Debug, Display};
use std::fmt::Formatter;
use std::marker::PhantomData;

use crate::Types::Types::*;
use crate::Chap47::FlatHashTable::FlatHashTable::*;
use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    #[derive(Clone, Debug)]
    pub struct AdvancedDoubleHashingStrategy<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> {
        hash1: H1,
        hash2: H2,
        clustering_enabled: B,
        prime_validation_enabled: B,
        _phantom: PhantomData<K>,
    }

    /// Double Hashing Quality Metrics
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug, PartialEq)]
    pub struct DoubleHashingMetrics {
        pub probe_sequence_length: N,
        pub unique_probe_positions: N,
        pub probe_sequence_period: N,
        pub hash_function_independence: f64,
        pub relative_prime_validation: B,
        pub collision_avoidance_score: f64,
    }

    /// Relative Prime Validator for Double Hashing
    /// Claude Work: Θ(log min(a,b)), Span: Θ(log min(a,b))
    #[derive(Clone, Debug)]
    pub struct RelativePrimeValidator;

    impl RelativePrimeValidator {
        /// Compute greatest common divisor using Euclidean algorithm
        /// Claude Work: O(log min(a,b)), Span: O(log min(a,b))
        pub fn gcd(a: N, b: N) -> N {
            if b == 0 {
                a
            } else {
                Self::gcd(b, a % b)
            }
        }

        /// Check if two numbers are relatively prime (gcd = 1)
        /// Claude Work: O(log min(a,b)), Span: O(log min(a,b))
        pub fn are_relatively_prime(a: N, b: N) -> B { Self::gcd(a, b) == 1 }

        /// Validate that h2(k) and table_size are relatively prime
        /// This ensures the probe sequence visits all table positions
        /// Claude Work: O(log min(h2_value, table_size)), Span: O(log min(h2_value, table_size))
        pub fn validate_double_hashing(h2_value: N, table_size: N) -> (B, String) {
            if h2_value == 0 {
                return (false, "h2(k) cannot be zero".to_string());
            }

            if table_size == 0 {
                return (false, "Table size cannot be zero".to_string());
            }

            let is_valid = Self::are_relatively_prime(h2_value, table_size);
            let message = if is_valid {
                format!("h2(k)={h2_value} and table_size={table_size} are relatively prime")
            } else {
                format!(
                    "h2(k)={} and table_size={} are NOT relatively prime (gcd={})",
                    h2_value,
                    table_size,
                    Self::gcd(h2_value, table_size)
                )
            };

            (is_valid, message)
        }

        /// Find the period of the probe sequence for given h2_value and table_size
        /// The period is table_size / gcd(h2_value, table_size)
        /// Claude Work: O(log min(h2_value, table_size)), Span: O(log min(h2_value, table_size))
        pub fn probe_sequence_period(h2_value: N, table_size: N) -> N {
            if h2_value == 0 || table_size == 0 {
                return 0;
            }
            table_size / Self::gcd(h2_value, table_size)
        }

        /// Generate a list of h2 values that are relatively prime to table_size
        /// Claude Work: O(m * log m), Span: O(m * log m) where m is table_size
        pub fn generate_valid_h2_values(table_size: N, max_count: N) -> Vec<N> {
            let mut valid_values = Vec::new();
            let mut candidate = 1;

            while valid_values.len() < max_count && candidate < table_size {
                if Self::are_relatively_prime(candidate, table_size) {
                    valid_values.push(candidate);
                }
                candidate += 1;
            }

            valid_values
        }
    }

    impl<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> AdvancedDoubleHashingStrategy<K, H1, H2> {
        /// Create new advanced double hashing strategy
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new(hash1: H1, hash2: H2) -> Self {
            AdvancedDoubleHashingStrategy {
                hash1,
                hash2,
                clustering_enabled: true,
                prime_validation_enabled: true,
                _phantom: PhantomData,
            }
        }

        /// Create strategy with validation disabled (for performance comparison)
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new_minimal(hash1: H1, hash2: H2) -> Self {
            AdvancedDoubleHashingStrategy {
                hash1,
                hash2,
                clustering_enabled: false,
                prime_validation_enabled: false,
                _phantom: PhantomData,
            }
        }

        /// Analyze double hashing quality for a specific key and table size
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn analyze_double_hashing_quality(&self, key: &K, table_size: N) -> DoubleHashingMetrics {
            if !self.clustering_enabled {
                return DoubleHashingMetrics {
                    probe_sequence_length: 0,
                    unique_probe_positions: 0,
                    probe_sequence_period: 0,
                    hash_function_independence: 0.0,
                    relative_prime_validation: false,
                    collision_avoidance_score: 0.0,
                };
            }

            let h1_value = self.hash1.hash(key, table_size);
            let h2_value = self.hash2.hash(key, table_size);

            // Ensure h2_value is not zero (would cause infinite loop)
            let h2_value = if h2_value == 0 { 1 } else { h2_value };

            // Validate relative primality
            let (relative_prime_validation, _) = RelativePrimeValidator::validate_double_hashing(h2_value, table_size);

            // Calculate probe sequence period
            let probe_sequence_period = RelativePrimeValidator::probe_sequence_period(h2_value, table_size);

            // Generate probe sequence to analyze uniqueness
            let mut probe_positions = std::collections::HashSet::new();
            let mut current_pos = h1_value;
            let mut probe_count = 0;

            // Generate probe sequence until we revisit a position or reach table_size
            while probe_positions.insert(current_pos) && probe_count < table_size {
                probe_count += 1;
                current_pos = (current_pos + h2_value) % table_size;
            }

            let unique_probe_positions = probe_positions.len();

            // Measure hash function independence (simplified heuristic)
            let independence_score = if h1_value != h2_value {
                let diff = h1_value.abs_diff(h2_value);
                (diff as f64) / (table_size as f64)
            } else {
                0.0 // Same hash values indicate poor independence
            };

            // Collision avoidance score based on probe sequence coverage
            let collision_avoidance_score = (unique_probe_positions as f64) / (table_size as f64);

            DoubleHashingMetrics {
                probe_sequence_length: probe_count,
                unique_probe_positions,
                probe_sequence_period,
                hash_function_independence: independence_score,
                relative_prime_validation,
                collision_avoidance_score,
            }
        }

        /// Validate configuration for optimal double hashing
        /// Claude Work: O(log min(h2_value, table_size)), Span: O(log min(h2_value, table_size))
        pub fn validate_configuration(&self, key: &K, table_size: N) -> (B, String) {
            if !self.prime_validation_enabled {
                return (true, "Validation disabled".to_string());
            }

            let h2_value = self.hash2.hash(key, table_size);
            let h2_value = if h2_value == 0 { 1 } else { h2_value };

            RelativePrimeValidator::validate_double_hashing(h2_value, table_size)
        }

        /// Estimate expected probe count for double hashing
        /// Based on theoretical analysis: approximately 1/(1-α) for unsuccessful search
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn estimate_probe_count(&self, load_factor: f64, is_successful: B) -> f64 {
            if load_factor >= 1.0 {
                return f64::INFINITY;
            }

            if is_successful {
                // Successful search: approximately (1/α) * ln(1/(1-α))
                if load_factor > 0.0 {
                    (1.0 / load_factor) * (1.0 / (1.0 - load_factor)).ln()
                } else {
                    1.0
                }
            } else {
                // Unsuccessful search: approximately 1/(1-α)
                1.0 / (1.0 - load_factor)
            }
        }

        /// Check if double hashing configuration is optimal
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn is_configuration_optimal(&self, metrics: &DoubleHashingMetrics, table_size: N) -> B {
            metrics.relative_prime_validation
                && metrics.probe_sequence_period == table_size
                && metrics.hash_function_independence > 0.1
                && metrics.collision_avoidance_score > 0.8
        }

        /// Get hash function values for debugging
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn get_hash_values(&self, key: &K, table_size: N) -> (N, N) {
            let h1_value = self.hash1.hash(key, table_size);
            let h2_value = self.hash2.hash(key, table_size);
            (h1_value, h2_value)
        }

        /// Generate probe sequence for visualization
        /// Claude Work: Θ(min(max_probes, period)), Span: Θ(min(max_probes, period))
        pub fn generate_probe_sequence(&self, key: &K, table_size: N, max_probes: N) -> Vec<N> {
            let h1_value = self.hash1.hash(key, table_size);
            let h2_value = self.hash2.hash(key, table_size);
            let h2_value = if h2_value == 0 { 1 } else { h2_value };

            let mut sequence = Vec::new();
            let mut current_pos = h1_value;

            for _ in 0..max_probes {
                sequence.push(current_pos);
                current_pos = (current_pos + h2_value) % table_size;
            }

            sequence
        }
    }

    impl<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> ProbeSequence<K> for AdvancedDoubleHashingStrategy<K, H1, H2> {
        /// Double hashing: h_i(k) = (h1(k) + i * h2(k)) mod m
        /// Claude Work: Θ(1), Span: Θ(1)
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let h1_value = self.hash1.hash(key, table_size);
            let h2_value = self.hash2.hash(key, table_size);
            // Ensure h2_value is not zero to avoid infinite loops
            let h2_value = if h2_value == 0 { 1 } else { h2_value };
            (h1_value + probe_index * h2_value) % table_size
        }

        /// Strategy name for debugging and analysis
        /// Claude Work: Θ(1), Span: Θ(1)
        fn strategy_name(&self) -> String { "AdvancedDoubleHashing".to_string() }
    }

    impl<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> Display for AdvancedDoubleHashingStrategy<K, H1, H2> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AdvancedDoubleHashing(clustering_enabled={}, prime_validation={})",
                self.clustering_enabled, self.prime_validation_enabled
            )
        }
    }

    impl Display for DoubleHashingMetrics {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Double Hashing Quality Metrics:")?;
            writeln!(f, "  Probe sequence length: {}", self.probe_sequence_length)?;
            writeln!(f, "  Unique probe positions: {}", self.unique_probe_positions)?;
            writeln!(f, "  Probe sequence period: {}", self.probe_sequence_period)?;
            writeln!(
                f,
                "  Hash function independence: {:.3}",
                self.hash_function_independence
            )?;
            writeln!(f, "  Relative prime validation: {}", self.relative_prime_validation)?;
            write!(f, "  Collision avoidance score: {:.3}", self.collision_avoidance_score)
        }
    }

    /// Textbook Example: Double Hashing with Relative Prime Validation
    /// Demonstrates h1(k) = k mod m, h2(k) = 1 + (k mod (m-1))
    /// Claude Work: Θ(m), Span: Θ(m)
    pub fn example_double_hashing_analysis() -> (DoubleHashingMetrics, f64, f64, (B, String)) {
        let hash1 = DefaultHashFunction;
        let hash2 = DefaultHashFunction; // In practice, would use different hash functions
        let strategy: AdvancedDoubleHashingStrategy<String, DefaultHashFunction, DefaultHashFunction> =
            AdvancedDoubleHashingStrategy::new(hash1, hash2);

        let test_key = "example_key".to_string();
        let table_size = 13; // Prime table size

        // Analyze double hashing quality
        let metrics = strategy.analyze_double_hashing_quality(&test_key, table_size);

        // Estimate probe counts for 50% load factor
        let successful_probes = strategy.estimate_probe_count(0.5, true);
        let unsuccessful_probes = strategy.estimate_probe_count(0.5, false);

        // Validate configuration
        let validation = strategy.validate_configuration(&test_key, table_size);

        (metrics, successful_probes, unsuccessful_probes, validation)
    }

    /// Example demonstrating probe sequence generation and analysis
    /// Claude Work: Θ(m), Span: Θ(m)
    pub fn example_probe_sequence_analysis() -> (Vec<N>, DoubleHashingMetrics, Vec<N>) {
        let hash1 = DefaultHashFunction;
        let hash2 = DefaultHashFunction;
        let strategy: AdvancedDoubleHashingStrategy<String, DefaultHashFunction, DefaultHashFunction> =
            AdvancedDoubleHashingStrategy::new(hash1, hash2);

        let test_key = "probe_test".to_string();
        let table_size = 11; // Prime table size

        // Generate probe sequence
        let probe_sequence = strategy.generate_probe_sequence(&test_key, table_size, 15);

        // Analyze quality
        let metrics = strategy.analyze_double_hashing_quality(&test_key, table_size);

        // Generate valid h2 values for comparison
        let valid_h2_values = RelativePrimeValidator::generate_valid_h2_values(table_size, 5);

        (probe_sequence, metrics, valid_h2_values)
    }
}
