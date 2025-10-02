//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 42 single-threaded persistent table implementation.

use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap42::TableStPer::TableStPer::*;
use apas_ai::Types::Types::*;
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn build_table(size: usize) -> TableStPer<i32, String> {
    let mut table = TableStPer::empty();
    for i in 0..size {
        table = table.insert(i as i32, format!("value_{}", i), |_old, new| new.clone());
    }
    table
}

fn build_set(size: usize) -> ArraySetStEph<i32> {
    let mut set = ArraySetStEph::empty();
    for i in 0..size {
        set.insert(i as i32);
    }
    set
}

fn bench_table_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStPer Operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    for size in [10].iter() {
        // Benchmark insert
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |table| {
                    let new_key = (size + 1) as i32;
                    let new_value = format!("new_value_{}", size + 1);
                    black_box(table.insert(new_key, new_value, |_old, new| new.clone()))
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark find
        group.bench_with_input(BenchmarkId::new("find_existing", size), size, |b, &size| {
            let table = build_table(size);
            let key = (size / 2) as i32;
            b.iter(|| black_box(table.find(&key)));
        });

        group.bench_with_input(BenchmarkId::new("find_missing", size), size, |b, &size| {
            let table = build_table(size);
            let key = (size + 100) as i32;
            b.iter(|| black_box(table.find(&key)));
        });

        // Benchmark delete
        group.bench_with_input(BenchmarkId::new("delete", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |table| {
                    let key = (size / 2) as i32;
                    black_box(table.delete(&key))
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark domain
        group.bench_with_input(BenchmarkId::new("domain", size), size, |b, &size| {
            let table = build_table(size);
            b.iter(|| black_box(table.domain()));
        });

        // Benchmark map
        group.bench_with_input(BenchmarkId::new("map", size), size, |b, &size| {
            let table = build_table(size);
            b.iter(|| black_box(table.map(|s| s.to_uppercase())));
        });

        // Benchmark filter
        group.bench_with_input(BenchmarkId::new("filter", size), size, |b, &size| {
            let table = build_table(size);
            b.iter(|| black_box(table.filter(|k, _v| *k % 2 == 0)));
        });
    }

    group.finish();
}

fn bench_table_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStPer Bulk Operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    for size in [10].iter() {
        // Benchmark intersection
        group.bench_with_input(BenchmarkId::new("intersection", size), size, |b, &size| {
            let table1 = build_table(size);
            let table2 = build_table(size / 2);
            b.iter(|| black_box(table1.intersection(&table2, |v1, v2| format!("{}+{}", v1, v2))));
        });

        // Benchmark union
        group.bench_with_input(BenchmarkId::new("union", size), size, |b, &size| {
            let table1 = build_table(size);
            let table2 = build_table(size / 2);
            b.iter(|| black_box(table1.union(&table2, |v1, v2| format!("{}+{}", v1, v2))));
        });

        // Benchmark difference
        group.bench_with_input(BenchmarkId::new("difference", size), size, |b, &size| {
            let table1 = build_table(size);
            let table2 = build_table(size / 2);
            b.iter(|| black_box(table1.difference(&table2)));
        });

        // Benchmark restrict
        group.bench_with_input(BenchmarkId::new("restrict", size), size, |b, &size| {
            let table = build_table(size);
            let keys = build_set(size / 2);
            b.iter(|| black_box(table.restrict(&keys)));
        });

        // Benchmark subtract
        group.bench_with_input(BenchmarkId::new("subtract", size), size, |b, &size| {
            let table = build_table(size);
            let keys = build_set(size / 2);
            b.iter(|| black_box(table.subtract(&keys)));
        });

        // Benchmark tabulate
        group.bench_with_input(BenchmarkId::new("tabulate", size), size, |b, &size| {
            let keys = build_set(size);
            b.iter(|| black_box(TableStPer::tabulate(|k| format!("computed_{}", k * k), &keys)));
        });
    }

    group.finish();
}

fn bench_table_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStPer Construction");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    for size in [10].iter() {
        // Benchmark sequential construction
        group.bench_with_input(BenchmarkId::new("sequential_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut table = TableStPer::empty();
                for i in 0..size {
                    table = table.insert(i as i32, format!("value_{}", i), |_old, new| new.clone());
                }
                black_box(table)
            });
        });

        // Benchmark singleton creation
        group.bench_with_input(BenchmarkId::new("singleton", size), size, |b, &_size| {
            b.iter(|| black_box(TableStPer::singleton(42, "test_value".to_string())));
        });
    }

    group.finish();
}

fn bench_table_persistence(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStPer Persistence");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    for size in [20].iter() {
        // Benchmark that persistent operations don't affect original
        group.bench_with_input(BenchmarkId::new("persistent_insert", size), size, |b, &size| {
            let original = build_table(size);
            b.iter(|| {
                let modified = original.insert(9999, "new_value".to_string(), |_old, new| new.clone());
                // Access both to ensure they're both valid
                black_box((original.size(), modified.size()))
            });
        });

        group.bench_with_input(BenchmarkId::new("persistent_delete", size), size, |b, &size| {
            let original = build_table(size);
            b.iter(|| {
                let modified = original.delete(&((size / 2) as i32));
                // Access both to ensure they're both valid
                black_box((original.size(), modified.size()))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_table_operations,
    bench_table_bulk_operations,
    bench_table_construction,
    bench_table_persistence
);
criterion_main!(benches);
