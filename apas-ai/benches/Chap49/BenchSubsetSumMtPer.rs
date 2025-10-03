//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for SubsetSumMtPer

use apas_ai::{Chap49::SubsetSumMtPer::SubsetSumMtPer::*, SubsetSumMtPerLit};
use criterion::*;
use std::time::Duration;

fn bench_subset_sum_mt_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_mt_per");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let small_solver = SubsetSumMtPerLit![1, 2, 3, 4, 5];
    group.bench_function("small_parallel_target_8", |b| {
        b.iter(|| black_box(small_solver.subset_sum(black_box(8))))
    });

    let medium_solver = SubsetSumMtPerLit![1, 2, 3, 4, 5, 6, 7, 8];
    group.bench_function("medium_parallel_target_20", |b| {
        b.iter(|| black_box(medium_solver.subset_sum(black_box(20))))
    });

    group.finish();
}

criterion_group!(benches, bench_subset_sum_mt_per);
criterion_main!(benches);
