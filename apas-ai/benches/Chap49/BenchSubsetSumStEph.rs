//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for SubsetSumStEph

use apas_ai::{Chap49::SubsetSumStEph::SubsetSumStEph::*, SubsetSumStEphLit};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_subset_sum_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_st_eph");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    group.bench_function("small_with_mutation", |b| {
        b.iter(|| {
            let mut solver = SubsetSumStEphLit![1, 2, 3, 4, 5];
            let result1 = black_box(solver.subset_sum(black_box(8)));
            solver.set(0, 10);
            let result2 = black_box(solver.subset_sum(black_box(15)));
            black_box((result1, result2))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_subset_sum_st_eph);
criterion_main!(benches);
