//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Tables Benchmarks - Optimized for fast execution

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;
use apas_ai::Chap47::SeparateChaining::SeparateChaining::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::LinearProbing::LinearProbing::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;

fn bench_separate_chaining_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("separate_chaining_insert");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    for size in [10, 25, 50].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let mut table = SeparateChainingHashTable::create_table(
                    DefaultKeyEquality,
                    DefaultHashFunction,
                    16
                );
                
                for i in 0..size {
                    let key = i.to_string();
                    let value = (i * 10).to_string();
                    table = table.insert(black_box(key), black_box(value));
                }
                
                black_box(table)
            });
        });
    }
    
    group.finish();
}

fn bench_separate_chaining_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("separate_chaining_lookup");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Pre-populate table
    let mut table = SeparateChainingHashTable::create_table(
        DefaultKeyEquality,
        DefaultHashFunction,
        32
    );
    
    for i in 0..50 {
        let key = i.to_string();
        let value = (i * 10).to_string();
        table = table.insert(key, value);
    }
    
    group.bench_function("lookup_existing", |b| {
        b.iter(|| {
            for i in 0..25 {
                let key = i.to_string();
                black_box(table.lookup(&black_box(key)));
            }
        });
    });
    
    group.bench_function("lookup_missing", |b| {
        b.iter(|| {
            for i in 100..125 {
                let key = i.to_string();
                black_box(table.lookup(&black_box(key)));
            }
        });
    });
    
    group.finish();
}

fn bench_flat_hash_table_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_hash_table_insert");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    for size in [10, 25, 50].iter() {
        group.bench_with_input(BenchmarkId::new("linear_probing", size), size, |b, &size| {
            b.iter(|| {
                let probe_strategy = LinearProbingStrategy::new(DefaultHashFunction);
                let mut table = FlatHashTable::create_table(probe_strategy, 64);
                
                for i in 0..size {
                    let key = i.to_string();
                    let value = (i * 10).to_string();
                    table = table.insert(black_box(key), black_box(value));
                }
                
                black_box(table)
            });
        });
    }
    
    group.finish();
}

fn bench_flat_hash_table_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_hash_table_lookup");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Pre-populate table
    let probe_strategy = LinearProbingStrategy::new(DefaultHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 64);
    
    for i in 0..30 {
        let key = i.to_string();
        let value = (i * 10).to_string();
        table = table.insert(key, value);
    }
    
    group.bench_function("lookup_existing", |b| {
        b.iter(|| {
            for i in 0..15 {
                let key = i.to_string();
                black_box(table.lookup(&black_box(key)));
            }
        });
    });
    
    group.bench_function("lookup_missing", |b| {
        b.iter(|| {
            for i in 100..115 {
                let key = i.to_string();
                black_box(table.lookup(&black_box(key)));
            }
        });
    });
    
    group.finish();
}

fn bench_hash_table_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_table_comparison");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    let size = 25;
    
    group.bench_function("separate_chaining", |b| {
        b.iter(|| {
            let mut table = SeparateChainingHashTable::create_table(
                DefaultKeyEquality,
                DefaultHashFunction,
                16
            );
            
            for i in 0..size {
                let key = i.to_string();
                let value = (i * 10).to_string();
                table = table.insert(black_box(key), black_box(value));
            }
            
            // Perform some lookups
            for i in 0..10 {
                let key = i.to_string();
                black_box(table.lookup(&key));
            }
            
            black_box(table)
        });
    });
    
    group.bench_function("linear_probing", |b| {
        b.iter(|| {
            let probe_strategy = LinearProbingStrategy::new(DefaultHashFunction);
            let mut table = FlatHashTable::create_table(probe_strategy, 32);
            
            for i in 0..size {
                let key = i.to_string();
                let value = (i * 10).to_string();
                table = table.insert(black_box(key), black_box(value));
            }
            
            // Perform some lookups
            for i in 0..10 {
                let key = i.to_string();
                black_box(table.lookup(&key));
            }
            
            black_box(table)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_separate_chaining_insert,
    bench_separate_chaining_lookup,
    bench_flat_hash_table_insert,
    bench_flat_hash_table_lookup,
    bench_hash_table_comparison
);
criterion_main!(benches);