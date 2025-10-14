//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for sequential divide-and-conquer via reduce (Chapter 26).

use std::time::Duration;

use criterion::*;

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};
use apas_ai::Chap26::DivConReduceSt::DivConReduceSt::DivConReduceStTrait;

fn bench_divcon_reduce_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("divcon_reduce_st");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("sum_10000", |b| {
        let seq = ArraySeqStPerS::tabulate(&|i| i, 10000);
        b.iter(|| ArraySeqStPerS::sum(black_box(&seq)))
    });

    group.bench_function("max_10000", |b| {
        let seq = ArraySeqStPerS::tabulate(&|i| i * 7 % 1000, 10000);
        b.iter(|| ArraySeqStPerS::max_element(black_box(&seq)))
    });

    group.finish();
}

criterion_group!(benches, bench_divcon_reduce_st);
criterion_main!(benches);
