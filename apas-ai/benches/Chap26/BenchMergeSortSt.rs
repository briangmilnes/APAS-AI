//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for sequential merge sort (Chapter 26).

use std::time::Duration;

use criterion::*;

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};
use apas_ai::Chap26::MergeSortSt::MergeSortSt::*;

fn bench_merge_sort_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_sort_st");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("sort_100", |b| {
        let seq = ArraySeqStPerS::tabulate(&|i| 100 - i, 100);
        b.iter(|| ArraySeqStPerS::merge_sort(black_box(&seq)))
    });

    group.bench_function("sort_1000", |b| {
        let seq = ArraySeqStPerS::tabulate(&|i| 1000 - i, 1000);
        b.iter(|| ArraySeqStPerS::merge_sort(black_box(&seq)))
    });

    group.finish();
}

criterion_group!(benches, bench_merge_sort_st);
criterion_main!(benches);
