//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 42 single-threaded ephemeral table implementation.

use std::time::Duration;

use criterion::*;

use apas_ai::Types::Types::*;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap42::TableStEph::TableStEph::*;

fn build_table(size: usize) -> TableStEph<i32, String> {
    let mut table = TableStEph::empty();
    for i in 0..size {
        table.insert(i as i32, format!("value_{i}"), |_old, new| new.clone());
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
    let mut group = c.benchmark_group("TableStEph Operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    {
        let size = &10;
        // Benchmark insert (ephemeral)
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    let new_key = (size + 1) as i32;
                    let new_value = format!("new_value_{}", size + 1);
                    table.insert(new_key, new_value, |_old, new| new.clone());
                    black_box(table)
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

        // Benchmark delete (ephemeral)
        group.bench_with_input(BenchmarkId::new("delete", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    let key = (size / 2) as i32;
                    table.delete(&key);
                    black_box(table)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark domain
        group.bench_with_input(BenchmarkId::new("domain", size), size, |b, &size| {
            let table = build_table(size);
            b.iter(|| black_box(table.domain()));
        });

        // Benchmark map (ephemeral)
        group.bench_with_input(BenchmarkId::new("map", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    table.map(|s| s.to_uppercase());
                    black_box(table)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark filter (ephemeral)
        group.bench_with_input(BenchmarkId::new("filter", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    table.filter(|k, _v| *k % 2 == 0);
                    black_box(table)
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_table_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStEph Bulk Operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    {
        let size = &10;
        // Benchmark intersection (ephemeral)
        group.bench_with_input(BenchmarkId::new("intersection", size), size, |b, &size| {
            b.iter_batched(
                || (build_table(size), build_table(size / 2)),
                |(mut table1, table2)| {
                    table1.intersection(&table2, |v1, v2| format!("{v1}+{v2}"));
                    black_box(table1)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark union (ephemeral)
        group.bench_with_input(BenchmarkId::new("union", size), size, |b, &size| {
            b.iter_batched(
                || (build_table(size), build_table(size / 2)),
                |(mut table1, table2)| {
                    table1.union(&table2, |v1, v2| format!("{v1}+{v2}"));
                    black_box(table1)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark difference (ephemeral)
        group.bench_with_input(BenchmarkId::new("difference", size), size, |b, &size| {
            b.iter_batched(
                || (build_table(size), build_table(size / 2)),
                |(mut table1, table2)| {
                    table1.difference(&table2);
                    black_box(table1)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark restrict (ephemeral)
        group.bench_with_input(BenchmarkId::new("restrict", size), size, |b, &size| {
            b.iter_batched(
                || (build_table(size), build_set(size / 2)),
                |(mut table, keys)| {
                    table.restrict(&keys);
                    black_box(table)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark subtract (ephemeral)
        group.bench_with_input(BenchmarkId::new("subtract", size), size, |b, &size| {
            b.iter_batched(
                || (build_table(size), build_set(size / 2)),
                |(mut table, keys)| {
                    table.subtract(&keys);
                    black_box(table)
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark tabulate
        group.bench_with_input(BenchmarkId::new("tabulate", size), size, |b, &size| {
            let keys = build_set(size);
            b.iter(|| black_box(TableStEph::tabulate(|k| format!("computed_{}", k * k), &keys)));
        });
    }

    group.finish();
}

fn bench_table_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStEph Construction");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    {
        let size = &10;
        // Benchmark sequential construction (ephemeral)
        group.bench_with_input(BenchmarkId::new("sequential_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut table = TableStEph::empty();
                for i in 0..size {
                    table.insert(i, format!("value_{i}"), |_old, new| new.clone());
                }
                black_box(table)
            });
        });

        // Benchmark singleton creation
        group.bench_with_input(BenchmarkId::new("singleton", size), size, |b, &_size| {
            b.iter(|| black_box(TableStEph::singleton(42, "test_value".to_string())));
        });
    }

    group.finish();
}

fn bench_table_ephemeral_semantics(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStEph Ephemeral Semantics");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(150));

    {
        let size = &20;
        // Benchmark ephemeral insert (modifies original)
        group.bench_with_input(BenchmarkId::new("ephemeral_insert", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    let original_size = table.size();
                    table.insert(9999, "new_value".to_string(), |_old, new| new.clone());
                    // Verify ephemeral behavior - original is modified
                    black_box((original_size, table.size()))
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark ephemeral delete (modifies original)
        group.bench_with_input(BenchmarkId::new("ephemeral_delete", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    let original_size = table.size();
                    table.delete(&((size / 2) as i32));
                    // Verify ephemeral behavior - original is modified
                    black_box((original_size, table.size()))
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark ephemeral map (modifies original)
        group.bench_with_input(BenchmarkId::new("ephemeral_map", size), size, |b, &size| {
            b.iter_batched(
                || build_table(size),
                |mut table| {
                    table.map(|s| s.to_uppercase());
                    // Access modified table to ensure it's valid
                    black_box(table.size())
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_table_operations,
    bench_table_bulk_operations,
    bench_table_construction,
    bench_table_ephemeral_semantics
);
criterion_main!(benches);
