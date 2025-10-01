//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for multi-threaded ephemeral reducer-augmented ordered table implementation.

use apas_ai::AugOrderedTableMtEphLit;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::{Arc, Mutex};
use std::thread;

fn bench_multithreaded_reduce_val_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_reduce_val");

    let sum_reducer = |a: &i32, b: &i32| a + b;

    for size in [100, 500, 1000, 2000].iter() {
        // Create augmented table
        let mut aug_table = AugOrderedTableMtEph::empty(sum_reducer, 0);
        for i in 1..=*size {
            aug_table.insert(i, i * 10, |_old, new| *new);
        }

        // Benchmark O(1) augmented reduceVal
        group.bench_with_input(BenchmarkId::new("multithreaded_reduce_val", size), size, |b, _| {
            b.iter(|| black_box(aug_table.reduce_val()))
        });

        // Benchmark parallel range reduction
        group.bench_with_input(BenchmarkId::new("parallel_range_reduction", size), size, |b, _| {
            b.iter(|| {
                let start = *size / 4;
                let end = (*size * 3) / 4;
                black_box(aug_table.reduce_range_parallel(&start, &end))
            })
        });
    }

    group.finish();
}

fn bench_concurrent_access_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");

    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    // Pre-populate table
    for i in 1..=1000 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    let table = Arc::new(table);

    group.bench_function("concurrent_reads", |b| {
        b.iter(|| {
            let table_clone = Arc::clone(&table);
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let table_ref = Arc::clone(&table_clone);
                    thread::spawn(move || table_ref.reduce_val())
                })
                .collect();

            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            black_box(results)
        })
    });

    group.bench_function("concurrent_range_queries", |b| {
        b.iter(|| {
            let table_clone = Arc::clone(&table);
            let handles: Vec<_> = (0..4)
                .map(|i| {
                    let table_ref = Arc::clone(&table_clone);
                    let start = (i * 250) + 1;
                    let end = (i + 1) * 250;
                    thread::spawn(move || table_ref.reduce_range(&start, &end))
                })
                .collect();

            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            black_box(results)
        })
    });

    group.finish();
}

