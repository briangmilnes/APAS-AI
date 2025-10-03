//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Advanced Quadratic Probing Strategy with Secondary Clustering Avoidance
//! Definition 47.8: Quadratic probing with h_i(k) = (h(k) + c1*i + c2*i^2) mod m

pub mod AdvancedQuadraticProbing {

use std::fmt::{Debug, Display};

use crate::Types::Types::*;
use crate::Chap47::FlatHashTable::FlatHashTable::*;
use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    #[derive(Clone, Debug)]
    pub struct AdvancedQuadraticProbingStrategy<K: StT, H: HashFunClone<K>> {
        base_hash: H,
        c1: N,
        c2: N,
        clustering_enabled: B,
        prime_validation_enabled: B,
        _phantom: std::marker::PhantomData<K>,
    }

    /// Secondary Clustering Metrics
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecondaryClusteringMetrics {
        pub collision_chains: N,
        pub max_chain_length: N,
        pub avg_chain_length: f64,
        pub secondary_clustering_coefficient: f64,
        pub probe_sequence_diversity: f64,
        pub table_size_is_prime: B,
    }

    /// Prime Table Size Validator
    /// Claude Work: Θ(√n), Span: Θ(√n)
    #[derive(Clone, Debug)]
    pub struct PrimeValidator;

    impl PrimeValidator {
        /// Check if a number is prime
        /// Claude Work: Θ(√n), Span: Θ(√n)
        pub fn is_prime(n: N) -> B {
            if n < 2 {
                return false;
            }
            if n == 2 {
                return true;
            }
            if n % 2 == 0 {
                return false;
            }

            let sqrt_n = (n as f64).sqrt() as N;
            for i in (3..=sqrt_n).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        /// Find next prime greater than or equal to n
        /// Claude Work: O(n log log n), Span: O(n log log n)
        pub fn next_prime(n: N) -> N {
            let mut candidate = n;
            while !Self::is_prime(candidate) {
                candidate += 1;
            }
            candidate
        }

        /// Validate quadratic probing parameters for given table size
        /// For quadratic probing to work well, m should be prime and c2 should be non-zero
        /// Claude Work: Θ(√m), Span: Θ(√m)
        pub fn validate_quadratic_params(table_size: N, _c1: N, c2: N) -> (B, String) {
            let mut issues = Vec::new();

            if !Self::is_prime(table_size) {
                issues.push(format!("Table size {} is not prime", table_size));
            }

            if c2 == 0 {
                issues.push("c2 coefficient is zero (reduces to linear probing)".to_string());
            }

            // For prime table size, c2 should be coprime to m
            if Self::is_prime(table_size) && Self::gcd(c2, table_size) != 1 {
                issues.push(format!("c2 ({}) is not coprime to table size ({})", c2, table_size));
            }

            let is_valid = issues.is_empty();
            let message = if is_valid {
                "Quadratic probing parameters are valid".to_string()
            } else {
                issues.join("; ")
            };

            (is_valid, message)
        }

        /// Compute greatest common divisor
        /// Claude Work: O(log min(a,b)), Span: O(log min(a,b))
        fn gcd(a: N, b: N) -> N {
            if b == 0 {
                a
            } else {
                Self::gcd(b, a % b)
            }
        }
    }

    impl<K: StT, H: HashFunClone<K>> AdvancedQuadraticProbingStrategy<K, H> {
        /// Create new advanced quadratic probing strategy with default coefficients
        /// Uses c1=1, c2=1 which work well with prime table sizes
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new(hash_fn: H) -> Self {
            AdvancedQuadraticProbingStrategy {
                base_hash: hash_fn,
                c1: 1,
                c2: 1,
                clustering_enabled: true,
                prime_validation_enabled: true,
                _phantom: std::marker::PhantomData,
            }
        }

        /// Create strategy with custom coefficients
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new_with_coefficients(hash_fn: H, c1: N, c2: N) -> Self {
            AdvancedQuadraticProbingStrategy {
                base_hash: hash_fn,
                c1,
                c2,
                clustering_enabled: true,
                prime_validation_enabled: true,
                _phantom: std::marker::PhantomData,
            }
        }

        /// Create strategy with all features disabled (for performance comparison)
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new_minimal(hash_fn: H, c1: N, c2: N) -> Self {
            AdvancedQuadraticProbingStrategy {
                base_hash: hash_fn,
                c1,
                c2,
                clustering_enabled: false,
                prime_validation_enabled: false,
                _phantom: std::marker::PhantomData,
            }
        }

