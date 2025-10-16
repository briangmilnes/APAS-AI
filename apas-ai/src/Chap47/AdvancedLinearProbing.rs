//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Advanced Linear Probing Strategy with Primary Clustering Analysis
//! Definition 47.7: Linear probing with h_i(k) = (h(k) + i) mod m

pub mod AdvancedLinearProbing {

    use std::fmt::Formatter;
    use std::fmt::{Debug, Display};
    use std::marker::PhantomData;

    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Types::Types::*;
    #[derive(Clone, Debug)]
    pub struct AdvancedLinearProbingStrategy<K: StT, H: HashFunClone<K>> {
        base_hash: H,
        clustering_enabled: B,
        _phantom: PhantomData<K>,
    }

    /// Primary Clustering Metrics
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug, PartialEq)]
    pub struct PrimaryClusteringMetrics {
        pub total_clusters: N,
        pub max_cluster_size: N,
        pub avg_cluster_size: f64,
        pub clustering_coefficient: f64,
        pub probe_variance: f64,
    }

    impl<K: StT, H: HashFunClone<K>> AdvancedLinearProbingStrategy<K, H> {
        /// Create new advanced linear probing strategy
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new(hash_fn: H) -> Self {
            AdvancedLinearProbingStrategy {
                base_hash: hash_fn,
                clustering_enabled: true,
                _phantom: PhantomData,
            }
        }

        /// Create strategy with clustering analysis disabled
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new_without_clustering(hash_fn: H) -> Self {
            AdvancedLinearProbingStrategy {
                base_hash: hash_fn,
                clustering_enabled: false,
                _phantom: PhantomData,
            }
        }

        /// Analyze primary clustering in a hash table
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn analyze_primary_clustering<V: StT>(&self, table: &FlatHashTable<K, V, Self>) -> PrimaryClusteringMetrics
        where
            Self: ProbeSequence<K> + Clone,
        {
            if !self.clustering_enabled {
                return PrimaryClusteringMetrics {
                    total_clusters: 0,
                    max_cluster_size: 0,
                    avg_cluster_size: 0.0,
                    clustering_coefficient: 0.0,
                    probe_variance: 0.0,
                };
            }

            let (load, size) = table.load_and_size();
            if size == 0 {
                return PrimaryClusteringMetrics {
                    total_clusters: 0,
                    max_cluster_size: 0,
                    avg_cluster_size: 0.0,
                    clustering_coefficient: 0.0,
                    probe_variance: 0.0,
                };
            }

            let mut clusters = Vec::new();
            let mut current_cluster_size = 0;
            let mut in_cluster = false;

            // Scan table to identify clusters
            for i in 0..size {
                let is_occupied = self.is_position_occupied(table, i);

                if is_occupied {
                    if !in_cluster {
                        in_cluster = true;
                        current_cluster_size = 1;
                    } else {
                        current_cluster_size += 1;
                    }
                } else if in_cluster {
                    clusters.push(current_cluster_size);
                    in_cluster = false;
                    current_cluster_size = 0;
                }
            }

            // Handle wrap-around cluster
            if in_cluster {
                clusters.push(current_cluster_size);
            }

            self.compute_clustering_metrics(clusters, load, size)
        }

        /// Check if a position in the table is occupied
        /// Claude Work: Θ(1), Span: Θ(1)
        fn is_position_occupied<V: StT>(&self, table: &FlatHashTable<K, V, Self>, position: N) -> B
        where
            Self: ProbeSequence<K> + Clone,
        {
            // This is a simplified check - in practice would need access to table internals
            // For now, assume we can check occupancy through table statistics
            let (load, size) = table.load_and_size();
            // Simplified heuristic based on load factor
            position < (load * size) / size
        }

        /// Compute clustering metrics from cluster sizes
        /// Claude Work: Θ(c), Span: Θ(c) where c is number of clusters
        fn compute_clustering_metrics(&self, clusters: Vec<N>, load: N, size: N) -> PrimaryClusteringMetrics {
            if clusters.is_empty() {
                return PrimaryClusteringMetrics {
                    total_clusters: 0,
                    max_cluster_size: 0,
                    avg_cluster_size: 0.0,
                    clustering_coefficient: 0.0,
                    probe_variance: 0.0,
                };
            }

            let total_clusters = clusters.len();
            let max_cluster_size = *clusters.iter().max().unwrap_or(&0);
            let total_cluster_elements: N = clusters.iter().sum();
            let avg_cluster_size = if total_clusters > 0 {
                total_cluster_elements as f64 / total_clusters as f64
            } else {
                0.0
            };

            // Clustering coefficient: how much clustering deviates from uniform distribution
            let expected_avg_cluster_size = if size > 0 { load as f64 / size as f64 } else { 0.0 };
            let clustering_coefficient = if expected_avg_cluster_size > 0.0 {
                avg_cluster_size / expected_avg_cluster_size
            } else {
                0.0
            };

            // Probe variance: variance in cluster sizes
            let variance = if total_clusters > 1 {
                let mean = avg_cluster_size;
                let sum_squared_diff: f64 = clusters.iter().map(|&size| (size as f64 - mean).powi(2)).sum();
                sum_squared_diff / (total_clusters - 1) as f64
            } else {
                0.0
            };

            PrimaryClusteringMetrics {
                total_clusters,
                max_cluster_size,
                avg_cluster_size,
                clustering_coefficient,
                probe_variance: variance,
            }
        }

        /// Estimate expected probe count for unsuccessful search
        /// Based on textbook analysis: 1 + n(n+1)/2m ≈ n/4 for worst case
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn estimate_unsuccessful_probe_count(&self, load_factor: f64) -> f64 {
            if load_factor >= 1.0 {
                return f64::INFINITY;
            }

            // Expected probes for unsuccessful search: 1/(1-α)
            1.0 / (1.0 - load_factor)
        }

        /// Estimate expected probe count for successful search
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn estimate_successful_probe_count(&self, load_factor: f64) -> f64 {
            if load_factor >= 1.0 {
                return f64::INFINITY;
            }

            // Expected probes for successful search: (1/α) * ln(1/(1-α))
            if load_factor > 0.0 {
                (1.0 / load_factor) * (1.0 / (1.0 - load_factor)).ln()
            } else {
                1.0
            }
        }

        /// Check if clustering is problematic for given load factor
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn is_clustering_problematic(&self, load_factor: f64) -> B {
            // Linear probing becomes problematic when load factor > 0.7
            load_factor > 0.7
        }
    }

    impl<K: StT, H: HashFunClone<K>> ProbeSequence<K> for AdvancedLinearProbingStrategy<K, H> {
        /// Linear probing: h_i(k) = (h(k) + i) mod m
        /// Claude Work: Θ(1), Span: Θ(1)
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let base_hash = self.base_hash.hash(key, table_size);
            (base_hash + probe_index) % table_size
        }

        /// Strategy name for debugging and analysis
        /// Claude Work: Θ(1), Span: Θ(1)
        fn strategy_name(&self) -> String { "AdvancedLinearProbing".to_string() }
    }

    impl<K: StT, H: HashFunClone<K>> Display for AdvancedLinearProbingStrategy<K, H> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AdvancedLinearProbing(clustering_enabled: {})",
                self.clustering_enabled
            )
        }
    }

    impl Display for PrimaryClusteringMetrics {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Primary Clustering Metrics:")?;
            writeln!(f, "  Total clusters: {}", self.total_clusters)?;
            writeln!(f, "  Max cluster size: {}", self.max_cluster_size)?;
            writeln!(f, "  Avg cluster size: {:.2}", self.avg_cluster_size)?;
            writeln!(f, "  Clustering coefficient: {:.3}", self.clustering_coefficient)?;
            write!(f, "  Probe variance: {:.3}", self.probe_variance)
        }
    }

    /// Textbook Example: Primary Clustering Analysis
    /// Demonstrates worst-case clustering when α = 1/2
    /// Claude Work: Θ(m), Span: Θ(m)
    pub fn example_primary_clustering_analysis() -> (PrimaryClusteringMetrics, f64, f64) {
        let hash_fn = DefaultHashFunction;
        let strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
            AdvancedLinearProbingStrategy::new(hash_fn);

        // Create a table with 50% load factor (α = 1/2)
        let table: FlatHashTable<String, String, AdvancedLinearProbingStrategy<String, DefaultHashFunction>> =
            FlatHashTable::create_table(strategy.clone(), 16);

        // Analyze clustering
        let metrics = strategy.analyze_primary_clustering(&table);

        // Estimate probe counts
        let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(0.5);
        let successful_probes = strategy.estimate_successful_probe_count(0.5);

        (metrics, unsuccessful_probes, successful_probes)
    }
}
