//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for ArraySetEnumMtEph

use std::time::Duration;

use apas_ai::ArraySetEnumMtEphLit;
use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_ai::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::*;
use criterion::*;

fn build_array_set_enum_mt(universe_size: usize, fill_ratio: f64) -> ArraySetEnumMtEph {
    let mut set = ArraySetEnumMtEph::empty(universe_size);
    let num_elements = (universe_size as f64 * fill_ratio) as usize;
    for i in 0..num_elements {
        set.insert(i);
    }
    set
}

fn bench_array_set_enum_mt_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_build");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        group.bench_with_input(BenchmarkId::new("build_50pct", size), size, |b, &size| {
            b.iter(|| black_box(build_array_set_enum_mt(size, 0.5)));
        });
    }
    group.finish();
}

fn bench_array_set_enum_mt_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_find");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        let set = build_array_set_enum_mt(*size, 0.5);

        group.bench_with_input(BenchmarkId::new("find_existing", size), size, |b, &size| {
            b.iter(|| {
                let target = size / 4; // Should exist
                black_box(set.find(target))
            });
        });

        group.bench_with_input(BenchmarkId::new("find_missing", size), size, |b, &size| {
            b.iter(|| {
                let target = size - 1; // Should not exist (only filled to 50%)
                black_box(set.find(target))
            });
        });
    }
    group.finish();
}

fn bench_array_set_enum_mt_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        group.bench_with_input(BenchmarkId::new("insert_new", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set_enum_mt(size, 0.5),
                |mut set| {
                    let new_value = size - 1; // Should be new
                    set.insert(new_value);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("insert_duplicate", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set_enum_mt(size, 0.5),
                |mut set| {
                    let existing_value = size / 4; // Should exist
                    set.insert(existing_value);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_array_set_enum_mt_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_delete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        group.bench_with_input(BenchmarkId::new("delete_existing", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set_enum_mt(size, 0.5),
                |mut set| {
                    let target = size / 4; // Should exist
                    set.delete(target);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("delete_missing", size), size, |b, &size| {
            b.iter_batched(
                || build_array_set_enum_mt(size, 0.5),
                |mut set| {
                    let target = size - 1; // Should not exist
                    set.delete(target);
                    black_box(set)
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_array_set_enum_mt_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_bulk");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        let set1 = build_array_set_enum_mt(*size, 0.5);
        let set2 = build_array_set_enum_mt(*size, 0.3);

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

fn bench_array_set_enum_mt_from_seq(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_from_seq");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        // Create sequence with some duplicates
        let mut vec_data = Vec::new();
        let num_elements = size / 2;
        for i in 0..num_elements {
            vec_data.push(i);
            if i % 3 == 0 {
                vec_data.push(i); // Add duplicate
            }
        }
        let seq = ArraySeqMtEphS::from_vec(vec_data);

        group.bench_with_input(BenchmarkId::new("from_seq", size), size, |b, &size| {
            b.iter(|| black_box(ArraySetEnumMtEph::from_seq(size, seq.clone())));
        });
    }
    group.finish();
}

fn bench_array_set_enum_mt_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEph_filter");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [300, 800].iter() {
        let set = build_array_set_enum_mt(*size, 0.5);

        group.bench_with_input(BenchmarkId::new("filter_half_parallel", size), size, |b, _| {
            fn is_even(x: usize) -> bool { x % 2 == 0 }
            b.iter(|| black_box(set.filter(is_even)));
        });

        group.bench_with_input(BenchmarkId::new("filter_simple_parallel", size), size, |b, _| {
            fn is_small(x: usize) -> bool { x < 100 }
            b.iter(|| black_box(set.filter(is_small)));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_array_set_enum_mt_build,
    bench_array_set_enum_mt_find,
    bench_array_set_enum_mt_insert,
    bench_array_set_enum_mt_delete,
    bench_array_set_enum_mt_bulk_operations,
    bench_array_set_enum_mt_from_seq,
    bench_array_set_enum_mt_filter
);
criterion_main!(benches);