fn bench_qadsan_multithreaded_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("qadsan_multithreaded_stock");

    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };

    group.bench_function("concurrent_venue_updates", |b| {
        b.iter_batched(
            || Arc::new(Mutex::new(AugOrderedTableMtEph::empty(max_reducer, 0))),
            |table| {
                let handles: Vec<_> = (0..3)
                    .map(|venue_id| {
                        let table_clone = Arc::clone(&table);
                        thread::spawn(move || {
                            let base_time = venue_id * 100;
                            for minute in 0..60 {
                                let timestamp = base_time + minute;
                                let price = 15000 + (minute * 10) + (venue_id * 50);

                                let mut t = table_clone.lock().unwrap();
                                t.insert(timestamp, price, |old, new| if old > new { *old } else { *new });
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }

                let final_table = table.lock().unwrap();
                black_box(final_table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("concurrent_analysis_queries", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableMtEph::empty(max_reducer, 0);
                // Simulate full trading day
                for minute in 570..960 {
                    let price = 15000 + ((minute - 570) % 100) * 5;
                    table.insert(minute, price, |old, new| if old > new { *old } else { *new });
                }
                Arc::new(table)
            },
            |table| {
                let handles: Vec<_> = vec![
                    (570, 630), // Opening hour
                    (720, 780), // Lunch hour
                    (900, 960), // Closing hour
                ]
                .into_iter()
                .map(|(start, end)| {
                    let table_clone = Arc::clone(&table);
                    thread::spawn(move || table_clone.reduce_range(&start, &end))
                })
                .collect();

                let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
                black_box(results)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_multithreaded_insert_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_insert");

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("sequential_inserts", |b| {
        b.iter_batched(
            || AugOrderedTableMtEph::empty(sum_reducer, 0),
            |mut table| {
                for i in 1..=100 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                black_box(table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("concurrent_inserts_with_mutex", |b| {
        b.iter_batched(
            || Arc::new(Mutex::new(AugOrderedTableMtEph::empty(sum_reducer, 0))),
            |table| {
                let handles: Vec<_> = (0..4)
                    .map(|thread_id| {
                        let table_clone = Arc::clone(&table);
                        thread::spawn(move || {
                            for i in 0..25 {
                                let key = thread_id * 25 + i + 1;
                                let value = key * 10;
                                let mut t = table_clone.lock().unwrap();
                                t.insert(key, value, |_old, new| *new);
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }

                let final_table = table.lock().unwrap();
                black_box(final_table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_parallel_vs_sequential_range_reduction(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_vs_sequential_range");

    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);

    // Create large dataset to benefit from parallelism
    for i in 1..=5000 {
        table.insert(i, i * 2, |_old, new| *new);
    }

    let large_range_start = 1000;
    let large_range_end = 4000;

    group.bench_function("sequential_range_reduction", |b| {
        b.iter(|| black_box(table.reduce_range(&large_range_start, &large_range_end)))
    });

    group.bench_function("parallel_range_reduction", |b| {
        b.iter(|| black_box(table.reduce_range_parallel(&large_range_start, &large_range_end)))
    });

    // Test with different range sizes
    for range_size in [500, 1000, 2000, 3000].iter() {
        let end = large_range_start + range_size;

        group.bench_with_input(
            BenchmarkId::new("parallel_range_by_size", range_size),
            range_size,
            |b, _| b.iter(|| black_box(table.reduce_range_parallel(&large_range_start, &end))),
        );
    }

    group.finish();
}

fn bench_different_reducers_multithreaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_reducer_types");

    let size = 1000;

    // Sum reducer
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut sum_table = AugOrderedTableMtEph::empty(sum_reducer, 0);
    for i in 1..=size {
        sum_table.insert(i, i, |_old, new| *new);
    }

    // Max reducer
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut max_table = AugOrderedTableMtEph::empty(max_reducer, i32::MIN);
    for i in 1..=size {
        max_table.insert(i, i, |_old, new| *new);
    }

    // Min reducer
    let min_reducer = |a: &i32, b: &i32| if a < b { *a } else { *b };
    let mut min_table = AugOrderedTableMtEph::empty(min_reducer, i32::MAX);
    for i in 1..=size {
        min_table.insert(i, i, |_old, new| *new);
    }

    group.bench_function("sum_reducer_multithreaded", |b| {
        b.iter(|| black_box(sum_table.reduce_val()))
    });

    group.bench_function("max_reducer_multithreaded", |b| {
        b.iter(|| black_box(max_table.reduce_val()))
    });

    group.bench_function("min_reducer_multithreaded", |b| {
        b.iter(|| black_box(min_table.reduce_val()))
    });

    group.finish();
}

fn bench_multithreaded_mutable_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_mutable_operations");

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("union_operation_multithreaded", |b| {
        b.iter_batched(
            || {
                let mut table1 = AugOrderedTableMtEph::empty(sum_reducer, 0);
                let mut table2 = AugOrderedTableMtEph::empty(sum_reducer, 0);

                for i in 1..=500 {
                    table1.insert(i, i * 10, |_old, new| *new);
                }
                for i in 400..=900 {
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

    group.bench_function("intersection_operation_multithreaded", |b| {
        b.iter_batched(
            || {
                let mut table1 = AugOrderedTableMtEph::empty(sum_reducer, 0);
                let mut table2 = AugOrderedTableMtEph::empty(sum_reducer, 0);

                for i in 1..=500 {
                    table1.insert(i, i * 10, |_old, new| *new);
                }
                for i in 250..=750 {
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

    group.finish();
}

fn bench_multithreaded_split_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_split_join");

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("split_key_multithreaded", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);
                for i in 1..=1000 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                table
            },
            |mut table| {
                let (left, right) = table.split_key(&500);
                black_box((left.reduce_val(), right.reduce_val()))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("join_key_multithreaded", |b| {
        b.iter_batched(
            || {
                let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);
                for i in 1..=1000 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                table.split_key(&500)
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

fn bench_multithreaded_macro_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_construction");

    let sum_reducer = |a: &i32, b: &i32| a + b;

    group.bench_function("macro_construction_multithreaded", |b| {
        b.iter(|| {
            let table: AugOrderedTableMtEph<i32, i32, _> = AugOrderedTableMtEphLit![
                reducer: sum_reducer, identity: 0,
                1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50
            ];
            black_box(table.reduce_val())
        })
    });

    group.bench_function("iterative_construction_multithreaded", |b| {
        b.iter(|| {
            let mut table = AugOrderedTableMtEph::empty(sum_reducer, 0);
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

fn bench_thread_safety_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_safety_overhead");

    let sum_reducer = |a: &i32, b: &i32| a + b;

    // Compare single-threaded usage of multithreaded structure
    group.bench_function("single_threaded_usage", |b| {
        b.iter_batched(
            || AugOrderedTableMtEph::empty(sum_reducer, 0),
            |mut table| {
                for i in 1..=100 {
                    table.insert(i, i * 10, |_old, new| *new);
                }
                black_box(table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Measure overhead of Arc<Mutex<>> wrapper
    group.bench_function("mutex_wrapped_usage", |b| {
        b.iter_batched(
            || Arc::new(Mutex::new(AugOrderedTableMtEph::empty(sum_reducer, 0))),
            |table| {
                for i in 1..=100 {
                    let mut t = table.lock().unwrap();
                    t.insert(i, i * 10, |_old, new| *new);
                }
                let final_table = table.lock().unwrap();
                black_box(final_table.reduce_val())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_multithreaded_reduce_val_performance,
    bench_concurrent_access_patterns,
    bench_qadsan_multithreaded_scenario,
    bench_multithreaded_insert_performance,
    bench_parallel_vs_sequential_range_reduction,
    bench_different_reducers_multithreaded,
    bench_multithreaded_mutable_operations,
    bench_multithreaded_split_join,
    bench_multithreaded_macro_construction,
    bench_thread_safety_overhead
);

criterion_main!(benches);
