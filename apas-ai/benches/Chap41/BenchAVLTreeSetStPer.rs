//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for AVLTreeSetStPer

use std::time::Duration;

use apas_ai::AVLTreeSetStPerLit;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS;
use apas_ai::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_avl_tree_set_per(len: usize) -> AVLTreeSetStPer<i32> {
    let mut set = AVLTreeSetStPer::empty();
    for i in 0..len {
        set = set.insert(i as i32);
    }
    set
}

fn bench_avl_tree_set_per_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_build");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
        group.bench_with_input(BenchmarkId::new("build", size), size, |b, &size| {
            b.iter(|| black_box(build_avl_tree_set_per(size)));
        });
    }
    group.finish();
}

fn bench_avl_tree_set_per_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_find");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
        let set = build_avl_tree_set_per(*size);
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

fn bench_avl_tree_set_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
        group.bench_with_input(BenchmarkId::new("insert_new", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set_per(size),
                |set| {
                    let new_value = (size + 1) as i32;
                    black_box(set.insert(new_value))
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("insert_duplicate", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set_per(size),
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

fn bench_avl_tree_set_per_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_delete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
        group.bench_with_input(BenchmarkId::new("delete_existing", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set_per(size),
                |set| {
                    let target = (size / 2) as i32;
                    black_box(set.delete(&target))
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("delete_missing", size), size, |b, &size| {
            b.iter_batched(
                || build_avl_tree_set_per(size),
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

fn bench_avl_tree_set_per_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_bulk");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
        let set1 = build_avl_tree_set_per(*size);
        let set2 = build_avl_tree_set_per(*size / 2);

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

fn bench_avl_tree_set_per_from_seq(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_from_seq");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
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
                || {
                    use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
                    AVLTreeSeqStPerS::from_vec(vec_data.clone())
                },
                |seq| black_box(AVLTreeSetStPer::from_seq(seq)),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_avl_tree_set_per_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPer_filter");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [20].iter() {
        let set = build_avl_tree_set_per(*size);

        group.bench_with_input(BenchmarkId::new("filter_half", size), size, |b, _| {
            b.iter(|| black_box(set.filter(|&x| x % 2 == 0)));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_avl_tree_set_per_build,
    bench_avl_tree_set_per_find,
    bench_avl_tree_set_per_insert,
    bench_avl_tree_set_per_delete,
    bench_avl_tree_set_per_bulk_operations,
    bench_avl_tree_set_per_from_seq,
    bench_avl_tree_set_per_filter
);
criterion_main!(benches);
