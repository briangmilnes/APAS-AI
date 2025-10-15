//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for Vec-based Chained Hash Table

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use apas_ai::Types::Types::*;
use apas_ai::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_ai::Chap47clean::VecChainedHashTable::VecChainedHashTable::*;

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("VecChained_Insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    group.sample_size(30);
    
    for size in [500, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, s| {
            let size = *s;
            b.iter(|| {
                let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 100);
                let mut table = <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(hash_fn, 100);
                for _ in 0..100 {
                    table.table.push(Vec::new());
                }
                for i in 0..size {
                    VecChainedHashTableStEph::insert(&mut table, black_box(i), black_box(format!("value{i}")));
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("VecChained_Lookup");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    group.sample_size(30);
    
    for size in [500, 1000] {
        let hash_fn: HashFun<i32> = Box::new(|k| (*k as N) % 100);
        let mut table = <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(hash_fn, 100);
        for _ in 0..100 {
            table.table.push(Vec::new());
        }
        for i in 0..size {
            VecChainedHashTableStEph::insert(&mut table, i, format!("value{i}"));
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, s| {
            let size = *s;
            b.iter(|| {
                for i in 0..size {
                    black_box(VecChainedHashTableStEph::lookup(&table, black_box(&{ i })));
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insert, bench_lookup);
criterion_main!(benches);