        /// Analyze secondary clustering in a hash table
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn analyze_secondary_clustering<V: StT>(
            &self,
            table: &FlatHashTable<K, V, Self>,
        ) -> SecondaryClusteringMetrics
        where
            Self: ProbeSequence<K> + Clone,
        {
            if !self.clustering_enabled {
                return SecondaryClusteringMetrics {
                    collision_chains: 0,
                    max_chain_length: 0,
                    avg_chain_length: 0.0,
                    secondary_clustering_coefficient: 0.0,
                    probe_sequence_diversity: 0.0,
                    table_size_is_prime: false,
                };
            }

            let (load, size) = table.load_and_size();
            if size == 0 {
                return SecondaryClusteringMetrics {
                    collision_chains: 0,
                    max_chain_length: 0,
                    avg_chain_length: 0.0,
                    secondary_clustering_coefficient: 0.0,
                    probe_sequence_diversity: 0.0,
                    table_size_is_prime: false,
                };
            }

            let table_size_is_prime = PrimeValidator::is_prime(size);

            // Analyze collision chains by tracking probe sequences
            let collision_chains;
            let mut probe_sequences = std::collections::HashSet::new();

            // Simulate probe sequences for different keys to measure diversity
            for i in 0..size.min(100) {
                // Sample up to 100 positions
                let mut sequence = Vec::new();
                for probe_idx in 0..size {
                    let pos = (i + self.c1 * probe_idx + self.c2 * probe_idx * probe_idx) % size;
                    sequence.push(pos);
                    if sequence.len() > 10 {
                        // Limit sequence length for analysis
                        break;
                    }
                }
                probe_sequences.insert(sequence);
            }

            // Estimate collision chains based on load factor and probe diversity
            collision_chains = (load as f64 * 0.7) as N; // Heuristic estimate
            let max_chain_length = if collision_chains > 0 {
                load / collision_chains.max(1)
            } else {
                0
            };
            let avg_chain_length = if collision_chains > 0 {
                load as f64 / collision_chains as f64
            } else {
                0.0
            };

            // Secondary clustering coefficient: how much probe sequences overlap
            let expected_diversity = size.min(100) as f64;
            let actual_diversity = probe_sequences.len() as f64;
            let probe_sequence_diversity = if expected_diversity > 0.0 {
                actual_diversity / expected_diversity
            } else {
                0.0
            };

            // Secondary clustering coefficient based on probe sequence similarity
            let secondary_clustering_coefficient = 1.0 - probe_sequence_diversity;

            SecondaryClusteringMetrics {
                collision_chains,
                max_chain_length,
                avg_chain_length,
                secondary_clustering_coefficient,
                probe_sequence_diversity,
                table_size_is_prime,
            }
        }

        /// Validate table size and coefficients for optimal quadratic probing
        /// Claude Work: Θ(√m), Span: Θ(√m)
        pub fn validate_configuration(&self, table_size: N) -> (B, String) {
            if !self.prime_validation_enabled {
                return (true, "Validation disabled".to_string());
            }

            PrimeValidator::validate_quadratic_params(table_size, self.c1, self.c2)
        }

        /// Recommend optimal table size for given capacity
        /// Claude Work: O(n log log n), Span: O(n log log n)
        pub fn recommend_table_size(&self, capacity: N) -> N {
            if !self.prime_validation_enabled {
                return capacity * 2; // Simple doubling
            }

            // Recommend next prime that's at least 2x capacity for good load factor
            let min_size = capacity * 2;
            PrimeValidator::next_prime(min_size)
        }

        /// Estimate expected probe count for quadratic probing
        /// Based on theoretical analysis for prime table sizes
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn estimate_probe_count(&self, load_factor: f64, is_successful: B) -> f64 {
            if load_factor >= 1.0 {
                return f64::INFINITY;
            }

            if is_successful {
                // Successful search: approximately 1 - ln(1-α)/α
                if load_factor > 0.0 {
                    1.0 - (1.0 - load_factor).ln() / load_factor
                } else {
                    1.0
                }
            } else {
                // Unsuccessful search: approximately 1/(1-α)
                1.0 / (1.0 - load_factor)
            }
        }

