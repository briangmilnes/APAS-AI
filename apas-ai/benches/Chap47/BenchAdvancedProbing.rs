//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark module for Advanced Probing Strategies
//! 
//! Benchmarks all three advanced probing strategies:
//! - Linear probing with primary clustering analysis
//! - Quadratic probing with secondary clustering avoidance
//! - Double hashing with dual hash functions
//! 
//! Includes clustering analysis performance impact measurements
//! 
//! Work: O(1) per operation for hash table operations
//! Span: O(1) per operation for hash table operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use apas_ai::Chap47::AdvancedLinearProbing::AdvancedLinearProbing::{
    AdvancedLinearProbingStrategy
};
use apas_ai::Chap47::AdvancedQuadraticProbing::AdvancedQuadraticProbing::{
    AdvancedQuadraticProbingStrategy
};
use apas_ai::Chap47::AdvancedDoubleHashing::AdvancedDoubleHashing::{
    AdvancedDoubleHashingStrategy
};
use apas_ai::Chap47::FlatHashTable::FlatHashTable::{
    FlatHashTable
};
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::{
    DefaultHashFunction
};
use apas_ai::Types::Types::{Pair, StT};

/// Benchmark linear probing insertion performance
fn bench_linear_probing_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("linear_probing_insert");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn);
    
    // Test different table sizes (small for fast benchmarks)
    for table_size in [17, 31].iter() {
        group.bench_with_input(
            BenchmarkId::new("table_size", table_size),
            table_size,
            |b, &size| {
                b.iter(|| {
                    let mut table = FlatHashTable::create_table(strategy.clone(), size);
                    
                    // Insert elements up to 50% load factor (faster)
                    let num_elements = (size as f64 * 0.5) as usize;
                    for i in 0..num_elements {
                        table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
                    }
                    
                    black_box(table)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark quadratic probing insertion performance
fn bench_quadratic_probing_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("quadratic_probing_insert");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new(hash_fn);
    
    // Test different table sizes (small primes for fast benchmarks)
    for table_size in [17, 31].iter() {
        group.bench_with_input(
            BenchmarkId::new("table_size", table_size),
            table_size,
            |b, &size| {
                b.iter(|| {
                    let mut table = FlatHashTable::create_table(strategy.clone(), size);
                    
                    // Insert elements up to 50% load factor (faster)
                    let num_elements = (size as f64 * 0.5) as usize;
                    for i in 0..num_elements {
                        table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
                    }
                    
                    black_box(table)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark double hashing insertion performance
fn bench_double_hashing_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("double_hashing_insert");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);
    
    // Test different table sizes (small primes for fast benchmarks)
    for table_size in [17, 31].iter() {
        group.bench_with_input(
            BenchmarkId::new("table_size", table_size),
            table_size,
            |b, &size| {
                b.iter(|| {
                    let mut table = FlatHashTable::create_table(strategy.clone(), size);
                    
                    // Insert elements up to 50% load factor (faster)
                    let num_elements = (size as f64 * 0.5) as usize;
                    for i in 0..num_elements {
                        table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
                    }
                    
                    black_box(table)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark lookup performance across all strategies
fn bench_lookup_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_performance");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    // Prepare tables with different strategies
    let hash_fn = DefaultHashFunction;
    let linear_strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn.clone());
    let quad_strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new(hash_fn.clone());
    let double_strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash_fn.clone(), hash_fn);
    
    let table_size = 31; // Small table for fast benchmarks
    let num_elements = (table_size as f64 * 0.5) as usize;
    
    // Pre-populate tables
    let mut linear_table = FlatHashTable::create_table(linear_strategy, table_size);
    let mut quad_table = FlatHashTable::create_table(quad_strategy, table_size);
    let mut double_table = FlatHashTable::create_table(double_strategy, table_size);
    
    for i in 0..num_elements {
        let key = i as i32;
        let value = format!("value_{}", i);
        linear_table = linear_table.insert(key, value.clone());
        quad_table = quad_table.insert(key, value.clone());
        double_table = double_table.insert(key, value);
    }
    
    // Benchmark lookups
    group.bench_function("linear_probing", |b| {
        b.iter(|| {
            for i in 0..10 { // Reduced iterations for faster benchmarks
                let key = black_box((i * 7) % num_elements as i32);
                black_box(linear_table.lookup(&key));
            }
        });
    });
    
    group.bench_function("quadratic_probing", |b| {
        b.iter(|| {
            for i in 0..10 { // Reduced iterations for faster benchmarks
                let key = black_box((i * 7) % num_elements as i32);
                black_box(quad_table.lookup(&key));
            }
        });
    });
    
    group.bench_function("double_hashing", |b| {
        b.iter(|| {
            for i in 0..10 { // Reduced iterations for faster benchmarks
                let key = black_box((i * 7) % num_elements as i32);
                black_box(double_table.lookup(&key));
            }
        });
    });
    
    group.finish();
}

/// Benchmark deletion performance across all strategies
fn bench_deletion_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("deletion_performance");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash_fn = DefaultHashFunction;
    let table_size = 17; // Small table for fast benchmarks
    let num_elements = (table_size as f64 * 0.5) as usize;
    
    // Benchmark linear probing deletion
    group.bench_function("linear_probing", |b| {
        b.iter_batched(
            || {
                let linear_strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn.clone());
                let mut table = FlatHashTable::create_table(linear_strategy, table_size);
                
                for i in 0..num_elements {
                    table = table.insert(i as i32, format!("value_{}", i));
                }
                
                table
            },
            |mut table| {
                // Delete every 3rd element
                for i in (0..num_elements).step_by(3) {
                    let (new_table, _) = table.delete(&(i as i32));
                    table = new_table;
                }
                black_box(table)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Benchmark quadratic probing deletion
    group.bench_function("quadratic_probing", |b| {
        b.iter_batched(
            || {
                let quad_strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new(hash_fn.clone());
                let mut table = FlatHashTable::create_table(quad_strategy, table_size);
                
                for i in 0..num_elements {
                    table = table.insert(i as i32, format!("value_{}", i));
                }
                
                table
            },
            |mut table| {
                // Delete every 3rd element
                for i in (0..num_elements).step_by(3) {
                    let (new_table, _) = table.delete(&(i as i32));
                    table = new_table;
                }
                black_box(table)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Benchmark double hashing deletion
    group.bench_function("double_hashing", |b| {
        b.iter_batched(
            || {
                let double_strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash_fn.clone(), hash_fn.clone());
                let mut table = FlatHashTable::create_table(double_strategy, table_size);
                
                for i in 0..num_elements {
                    table = table.insert(i as i32, format!("value_{}", i));
                }
                
                table
            },
            |mut table| {
                // Delete every 3rd element
                for i in (0..num_elements).step_by(3) {
                    let (new_table, _) = table.delete(&(i as i32));
                    table = new_table;
                }
                black_box(table)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

/// Benchmark clustering analysis performance impact
fn bench_clustering_analysis_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("clustering_analysis_impact");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash_fn = DefaultHashFunction;
    let table_size = 17; // Small table for fast benchmarks
    let num_elements = (table_size as f64 * 0.5) as usize;
    
    // Benchmark linear probing with clustering analysis enabled vs disabled
    group.bench_function("linear_with_clustering", |b| {
        b.iter(|| {
            let strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn.clone());
            let mut table = FlatHashTable::create_table(strategy.clone(), table_size);
            
            for i in 0..num_elements {
                table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
            }
            
            // Perform clustering analysis
            let _metrics = strategy.analyze_primary_clustering(&table);
            
            black_box(table)
        });
    });
    
    group.bench_function("linear_without_clustering", |b| {
        b.iter(|| {
            let strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new_without_clustering(hash_fn.clone());
            let mut table = FlatHashTable::create_table(strategy, table_size);
            
            for i in 0..num_elements {
                table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
            }
            
            black_box(table)
        });
    });
    
    // Benchmark quadratic probing with clustering analysis enabled vs disabled
    group.bench_function("quadratic_with_clustering", |b| {
        b.iter(|| {
            let strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new(hash_fn.clone());
            let mut table = FlatHashTable::create_table(strategy.clone(), table_size);
            
            for i in 0..num_elements {
                table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
            }
            
            // Perform clustering analysis
            let _metrics = strategy.analyze_secondary_clustering(&table);
            
            black_box(table)
        });
    });
    
    group.bench_function("quadratic_without_clustering", |b| {
        b.iter(|| {
            let strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new_minimal(hash_fn.clone(), 1, 1);
            let mut table = FlatHashTable::create_table(strategy, table_size);
            
            for i in 0..num_elements {
                table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
            }
            
            black_box(table)
        });
    });
    
    group.finish();
}

/// Benchmark load factor impact on performance
fn bench_load_factor_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_factor_impact");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash_fn = DefaultHashFunction;
    let table_size = 17; // Small table for fast benchmarks
    
    // Test different load factors (fewer for faster benchmarks)
    for load_factor in [0.3, 0.7].iter() {
        let num_elements = (table_size as f64 * load_factor) as usize;
        
        // Linear probing performance at different load factors
        group.bench_with_input(
            BenchmarkId::new("linear_probing", format!("{:.1}", load_factor)),
            &(load_factor, num_elements),
            |b, &(_, num_elems)| {
                b.iter_batched(
                    || {
                        let strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn.clone());
                        let mut table = FlatHashTable::create_table(strategy, table_size);
                        
                        for i in 0..num_elems {
                            table = table.insert(i as i32, format!("value_{}", i));
                        }
                        
                        table
                    },
                    |table| {
                        // Perform lookups
                        for i in 0..5 { // Reduced for faster benchmarks
                            let key = black_box((i * 13) % num_elems as i32);
                            black_box(table.lookup(&key));
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
        
        // Double hashing performance at different load factors
        group.bench_with_input(
            BenchmarkId::new("double_hashing", format!("{:.1}", load_factor)),
            &(load_factor, num_elements),
            |b, &(_, num_elems)| {
                b.iter_batched(
                    || {
                        let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash_fn.clone(), hash_fn.clone());
                        let mut table = FlatHashTable::create_table(strategy, table_size);
                        
                        for i in 0..num_elems {
                            table = table.insert(i as i32, format!("value_{}", i));
                        }
                        
                        table
                    },
                    |table| {
                        // Perform lookups
                        for i in 0..5 { // Reduced for faster benchmarks
                            let key = black_box((i * 13) % num_elems as i32);
                            black_box(table.lookup(&key));
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark resize operations
fn bench_resize_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("resize_performance");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);
    
    let hash_fn = DefaultHashFunction;
    let initial_size = 17; // Small sizes for fast benchmarks
    let new_size = 31;
    let num_elements = (initial_size as f64 * 0.6) as usize;
    
    // Benchmark linear probing resize
    group.bench_function("linear_probing_resize", |b| {
        b.iter_batched(
            || {
                let strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn.clone());
                let mut table = FlatHashTable::create_table(strategy, initial_size);
                
                for i in 0..num_elements {
                    table = table.insert(i as i32, format!("value_{}", i));
                }
                
                table
            },
            |table| {
                let resized_table = table.resize(new_size);
                black_box(resized_table)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Benchmark quadratic probing resize
    group.bench_function("quadratic_probing_resize", |b| {
        b.iter_batched(
            || {
                let strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new(hash_fn.clone());
                let mut table = FlatHashTable::create_table(strategy, initial_size);
                
                for i in 0..num_elements {
                    table = table.insert(i as i32, format!("value_{}", i));
                }
                
                table
            },
            |table| {
                let resized_table = table.resize(new_size);
                black_box(resized_table)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Benchmark double hashing resize
    group.bench_function("double_hashing_resize", |b| {
        b.iter_batched(
            || {
                let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash_fn.clone(), hash_fn.clone());
                let mut table = FlatHashTable::create_table(strategy, initial_size);
                
                for i in 0..num_elements {
                    table = table.insert(i as i32, format!("value_{}", i));
                }
                
                table
            },
            |table| {
                let resized_table = table.resize(new_size);
                black_box(resized_table)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_linear_probing_insert,
    bench_quadratic_probing_insert,
    bench_double_hashing_insert,
    bench_lookup_performance,
    bench_deletion_performance,
    bench_clustering_analysis_impact,
    bench_load_factor_impact,
    bench_resize_performance
);

criterion_main!(benches);
