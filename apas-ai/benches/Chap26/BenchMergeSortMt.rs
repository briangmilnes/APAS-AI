//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for parallel merge sort (Chapter 26).

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::{
    ArraySeqMtPerBaseTrait, ArraySeqMtPerRedefinableTrait, ArraySeqMtPerS,
};
use apas_ai::Chap26::MergeSortMt::MergeSortMt::*;

fn bench_merge_sort_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_sort_mt");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("sort_parallel_100", |b| {
        let seq = ArraySeqMtPerS::tabulate(&|i| 100 - i, 100);
        b.iter(|| ArraySeqMtPerS::merge_sort_parallel(black_box(&seq)))
    });

    group.bench_function("sort_parallel_350", |b| {
        let seq = ArraySeqMtPerS::tabulate(&|i| 350 - i, 350);
        b.iter(|| ArraySeqMtPerS::merge_sort_parallel(black_box(&seq)))
    });

    group.finish();
}

criterion_group!(benches, bench_merge_sort_mt);
criterion_main!(benches);