        /// Check if secondary clustering is problematic
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn is_secondary_clustering_problematic(&self, metrics: &SecondaryClusteringMetrics) -> B {
            metrics.secondary_clustering_coefficient > 0.5 || !metrics.table_size_is_prime
        }

        /// Get coefficients for debugging
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn get_coefficients(&self) -> (N, N) { (self.c1, self.c2) }
    }

    impl<K: StT, H: HashFunClone<K>> ProbeSequence<K> for AdvancedQuadraticProbingStrategy<K, H> {
        /// Quadratic probing: h_i(k) = (h(k) + c1*i + c2*i^2) mod m
        /// Claude Work: Θ(1), Span: Θ(1)
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let base_hash = self.base_hash.hash(key, table_size);
            let quadratic_offset = self.c1 * probe_index + self.c2 * probe_index * probe_index;
            (base_hash + quadratic_offset) % table_size
        }

        /// Strategy name for debugging and analysis
        /// Claude Work: Θ(1), Span: Θ(1)
        fn strategy_name(&self) -> String { format!("AdvancedQuadraticProbing(c1={}, c2={})", self.c1, self.c2) }
    }

    impl<K: StT, H: HashFunClone<K>> Display for AdvancedQuadraticProbingStrategy<K, H> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AdvancedQuadraticProbing(c1={}, c2={}, clustering_enabled={}, prime_validation={})",
                self.c1, self.c2, self.clustering_enabled, self.prime_validation_enabled
            )
        }
    }

    impl Display for SecondaryClusteringMetrics {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Secondary Clustering Metrics:")?;
            writeln!(f, "  Collision chains: {}", self.collision_chains)?;
            writeln!(f, "  Max chain length: {}", self.max_chain_length)?;
            writeln!(f, "  Avg chain length: {:.2}", self.avg_chain_length)?;
            writeln!(
                f,
                "  Secondary clustering coefficient: {:.3}",
                self.secondary_clustering_coefficient
            )?;
            writeln!(f, "  Probe sequence diversity: {:.3}", self.probe_sequence_diversity)?;
            write!(f, "  Table size is prime: {}", self.table_size_is_prime)
        }
    }

    /// Textbook Example: Secondary Clustering Analysis with Prime Table Size
    /// Demonstrates quadratic probing with m=17 (prime), c1=1, c2=1
    /// Claude Work: Θ(m), Span: Θ(m)
    pub fn example_secondary_clustering_analysis() -> (SecondaryClusteringMetrics, f64, f64, (B, String)) {
        let hash_fn = DefaultHashFunction;
        let strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
            AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 1, 1);

        // Create a table with prime size 17
        let table: FlatHashTable<String, String, AdvancedQuadraticProbingStrategy<String, DefaultHashFunction>> =
            FlatHashTable::create_table(strategy.clone(), 17);

        // Analyze secondary clustering
        let metrics = strategy.analyze_secondary_clustering(&table);

        // Estimate probe counts for 50% load factor
        let successful_probes = strategy.estimate_probe_count(0.5, true);
        let unsuccessful_probes = strategy.estimate_probe_count(0.5, false);

        // Validate configuration
        let validation = strategy.validate_configuration(17);

        (metrics, successful_probes, unsuccessful_probes, validation)
    }

    /// Example demonstrating the importance of prime table sizes
    /// Claude Work: Θ(m), Span: Θ(m)
    pub fn example_prime_vs_composite_table_size() -> (SecondaryClusteringMetrics, SecondaryClusteringMetrics) {
        let hash_fn = DefaultHashFunction;

        // Strategy with prime table size (17)
        let strategy_prime: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
            AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn.clone(), 1, 1);
        let table_prime: FlatHashTable<String, String, AdvancedQuadraticProbingStrategy<String, DefaultHashFunction>> =
            FlatHashTable::create_table(strategy_prime.clone(), 17);
        let metrics_prime = strategy_prime.analyze_secondary_clustering(&table_prime);

        // Strategy with composite table size (16)
        let strategy_composite: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
            AdvancedQuadraticProbingStrategy::new_with_coefficients(hash_fn, 1, 1);
        let table_composite: FlatHashTable<
            String,
            String,
            AdvancedQuadraticProbingStrategy<String, DefaultHashFunction>,
        > = FlatHashTable::create_table(strategy_composite.clone(), 16);
        let metrics_composite = strategy_composite.analyze_secondary_clustering(&table_composite);

        (metrics_prime, metrics_composite)
    }
}
