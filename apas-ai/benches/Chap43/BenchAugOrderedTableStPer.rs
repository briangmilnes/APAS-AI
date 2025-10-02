//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for single-threaded persistent reducer-augmented ordered table implementation.

use apas_ai::AugOrderedTableStPerLit;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::*;
use apas_ai::Chap43::OrderedTableStPer::OrderedTableStPer::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_reduce_val_vs_naive(c: &mut Criterion) {
    let mut group = c.benchmark_group("reduce_val_comparison");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;

    for size in [100, 500, 1000, 2000].iter() {
        // Create augmented table
        let mut aug_table = AugOrderedTableStPer::empty(sum_reducer, 0);
        for i in 1..=*size {
            aug_table = aug_table.insert(i, i * 10);
        }

        // Benchmark O(1) augmented reduceVal
        group.bench_with_input(BenchmarkId::new("augmented_reduce_val", size), size, |b, _| {
            b.iter(|| black_box(aug_table.reduce_val()))
        });

        // Benchmark O(n) naive reduction on augmented table for comparison
        group.bench_with_input(BenchmarkId::new("naive_reduce_on_augmented", size), size, |b, _| {
            b.iter(|| {
                let pairs = aug_table.collect();
                let mut sum = 0;
                for i in 0..pairs.length() {
                    let pair = pairs.nth(i);
                    sum += pair.1;
                }
                black_box(sum)
            })
        });
    }

    group.finish();
}

fn bench_range_reduction_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("range_reduction");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    // Create large dataset
    for i in 1..=5000 {
        table = table.insert(i, i * 2);
    }

    let range_sizes = [100, 500, 1000, 2000];

    for range_size in range_sizes.iter() {
        let start_key = 1000;
        let end_key = start_key + range_size;

        group.bench_with_input(BenchmarkId::new("reduce_range", range_size), range_size, |b, _| {
            b.iter(|| black_box(table.reduce_range(&start_key, &end_key)))
        });
    }

    group.finish();
}

fn bench_tramlaw_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("tramlaw_sales_analysis");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut sales_table = AugOrderedTableStPer::empty(sum_reducer, 0);

    // Simulate a full day of sales data (1440 minutes)
    for minute in 0..1440 {
        let base_sales = 1000;
        let time_factor = if minute >= 540 && minute <= 1200 { 2 } else { 1 }; // Business hours
        let sales = base_sales * time_factor + (minute % 100);
        sales_table = sales_table.insert(minute, sales);
    }

    group.bench_function("total_daily_sales", |b| b.iter(|| black_box(sales_table.reduce_val())));

    group.bench_function("business_hours_sales", |b| {
        b.iter(|| {
            black_box(sales_table.reduce_range(&540, &1200)) // 9am to 8pm
        })
    });

    group.bench_function("peak_hours_sales", |b| {
        b.iter(|| {
            black_box(sales_table.reduce_range(&720, &840)) // 12pm to 2pm
        })
    });

    group.bench_function("multiple_range_queries", |b| {
        b.iter(|| {
            let morning = sales_table.reduce_range(&480, &720); // 8am-12pm
            let afternoon = sales_table.reduce_range(&720, &960); // 12pm-4pm
            let evening = sales_table.reduce_range(&960, &1200); // 4pm-8pm
            black_box((morning, afternoon, evening))
        })
    });

    group.finish();
}

fn bench_insert_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;

    for initial_size in [0, 100, 500, 1000].iter() {
        let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

        // Pre-populate table
        for i in 1..=*initial_size {
            table = table.insert(i, i * 10);
        }

        group.bench_with_input(
            BenchmarkId::new("insert_with_reduction_update", initial_size),
            initial_size,
            |b, _| {
                b.iter_batched(
                    || (table.clone(), *initial_size + 1),
                    |(local_table, counter)| {
                        let new_table = local_table.insert(counter, counter * 10);
                        black_box(new_table)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn bench_different_reducers(c: &mut Criterion) {
    let mut group = c.benchmark_group("reducer_types");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let size = 1000;

    // Sum reducer
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut sum_table = AugOrderedTableStPer::empty(sum_reducer, 0);
    for i in 1..=size {
        sum_table = sum_table.insert(i, i);
    }

    // Max reducer
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut max_table = AugOrderedTableStPer::empty(max_reducer, i32::MIN);
    for i in 1..=size {
        max_table = max_table.insert(i, i);
    }

    // Min reducer
    let min_reducer = |a: &i32, b: &i32| if a < b { *a } else { *b };
    let mut min_table = AugOrderedTableStPer::empty(min_reducer, i32::MAX);
    for i in 1..=size {
        min_table = min_table.insert(i, i);
    }

    group.bench_function("sum_reducer", |b| b.iter(|| black_box(sum_table.reduce_val())));

    group.bench_function("max_reducer", |b| b.iter(|| black_box(max_table.reduce_val())));

    group.bench_function("min_reducer", |b| b.iter(|| black_box(min_table.reduce_val())));

    group.finish();
}

fn bench_split_join_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("split_join_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    // Create test data
    for i in 1..=1000 {
        table = table.insert(i, i * 10);
    }

    group.bench_function("split_key", |b| {
        b.iter(|| {
            let (left, middle, right) = table.split_key(&500);
            black_box((left.reduce_val(), middle, right.reduce_val()))
        })
    });

    group.bench_function("join_key", |b| {
        b.iter(|| {
            let (left, _middle, right) = table.split_key(&500);
            let rejoined = AugOrderedTableStPer::join_key(&left, &right);
            black_box(rejoined.reduce_val())
        })
    });

    group.finish();
}

fn bench_macro_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction_methods");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("macro_construction_small", |b| {
        b.iter(|| {
            let table: AugOrderedTableStPer<i32, i32, _> = AugOrderedTableStPerLit![
                reducer: sum_reducer, identity: 0,
                1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50
            ];
            black_box(table.reduce_val())
        })
    });

    group.bench_function("iterative_construction_small", |b| {
        b.iter(|| {
            let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);
            table = table.insert(1, 10);
            table = table.insert(2, 20);
            table = table.insert(3, 30);
            table = table.insert(4, 40);
            table = table.insert(5, 50);
            black_box(table.reduce_val())
        })
    });

    group.finish();
}

fn bench_memory_usage_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let sum_reducer = |a: &i32, b: &i32| a + b;

    // Test persistent semantics - multiple versions
    group.bench_function("persistent_versions", |b| {
        b.iter(|| {
            let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);
            let mut versions = Vec::new();

            for i in 1..=100 {
                table = table.insert(i, i * 10);
                if i % 10 == 0 {
                    versions.push(table.clone());
                }
            }

            // Access all versions
            let mut total = 0;
            for version in &versions {
                total += version.reduce_val();
            }
            black_box(total)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_reduce_val_vs_naive,
    bench_range_reduction_performance,
    bench_tramlaw_scenario,
    bench_insert_performance,
    bench_different_reducers,
    bench_split_join_operations,
    bench_macro_construction,
    bench_memory_usage_patterns
);

criterion_main!(benches);
