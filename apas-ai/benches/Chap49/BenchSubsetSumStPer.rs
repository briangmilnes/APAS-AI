//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for SubsetSumStPer

use apas_ai::{Chap49::SubsetSumStPer::SubsetSumStPer::*, SubsetSumStPerLit};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_subset_sum_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_st_per");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let small_solver = SubsetSumStPerLit![1, 2, 3, 4, 5];
    group.bench_function("small_target_8", |b| {
        b.iter(|| black_box(small_solver.subset_sum(black_box(8))))
    });

    let medium_solver = SubsetSumStPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    group.bench_function("medium_target_25", |b| {
        b.iter(|| black_box(medium_solver.subset_sum(black_box(25))))
    });

    group.finish();
}

criterion_group!(benches, bench_subset_sum_st_per);
criterion_main!(benches);
