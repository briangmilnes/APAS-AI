//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Tables Benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use apas_ai::Chap47::SeparateChaining::SeparateChaining::*;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::LinearProbing::LinearProbing::*;
use apas_ai::Chap47::NestedHashTable::NestedHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;

fn bench_separate_chaining_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("separate_chaining_insert");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let mut table = SeparateChainingHashTable::create_table(
                    DefaultKeyEquality,
                    DefaultHashFunction,
                    16
                );
                
                for i in 0..size {
                    let key = format!("key_{}", i);
                    let value = format!("value_{}", i);
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
    
    // Pre-populate table
    let mut table = SeparateChainingHashTable::create_table(
        DefaultKeyEquality,
        DefaultHashFunction,
        64
    );
    
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        table = table.insert(key, value);
    }
    
    group.bench_function("lookup_existing", |b| {
        b.iter(|| {
            for i in 0..100 {
                let key = format!("key_{}", i);
                black_box(table.lookup(&black_box(key)));
            }
        });
    });
    
    group.bench_function("lookup_missing", |b| {
        b.iter(|| {
            for i in 1000..1100 {
                let key = format!("key_{}", i);
                black_box(table.lookup(&black_box(key)));
            }
        });
    });
    
    group.finish();
}

fn bench_flat_hash_table_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_hash_table_insert");
    
    for size in [100, 1000, 5000].iter() { // Smaller sizes for flat tables due to load factor limits
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let probe_strategy = LinearProbingStrategy::new(DefaultHashFunction);
                let mut table = FlatHashTable::create_table(probe_strategy, 32);
                
                for i in 0..size {
                    let key = format!("key_{}", i);
                    let value = format!("value_{}", i);
                    table = table.insert(black_box(key), black_box(value));
                }
                
                black_box(table)
            });
        });
    }
    
    group.finish();
}

fn bench_nested_hash_table_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_hash_table_insert");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let mut table: NestedHashTable<String, String> = NestedHashTable::create_table(16);
                
                for i in 0..size {
                    let key = format!("key_{}", i);
                    let value = format!("value_{}", i);
                    table = table.insert(black_box(key), black_box(value));
                }
                
                black_box(table)
            });
        });
    }
    
    group.finish();
}

fn bench_hash_table_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_table_comparison");
    
    let size = 1000;
    
    // Separate Chaining
    group.bench_function("separate_chaining", |b| {
        b.iter(|| {
            let mut table = SeparateChainingHashTable::create_table(
                DefaultKeyEquality,
                DefaultHashFunction,
                32
            );
            
            for i in 0..size {
                let key = format!("key_{}", i);
                let value = i;
                table = table.insert(black_box(key), black_box(value));
            }
            
            // Lookup test
            for i in 0..100 {
                let key = format!("key_{}", i);
                black_box(table.lookup(&black_box(key)));
            }
            
            black_box(table)
        });
    });
    
    // Nested Hash Table
    group.bench_function("nested_hash_table", |b| {
        b.iter(|| {
            let mut table: NestedHashTable<String, i32> = NestedHashTable::create_table(32);
            
            for i in 0..size {
                let key = format!("key_{}", i);
                let value = i;
                table = table.insert(black_box(key), black_box(value));
            }
            
            // Lookup test
            for i in 0..100 {
                let key = format!("key_{}", i);
                black_box(table.lookup(&black_box(key)));
            }
            
            black_box(table)
        });
    });
    
    // Flat Hash Table (smaller size due to load factor)
    group.bench_function("flat_hash_table", |b| {
        b.iter(|| {
            let probe_strategy = LinearProbingStrategy::new(DefaultHashFunction);
            let mut table = FlatHashTable::create_table(probe_strategy, 64);
            
            for i in 0..500 { // Smaller size to avoid excessive resizing
                let key = format!("key_{}", i);
                let value = i;
                table = table.insert(black_box(key), black_box(value));
            }
            
            // Lookup test
            for i in 0..100 {
                let key = format!("key_{}", i);
                black_box(table.lookup(&black_box(key)));
            }
            
            black_box(table)
        });
    });
    
    group.finish();
}

fn bench_hash_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_functions");
    
    let keys: Vec<String> = (0..1000).map(|i| format!("test_key_{}", i)).collect();
    let table_size = 1024;
    
    group.bench_function("string_position_hash", |b| {
        let hash_fn = StringPositionHashFunction;
        b.iter(|| {
            for key in &keys {
                black_box(hash_fn.hash(black_box(key), table_size));
            }
        });
    });
    
    group.bench_function("polynomial_hash", |b| {
        let hash_fn = PolynomialHashFunction::new(31);
        b.iter(|| {
            for key in &keys {
                black_box(hash_fn.hash(black_box(key), table_size));
            }
        });
    });
    
    group.bench_function("default_hash", |b| {
        let hash_fn = DefaultHashFunction;
        b.iter(|| {
            for key in &keys {
                black_box(hash_fn.hash(black_box(key), table_size));
            }
        });
    });
    
    group.finish();
}

fn bench_collision_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("collision_scenarios");
    
    // High collision scenario - keys that hash to same values
    group.bench_function("high_collision_separate_chaining", |b| {
        b.iter(|| {
            let mut table = SeparateChainingHashTable::create_table(
                DefaultKeyEquality,
                StringPositionHashFunction,
                5 // Small table to force collisions
            );
            
            // Insert keys that will collide
            let colliding_keys = vec!["aa", "ff", "kk", "pp", "uu"]; // All hash to 0 with StringPositionHashFunction
            for (i, key) in colliding_keys.iter().enumerate() {
                table = table.insert(black_box(key.to_string()), black_box(i));
            }
            
            // Lookup all keys
            for key in &colliding_keys {
                black_box(table.lookup(&black_box(key.to_string())));
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
    bench_nested_hash_table_insert,
    bench_hash_table_comparison,
    bench_hash_functions,
    bench_collision_scenarios
);

criterion_main!(benches);
