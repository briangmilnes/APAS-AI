//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};
use apas_ai::Chap35::OrderStatSelectStPer::OrderStatSelectStPer::OrderStatSelectStPerTrait;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn bench_select_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap35::OrderStatSelectStPer");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a100: ArraySeqStPerS<i32> = ArraySeqStPerS::tabulate(&|i| ((i * 7) % 100) as i32, 100);
    group.bench_function("select_median_100", |b| {
        b.iter(|| black_box(&a100).select(black_box(50)))
    });

    let a1000: ArraySeqStPerS<i32> = ArraySeqStPerS::tabulate(&|i| ((i * 7) % 1000) as i32, 1000);
    group.bench_function("select_median_1000", |b| {
        b.iter(|| black_box(&a1000).select(black_box(500)))
    });

    group.finish();
}

criterion_group!(benches, bench_select_st_per);
criterion_main!(benches);
