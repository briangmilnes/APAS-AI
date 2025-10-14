//! Benchmark for Linear Probing Flat Hash Table

use apas_ai::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_ai::Chap47clean::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47clean::LinProbFlatHashTable::LinProbFlatHashTable::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinProb_Insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    group.sample_size(30);
    
    for size in [500, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, s| {
            let size = *s;
            b.iter(|| {
                let table_size = size * 2;
                let hash_fn: HashFun<i32> = Box::new(move |k| (*k as N) % table_size);
                let mut table = <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(hash_fn, table_size);
                for _ in 0..table_size {
                    table.table.push(FlatEntry::Empty);
                }
                for i in 0..size {
                    LinProbFlatHashTableStEph::insert(&mut table, black_box(i as i32), black_box(format!("value{}", i)));
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinProb_Lookup");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    group.sample_size(30);
    
    for size in [500, 1000] {
        let table_size = size * 2;
        let hash_fn: HashFun<i32> = Box::new(move |k| (*k as N) % table_size);
        let mut table = <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(hash_fn, table_size);
        for _ in 0..table_size {
            table.table.push(FlatEntry::Empty);
        }
        for i in 0..size {
            LinProbFlatHashTableStEph::insert(&mut table, i as i32, format!("value{}", i));
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, s| {
            let size = *s;
            b.iter(|| {
                for i in 0..size {
                    black_box(LinProbFlatHashTableStEph::lookup(&table, black_box(&(i as i32))));
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insert, bench_lookup);
criterion_main!(benches);
