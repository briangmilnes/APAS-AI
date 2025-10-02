//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Clustering Analysis for Hash Table Probing Strategies
//! Comprehensive analysis of primary and secondary clustering phenomena

pub mod ClusteringAnalysis {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display};

    use crate::Chap47::AdvancedDoubleHashing::AdvancedDoubleHashing::*;
    use crate::Chap47::AdvancedLinearProbing::AdvancedLinearProbing::*;
    use crate::Chap47::AdvancedQuadraticProbing::AdvancedQuadraticProbing::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct ComprehensiveClusteringAnalysis {
        pub strategy_name: String,
        pub table_size: N,
        pub load_factor: f64,
        pub primary_clustering: Option<PrimaryClusteringMetrics>,
        pub secondary_clustering: Option<SecondaryClusteringMetrics>,
        pub double_hashing_quality: Option<DoubleHashingMetrics>,
        pub overall_clustering_score: f64,
        pub performance_impact: ClusteringPerformanceImpact,
    }

    /// Performance Impact Assessment
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClusteringPerformanceImpact {
        pub expected_successful_probes: f64,
        pub expected_unsuccessful_probes: f64,
        pub clustering_severity: ClusteringSeverity,
        pub recommended_action: String,
    }

    /// Clustering Severity Classification
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug, PartialEq)]
    pub enum ClusteringSeverity {
        Minimal,  // < 10% performance degradation
        Moderate, // 10-30% performance degradation
        Severe,   // 30-50% performance degradation
        Critical, // > 50% performance degradation
    }

    /// Clustering Comparison Results
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug)]
    pub struct ClusteringComparison {
        pub linear_probing: ComprehensiveClusteringAnalysis,
        pub quadratic_probing: ComprehensiveClusteringAnalysis,
        pub double_hashing: ComprehensiveClusteringAnalysis,
        pub best_strategy: String,
        pub worst_strategy: String,
        pub performance_ranking: Vec<(String, f64)>,
    }

    /// Clustering Analysis Engine
    /// Claude Work: Θ(1), Span: Θ(1)
    #[derive(Clone, Debug)]
    pub struct ClusteringAnalyzer {
        pub analysis_enabled: B,
        pub detailed_metrics: B,
    }

    impl ClusteringAnalyzer {
        /// Create new clustering analyzer
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new() -> Self {
            ClusteringAnalyzer {
                analysis_enabled: true,
                detailed_metrics: true,
            }
        }

        /// Create minimal analyzer for performance testing
        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn new_minimal() -> Self {
            ClusteringAnalyzer {
                analysis_enabled: false,
                detailed_metrics: false,
            }
        }

        /// Analyze linear probing clustering
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn analyze_linear_probing<K: StT, H: HashFunClone<K>>(
            &self,
            strategy: &AdvancedLinearProbingStrategy<K, H>,
            table: &FlatHashTable<K, String, AdvancedLinearProbingStrategy<K, H>>,
        ) -> ComprehensiveClusteringAnalysis
        where
            AdvancedLinearProbingStrategy<K, H>: ProbeSequence<K> + Clone,
        {
            if !self.analysis_enabled {
                return self.create_empty_analysis("LinearProbing".to_string(), 0, 0.0);
            }

            let (load, size) = table.load_and_size();
            let load_factor = if size > 0 { load as f64 / size as f64 } else { 0.0 };

            let primary_clustering = if self.detailed_metrics {
                Some(strategy.analyze_primary_clustering(table))
            } else {
                None
            };

            let successful_probes = strategy.estimate_successful_probe_count(load_factor);
            let unsuccessful_probes = strategy.estimate_unsuccessful_probe_count(load_factor);

            let clustering_score = self.calculate_linear_clustering_score(&primary_clustering, load_factor);
            let performance_impact =
                self.assess_performance_impact(successful_probes, unsuccessful_probes, clustering_score);

            ComprehensiveClusteringAnalysis {
                strategy_name: "AdvancedLinearProbing".to_string(),
                table_size: size,
                load_factor,
                primary_clustering,
                secondary_clustering: None,
                double_hashing_quality: None,
                overall_clustering_score: clustering_score,
                performance_impact,
            }
        }

        /// Analyze quadratic probing clustering
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn analyze_quadratic_probing<K: StT, H: HashFunClone<K>>(
            &self,
            strategy: &AdvancedQuadraticProbingStrategy<K, H>,
            table: &FlatHashTable<K, String, AdvancedQuadraticProbingStrategy<K, H>>,
        ) -> ComprehensiveClusteringAnalysis
        where
            AdvancedQuadraticProbingStrategy<K, H>: ProbeSequence<K> + Clone,
        {
            if !self.analysis_enabled {
                return self.create_empty_analysis("QuadraticProbing".to_string(), 0, 0.0);
            }

            let (load, size) = table.load_and_size();
            let load_factor = if size > 0 { load as f64 / size as f64 } else { 0.0 };

            let secondary_clustering = if self.detailed_metrics {
                Some(strategy.analyze_secondary_clustering(table))
            } else {
                None
            };

            let successful_probes = strategy.estimate_probe_count(load_factor, true);
            let unsuccessful_probes = strategy.estimate_probe_count(load_factor, false);

            let clustering_score = self.calculate_quadratic_clustering_score(&secondary_clustering, load_factor);
            let performance_impact =
                self.assess_performance_impact(successful_probes, unsuccessful_probes, clustering_score);

            ComprehensiveClusteringAnalysis {
                strategy_name: "AdvancedQuadraticProbing".to_string(),
                table_size: size,
                load_factor,
                primary_clustering: None,
                secondary_clustering,
                double_hashing_quality: None,
                overall_clustering_score: clustering_score,
                performance_impact,
            }
        }

        /// Analyze double hashing quality
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn analyze_double_hashing<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>>(
            &self,
            strategy: &AdvancedDoubleHashingStrategy<K, H1, H2>,
            key: &K,
            table_size: N,
            load_factor: f64,
        ) -> ComprehensiveClusteringAnalysis {
            if !self.analysis_enabled {
                return self.create_empty_analysis("DoubleHashing".to_string(), table_size, load_factor);
            }

            let double_hashing_quality = if self.detailed_metrics {
                Some(strategy.analyze_double_hashing_quality(key, table_size))
            } else {
                None
            };

            let successful_probes = strategy.estimate_probe_count(load_factor, true);
            let unsuccessful_probes = strategy.estimate_probe_count(load_factor, false);

            let clustering_score = self.calculate_double_hashing_score(&double_hashing_quality, load_factor);
            let performance_impact =
                self.assess_performance_impact(successful_probes, unsuccessful_probes, clustering_score);

            ComprehensiveClusteringAnalysis {
                strategy_name: "AdvancedDoubleHashing".to_string(),
                table_size,
                load_factor,
                primary_clustering: None,
                secondary_clustering: None,
                double_hashing_quality,
                overall_clustering_score: clustering_score,
                performance_impact,
            }
        }

        /// Compare all three probing strategies
        /// Claude Work: Θ(m), Span: Θ(m) where m is table size
        pub fn compare_probing_strategies(&self, table_size: N, load_factor: f64) -> ClusteringComparison {
            // Create test key for analysis
            let test_key = "clustering_test_key".to_string();

            // Analyze linear probing
            let linear_strategy: AdvancedLinearProbingStrategy<String, DefaultHashFunction> =
                AdvancedLinearProbingStrategy::new(DefaultHashFunction);
            let linear_table: FlatHashTable<
                String,
                String,
                AdvancedLinearProbingStrategy<String, DefaultHashFunction>,
            > = FlatHashTable::create_table(linear_strategy.clone(), table_size);
            let linear_analysis = self.analyze_linear_probing(&linear_strategy, &linear_table);

            // Analyze quadratic probing
            let quadratic_strategy: AdvancedQuadraticProbingStrategy<String, DefaultHashFunction> =
                AdvancedQuadraticProbingStrategy::new(DefaultHashFunction);
            let quadratic_table: FlatHashTable<
                String,
                String,
                AdvancedQuadraticProbingStrategy<String, DefaultHashFunction>,
            > = FlatHashTable::create_table(quadratic_strategy.clone(), table_size);
            let quadratic_analysis = self.analyze_quadratic_probing(&quadratic_strategy, &quadratic_table);

            // Analyze double hashing
            let double_strategy: AdvancedDoubleHashingStrategy<String, DefaultHashFunction, DefaultHashFunction> =
                AdvancedDoubleHashingStrategy::new(DefaultHashFunction, DefaultHashFunction);
            let double_analysis = self.analyze_double_hashing(&double_strategy, &test_key, table_size, load_factor);

            // Rank strategies by performance
            let mut performance_ranking = vec![
                ("LinearProbing".to_string(), linear_analysis.overall_clustering_score),
                (
                    "QuadraticProbing".to_string(),
                    quadratic_analysis.overall_clustering_score,
                ),
                ("DoubleHashing".to_string(), double_analysis.overall_clustering_score),
            ];

            // Sort by clustering score (lower is better)
            performance_ranking.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            let best_strategy = performance_ranking[0].0.clone();
            let worst_strategy = performance_ranking[2].0.clone();

            ClusteringComparison {
                linear_probing: linear_analysis,
                quadratic_probing: quadratic_analysis,
                double_hashing: double_analysis,
                best_strategy,
                worst_strategy,
                performance_ranking,
            }
        }

        /// Calculate clustering score for linear probing
        /// Claude Work: Θ(1), Span: Θ(1)
        fn calculate_linear_clustering_score(
            &self,
            metrics: &Option<PrimaryClusteringMetrics>,
            load_factor: f64,
        ) -> f64 {
            if let Some(m) = metrics {
                // Higher clustering coefficient and variance indicate worse clustering
                let base_score = load_factor * 100.0; // Base score from load factor
                let clustering_penalty = m.clustering_coefficient * 50.0;
                let variance_penalty = m.probe_variance.sqrt() * 10.0;
                base_score + clustering_penalty + variance_penalty
            } else {
                load_factor * 100.0 // Simple load factor based score
            }
        }

        /// Calculate clustering score for quadratic probing
        /// Claude Work: Θ(1), Span: Θ(1)
        fn calculate_quadratic_clustering_score(
            &self,
            metrics: &Option<SecondaryClusteringMetrics>,
            load_factor: f64,
        ) -> f64 {
            if let Some(m) = metrics {
                let base_score = load_factor * 80.0; // Quadratic probing generally better than linear
                let secondary_penalty = m.secondary_clustering_coefficient * 30.0;
                let prime_bonus = if m.table_size_is_prime { -10.0 } else { 20.0 };
                let diversity_bonus = (1.0 - m.probe_sequence_diversity) * 15.0;
                base_score + secondary_penalty + prime_bonus + diversity_bonus
            } else {
                load_factor * 80.0
            }
        }

        /// Calculate clustering score for double hashing
        /// Claude Work: Θ(1), Span: Θ(1)
        fn calculate_double_hashing_score(&self, metrics: &Option<DoubleHashingMetrics>, load_factor: f64) -> f64 {
            if let Some(m) = metrics {
                let base_score = load_factor * 60.0; // Double hashing generally best
                let independence_bonus = m.hash_function_independence * -20.0;
                let prime_bonus = if m.relative_prime_validation { -15.0 } else { 25.0 };
                let collision_bonus = m.collision_avoidance_score * -10.0;
                base_score + independence_bonus + prime_bonus + collision_bonus
            } else {
                load_factor * 60.0
            }
        }

        /// Assess performance impact based on probe counts and clustering score
        /// Claude Work: Θ(1), Span: Θ(1)
        fn assess_performance_impact(
            &self,
            successful_probes: f64,
            unsuccessful_probes: f64,
            clustering_score: f64,
        ) -> ClusteringPerformanceImpact {
            let severity = if clustering_score < 50.0 {
                ClusteringSeverity::Minimal
            } else if clustering_score < 100.0 {
                ClusteringSeverity::Moderate
            } else if clustering_score < 150.0 {
                ClusteringSeverity::Severe
            } else {
                ClusteringSeverity::Critical
            };

            let recommended_action = match severity {
                | ClusteringSeverity::Minimal => "Current configuration is optimal".to_string(),
                | ClusteringSeverity::Moderate => {
                    "Consider reducing load factor or switching to quadratic probing".to_string()
                }
                | ClusteringSeverity::Severe => "Switch to double hashing or resize table".to_string(),
                | ClusteringSeverity::Critical => {
                    "Immediate action required: resize table and use double hashing".to_string()
                }
            };

            ClusteringPerformanceImpact {
                expected_successful_probes: successful_probes,
                expected_unsuccessful_probes: unsuccessful_probes,
                clustering_severity: severity,
                recommended_action,
            }
        }

        /// Create empty analysis for disabled analyzer
        /// Claude Work: Θ(1), Span: Θ(1)
        fn create_empty_analysis(
            &self,
            strategy_name: String,
            table_size: N,
            load_factor: f64,
        ) -> ComprehensiveClusteringAnalysis {
            ComprehensiveClusteringAnalysis {
                strategy_name,
                table_size,
                load_factor,
                primary_clustering: None,
                secondary_clustering: None,
                double_hashing_quality: None,
                overall_clustering_score: 0.0,
                performance_impact: ClusteringPerformanceImpact {
                    expected_successful_probes: 1.0,
                    expected_unsuccessful_probes: 1.0,
                    clustering_severity: ClusteringSeverity::Minimal,
                    recommended_action: "Analysis disabled".to_string(),
                },
            }
        }
    }

    impl Display for ComprehensiveClusteringAnalysis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "=== Comprehensive Clustering Analysis ===")?;
            writeln!(f, "Strategy: {}", self.strategy_name)?;
            writeln!(
                f,
                "Table size: {}, Load factor: {:.3}",
                self.table_size, self.load_factor
            )?;
            writeln!(f, "Overall clustering score: {:.2}", self.overall_clustering_score)?;
            writeln!(f)?;

            if let Some(ref primary) = self.primary_clustering {
                writeln!(f, "{}", primary)?;
                writeln!(f)?;
            }

            if let Some(ref secondary) = self.secondary_clustering {
                writeln!(f, "{}", secondary)?;
                writeln!(f)?;
            }

            if let Some(ref double) = self.double_hashing_quality {
                writeln!(f, "{}", double)?;
                writeln!(f)?;
            }

            write!(f, "{}", self.performance_impact)
        }
    }

    impl Display for ClusteringPerformanceImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Performance Impact Assessment:")?;
            writeln!(
                f,
                "  Expected successful probes: {:.2}",
                self.expected_successful_probes
            )?;
            writeln!(
                f,
                "  Expected unsuccessful probes: {:.2}",
                self.expected_unsuccessful_probes
            )?;
            writeln!(f, "  Clustering severity: {:?}", self.clustering_severity)?;
            write!(f, "  Recommended action: {}", self.recommended_action)
        }
    }

    impl Display for ClusteringComparison {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "=== Probing Strategy Comparison ===")?;
            writeln!(f, "Best strategy: {}", self.best_strategy)?;
            writeln!(f, "Worst strategy: {}", self.worst_strategy)?;
            writeln!(f)?;
            writeln!(f, "Performance ranking:")?;
            for (i, (strategy, score)) in self.performance_ranking.iter().enumerate() {
                writeln!(f, "  {}. {} (score: {:.2})", i + 1, strategy, score)?;
            }
            writeln!(f)?;
            writeln!(f, "--- Linear Probing Analysis ---")?;
            writeln!(f, "{}", self.linear_probing)?;
            writeln!(f)?;
            writeln!(f, "--- Quadratic Probing Analysis ---")?;
            writeln!(f, "{}", self.quadratic_probing)?;
            writeln!(f)?;
            writeln!(f, "--- Double Hashing Analysis ---")?;
            write!(f, "{}", self.double_hashing)
        }
    }

    /// Textbook Example: Comprehensive Clustering Analysis
    /// Demonstrates clustering analysis across all three probing strategies
    /// Claude Work: Θ(m), Span: Θ(m)
    pub fn example_comprehensive_clustering_analysis() -> ClusteringComparison {
        let analyzer = ClusteringAnalyzer::new();

        // Analyze with moderate load factor and prime table size
        analyzer.compare_probing_strategies(17, 0.6)
    }

    /// Example: Load Factor Impact Analysis
    /// Shows how clustering changes with different load factors
    /// Claude Work: Θ(k * m), Span: Θ(k * m) where k is number of load factors tested
    pub fn example_load_factor_impact_analysis() -> Vec<(f64, ClusteringComparison)> {
        let analyzer = ClusteringAnalyzer::new();
        let load_factors = vec![0.25, 0.5, 0.75, 0.9];
        let table_size = 17; // Prime table size

        load_factors
            .into_iter()
            .map(|lf| (lf, analyzer.compare_probing_strategies(table_size, lf)))
            .collect()
    }
}
