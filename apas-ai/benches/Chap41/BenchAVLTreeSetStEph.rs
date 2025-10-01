//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for AVLTreeSetStEph - Ultra-fast minimal version

use std::time::Duration;

use apas_ai::AVLTreeSetStEphLit;
use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS;
use apas_ai::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_avl_tree_set(len: usize) -> AVLTreeSetStEph<i32> {
    let mut set = AVLTreeSetStEph::empty();
    for i in 0..len {
        set.insert(i as i32);
    }
    set
}

fn bench_avl_tree_set_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStEph_basic");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // Only test size 3 to keep it ultra-fast
    let size = 3;

    // Build benchmark
    group.bench_function("build", |b| {
        b.iter(|| black_box(build_avl_tree_set(size)));
    });

    // Find benchmark
    let set = build_avl_tree_set(size);
    group.bench_function("find", |b| {
        b.iter(|| black_box(set.find(&1)));
    });

    // Insert benchmark
    group.bench_function("insert", |b| {
        b.iter_batched(
            || build_avl_tree_set(size),
            |mut set| {
                set.insert(99);
                black_box(set)
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_avl_tree_set_basic);
criterion_main!(benches);
