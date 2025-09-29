//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for AVLTreeSetStEph

use std::time::Duration;

use apas_ai::AVLTreeSetStEphLit;
use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS;
use apas_ai::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn build_avl_tree_set(len: usize) -> AVLTreeSetStEph<i32> {
    let mut set = AVLTreeSetStEph::empty();
    for i in 0..len {
        set = set.insert(i as i32);
    }
    set
}

fn bench_avl_tree_set_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_build");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000, 2000].iter() {
        group.bench_with_input(BenchmarkId::new("build", size), size, |b, &size| {
            b.iter(|| black_box(build_avl_tree_set(size)));
        });
    }
    group.finish();
}

fn bench_avl_tree_set_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_find");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000, 2000].iter() {
        let set = build_avl_tree_set(*size);
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

fn bench_avl_tree_set_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_insert");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("insert_new", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set(size),
                |set| {
                    let new_value = (size + 1) as i32;
                    black_box(set.insert(new_value))
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("insert_duplicate", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set(size),
                |set| {
                    let existing_value = (size / 2) as i32;
                    black_box(set.insert(existing_value))
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_avl_tree_set_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_delete");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("delete_existing", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set(size),
                |set| {
                    let target = (size / 2) as i32;
                    black_box(set.delete(&target))
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("delete_missing", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set(size),
                |set| {
                    let target = (size + 100) as i32;
                    black_box(set.delete(&target))
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_avl_tree_set_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_bulk");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000].iter() {
        let set1 = build_avl_tree_set(*size);
        let set2 = build_avl_tree_set(*size / 2);

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

fn bench_avl_tree_set_from_seq(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_from_seq");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000].iter() {
        // Create sequence with some duplicates
        let mut vec_data = Vec::new();
        for i in 0..*size {
            vec_data.push(i as i32);
            if i % 3 == 0 {
                vec_data.push(i as i32); // Add duplicate
            }
        }

        group.bench_with_input(BenchmarkId::new("from_seq", size), size, |b, _| {
            b.iter_batched(
                || AVLTreeSeqStEphS::from_vec(vec_data.clone()),
                |seq| black_box(AVLTreeSetStEph::from_seq(seq)),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_avl_tree_set_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_filter");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000].iter() {
        let set = build_avl_tree_set(*size);

        group.bench_with_input(BenchmarkId::new("filter_half", size), size, |b, _| {
            b.iter(|| black_box(set.filter(|&x| x % 2 == 0)));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_avl_tree_set_build,
    bench_avl_tree_set_find,
    bench_avl_tree_set_insert,
    bench_avl_tree_set_delete,
    bench_avl_tree_set_bulk_operations,
    bench_avl_tree_set_from_seq,
    bench_avl_tree_set_filter
);
criterion_main!(benches);
