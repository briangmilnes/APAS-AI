//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for ArraySetStEph

use std::time::Duration;

use apas_ai::ArraySetStEphLit;
use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_array_set(len: usize) -> ArraySetStEph<i32> {
    let mut set = ArraySetStEph::empty();
    for i in 0..len {
        set.insert(i as i32);
    }
    set
}

fn bench_array_set_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_build");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("build", size), size, |b, &size| {
            b.iter(|| black_box(build_array_set(size)));
        });
    }
    group.finish();
}

fn bench_array_set_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_find");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20].iter() {
        let set = build_array_set(*size);
        group.bench_with_input(BenchmarkId::new("find_existing", size), size, |b, &size| {
            b.iter(|| {
                let target = (size / 2) as i32;
                black_box(set.find(&target))
            });
        });

        group.bench_with_input(BenchmarkId::new("find_missing", size), size, |b, &size| {
            b.iter(|| {
                let target = (size + 100) as i32;
                black_box(set.find(&target))
            });
        });
    }
    group.finish();
}

fn bench_array_set_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("insert_new", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set(size),
                |mut set| {
                    let new_value = (size + 1) as i32;
                    set.insert(new_value);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("insert_duplicate", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set(size),
                |mut set| {
                    let existing_value = (size / 2) as i32;
                    set.insert(existing_value);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_array_set_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_delete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("delete_existing", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set(size),
                |mut set| {
                    let target = (size / 2) as i32;
                    set.delete(&target);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("delete_missing", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set(size),
                |mut set| {
                    let target = (size + 100) as i32;
                    set.delete(&target);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_array_set_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_bulk");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [100, 1000].iter() {
        let set1 = build_array_set(*size);
        let set2 = build_array_set(*size / 2);

        group.bench_with_input(BenchmarkId::new("union", size), size, |b, _| {
            b.iter(|| black_box(set1.union(&set2)));
        });

        group.bench_with_input(BenchmarkId::new("intersection", size), size, |b, _| {
            b.iter(|| black_box(set1.intersection(&set2)));
        });

        group.bench_with_input(BenchmarkId::new("difference", size), size, |b, _| {
            b.iter(|| black_box(set1.difference(&set2)));
        });
    }
    group.finish();
}

fn bench_array_set_from_seq(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_from_seq");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [100, 1000].iter() {
        // Create sequence with some duplicates
        let mut vec_data = Vec::new();
        for i in 0..*size {
            vec_data.push(i as i32);
            if i % 3 == 0 {
                vec_data.push(i as i32); // Add duplicate
            }
        }
        let seq = ArraySeqStEphS::from_vec(vec_data);

        group.bench_with_input(BenchmarkId::new("from_seq", size), size, |b, _| {
            b.iter(|| black_box(ArraySetStEph::from_seq(seq.clone())));
        });
    }
    group.finish();
}

fn bench_array_set_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetStEph_filter");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [100, 1000].iter() {
        let set = build_array_set(*size);

        group.bench_with_input(BenchmarkId::new("filter_half", size), size, |b, _| {
            b.iter(|| black_box(set.filter(|&x| x % 2 == 0)));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_array_set_build,
    bench_array_set_find,
    bench_array_set_insert,
    bench_array_set_delete,
    bench_array_set_bulk_operations,
    bench_array_set_from_seq,
    bench_array_set_filter
);
criterion_main!(benches);
