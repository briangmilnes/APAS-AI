//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for single-threaded ephemeral reducer-augmented ordered table implementation.

use std::time::Duration;

use criterion::*;

use apas_ai::AugOrderedTableStEphLit;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::*;
use apas_ai::Types::Types::*;

fn bench_ephemeral_reduce_val_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_reduce_val");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    for size in [100, 500, 1000, 2000].iter() {
        // Create augmented table
        let mut aug_table = AugOrderedTableStEph::empty(sum_reducer, 0);
        for i in 1..=*size {
            aug_table.insert(i, i * 10, |_old, new| *new);
        }

        // Benchmark O(1) augmented reduceVal
        group.bench_with_input(BenchmarkId::new("ephemeral_reduce_val", size), size, |b, _| {
            b.iter(|| black_box(aug_table.reduce_val()))
        });

        // Benchmark O(n) naive reduction for comparison
        group.bench_with_input(BenchmarkId::new("naive_reduce_ephemeral", size), size, |b, _| {
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

fn bench_qadsan_stock_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("qadsan_stock_analysis");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut stock_table = AugOrderedTableStEph::empty(max_reducer, 0);

    // Simulate a full trading day (390 minutes: 9:30am-4:00pm)
    for minute in 570..960 {
        // 9:30am to 4:00pm in minutes from midnight
        let base_price = 15000; // $150.00 in cents
        let volatility = ((minute - 570) % 50) * 10; // Cyclical volatility
        let trend = (minute - 570) / 10; // Slight upward trend
        let price = base_price + volatility + trend;
        stock_table.insert(minute, price, |old, new| if old > new { *old } else { *new });
    }

    group.bench_function("daily_max_price", |b| b.iter(|| black_box(stock_table.reduce_val())));

    group.bench_function("morning_session_max", |b| {
        b.iter(|| {
            black_box(stock_table.reduce_range(&570, &720)) // 9:30am to 12:00pm
        })
    });

    group.bench_function("afternoon_session_max", |b| {
        b.iter(|| {
            black_box(stock_table.reduce_range(&720, &960)) // 12:00pm to 4:00pm
        })
    });

    group.bench_function("multiple_time_window_queries", |b| {
        b.iter(|| {
            let opening_hour = stock_table.reduce_range(&570, &630); // 9:30-10:30am
            let midday = stock_table.reduce_range(&720, &780); // 12:00-1:00pm
            let closing_hour = stock_table.reduce_range(&900, &960); // 3:00-4:00pm
            black_box((opening_hour, midday, closing_hour))
        })
    });

    group.finish();
}

fn bench_ephemeral_insert_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_insert_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    for initial_size in [0, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("insert_with_reduction_update", initial_size),
            initial_size,
            |b, size| {
                b.iter_batched(
                    || {
                        let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
                        // Pre-populate table
                        for i in 1..=*size {
                            table.insert(i, i * 10, |_old, new| *new);
                        }
                        table
                    },
                    |mut table| {
                        table.insert(*size + 1, (*size + 1) * 10, |_old, new| *new);
                        black_box(table.reduce_val())
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn bench_ephemeral_delete_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_delete_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("delete_with_reduction_recalc", size),
            size,
            |b, size| {
                b.iter_batched(
                    || {
                        let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
                        for i in 1..=*size {
                            table.insert(i, i * 10, |_old, new| *new);
                        }
                        table
                    },
                    |mut table| {
                        table.delete(&(*size / 2));
                        black_box(table.reduce_val())
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn bench_different_reducers_ephemeral(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_reducer_types");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let size = 1000;

    // Sum reducer
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut sum_table = AugOrderedTableStEph::empty(sum_reducer, 0);
    for i in 1..=size {
        sum_table.insert(i, i, |_old, new| *new);
    }

    // Max reducer
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut max_table = AugOrderedTableStEph::empty(max_reducer, i32::MIN);
    for i in 1..=size {
        max_table.insert(i, i, |_old, new| *new);
    }

    // Min reducer
    let min_reducer = |a: &i32, b: &i32| if a < b { *a } else { *b };
    let mut min_table = AugOrderedTableStEph::empty(min_reducer, i32::MAX);
    for i in 1..=size {
        min_table.insert(i, i, |_old, new| *new);
    }

    group.bench_function("sum_reducer_ephemeral", |b| {
        b.iter(|| black_box(sum_table.reduce_val()))
    });

    group.bench_function("max_reducer_ephemeral", |b| {
        b.iter(|| black_box(max_table.reduce_val()))
    });

    group.bench_function("min_reducer_ephemeral", |b| {
        b.iter(|| black_box(min_table.reduce_val()))
    });

    group.finish();
}

fn bench_ephemeral_mutable_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_mutable_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("union_operation", |b| {
        b.iter_batched(
            || {
                let mut table1 = AugOrderedTableStEph::empty(sum_reducer, 0);
                let mut table2 = AugOrderedTableStEph::empty(sum_reducer, 0);

                for i in 1..=300 {
                    table1.insert(i, i * 10, |_old, new| *new);
                }
                for i in 240..=540 {
                    table2.insert(i, i * 5, |_old, new| *new);
                }
                (table1, table2)
            },
            |(mut table1, table2)| {
                table1.union(&table2, |v1, v2| v1 + v2);
                black_box(table1.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("intersection_operation", |b| {
        b.iter_batched(
            || {
                let mut table1 = AugOrderedTableStEph::empty(sum_reducer, 0);
                let mut table2 = AugOrderedTableStEph::empty(sum_reducer, 0);

                for i in 1..=300 {
                    table1.insert(i, i * 10, |_old, new| *new);
                }
                for i in 150..=450 {
                    table2.insert(i, i * 5, |_old, new| *new);
                }
                (table1, table2)
            },
            |(mut table1, table2)| {
                table1.intersection(&table2, |v1, v2| v1 + v2);
                black_box(table1.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("difference_operation", |b| {
        b.iter_batched(
            || {
                let mut table1 = AugOrderedTableStEph::empty(sum_reducer, 0);
                let mut table2 = AugOrderedTableStEph::empty(sum_reducer, 0);

                for i in 1..=300 {
                    table1.insert(i, i * 10, |_old, new| *new);
                }
                for i in 120..=180 {
                    table2.insert(i, i * 5, |_old, new| *new);
                }
                (table1, table2)
            },
            |(mut table1, table2)| {
                table1.difference(&table2);
                black_box(table1.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_ephemeral_split_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_split_join");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("split_key_operation", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
                for i in 1..=600 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                table
            },
            |mut table| {
                let (left, right) = table.split_key(&300);
                black_box((left.reduce_val(), right.reduce_val()))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("join_key_operation", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
                for i in 1..=600 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                table.split_key(&300)
            },
            |(mut left, right)| {
                left.join_key(right);
                black_box(left.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_ephemeral_range_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_range_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Create large dataset
    for i in 1..=2000 {
        table.insert(i, i * 2, |_old, new| *new);
    }

    let range_sizes = [100, 500, 1000, 2000];

    for range_size in range_sizes.iter() {
        let start_key = 1000;
        let end_key = start_key + range_size;

        group.bench_with_input(
            BenchmarkId::new("reduce_range_ephemeral", range_size),
            range_size,
            |b, _| b.iter(|| black_box(table.reduce_range(&start_key, &end_key))),
        );
    }

    group.finish();
}

fn bench_ephemeral_macro_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_construction");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("macro_construction_ephemeral", |b| {
        b.iter(|| {
            let table: AugOrderedTableStEph<i32, i32, _> = AugOrderedTableStEphLit![
                reducer: sum_reducer, identity: 0,
                1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50
            ];
            black_box(table.reduce_val())
        })
    });

    group.bench_function("iterative_construction_ephemeral", |b| {
        b.iter(|| {
            let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
            table.insert(1, 10, |_old, new| *new);
            table.insert(2, 20, |_old, new| *new);
            table.insert(3, 30, |_old, new| *new);
            table.insert(4, 40, |_old, new| *new);
            table.insert(5, 50, |_old, new| *new);
            black_box(table.reduce_val())
        })
    });

    group.finish();
}

fn bench_ephemeral_update_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("ephemeral_update_patterns");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("replace_updates", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
                for i in 1..=100 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                table
            },
            |mut table| {
                for i in 1..=100 {
                    table.insert(i, i * 20, |_old, new| *new); // Replace
                }
                black_box(table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("accumulate_updates", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);
                for i in 1..=100 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                table
            },
            |mut table| {
                for i in 1..=100 {
                    table.insert(i, i * 5, |old, new| old + new); // Accumulate
                }
                black_box(table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_ephemeral_reduce_val_performance,
    bench_qadsan_stock_scenario,
    bench_ephemeral_insert_performance,
    bench_ephemeral_delete_performance,
    bench_different_reducers_ephemeral,
    bench_ephemeral_mutable_operations,
    bench_ephemeral_split_join,
    bench_ephemeral_range_operations,
    bench_ephemeral_macro_construction,
    bench_ephemeral_update_patterns
);

criterion_main!(benches);
