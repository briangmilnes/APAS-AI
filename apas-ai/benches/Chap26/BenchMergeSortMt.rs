//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for parallel merge sort (Chapter 26).

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerS, ArraySeqMtPerTrait};
use apas_ai::Chap26::MergeSortMt::MergeSortMt::MergeSortMtTrait;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_merge_sort_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_sort_mt");
    group.warm_up_time(Duration::from_millis(1000));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    group.bench_function("sort_parallel_100", |b| {
        let seq = ArraySeqMtPerS::tabulate(&|i| 100 - i, 100);
        b.iter(|| ArraySeqMtPerS::merge_sort_parallel(black_box(&seq)))
    });

    group.bench_function("sort_parallel_1000", |b| {
        let seq = ArraySeqMtPerS::tabulate(&|i| 1000 - i, 1000);
        b.iter(|| ArraySeqMtPerS::merge_sort_parallel(black_box(&seq)))
    });

    group.finish();
}

criterion_group!(benches, bench_merge_sort_mt);
criterion_main!(benches);
