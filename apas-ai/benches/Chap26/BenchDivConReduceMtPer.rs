//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for parallel divide-and-conquer via reduce (Chapter 26).

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::{
    ArraySeqMtPerBaseTrait, ArraySeqMtPerRedefinableTrait, ArraySeqMtPerS,
};
use apas_ai::Chap26::DivConReduceMtPer::DivConReduceMtPer::*;

fn bench_divcon_reduce_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("divcon_reduce_mt");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("sum_parallel_10000", |b| {
        let seq = ArraySeqMtPerS::tabulate(&|i| i, 600);
        b.iter(|| ArraySeqMtPerS::sum_parallel(black_box(&seq)))
    });

    group.bench_function("max_parallel_10000", |b| {
        let seq = ArraySeqMtPerS::tabulate(&|i| i * 7 % 1000, 600);
        b.iter(|| ArraySeqMtPerS::max_element_parallel(black_box(&seq)))
    });

    group.finish();
}

criterion_group!(benches, bench_divcon_reduce_mt);
criterion_main!(benches);
