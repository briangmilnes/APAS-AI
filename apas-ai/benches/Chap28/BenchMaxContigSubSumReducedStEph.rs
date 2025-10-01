//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap28::MaxContigSubSumReducedStEph::MaxContigSubSumReducedStEph::MaxContigSubSumReducedTrait;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn bench_reduced(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumReducedStEph");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    let a100: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 100);
    group.bench_function("reduced_100", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_reduced(black_box(&a100)))
    });

    let a1000: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 1000);
    group.bench_function("reduced_1000", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_reduced(black_box(&a1000)))
    });

    group.finish();
}

criterion_group!(benches, bench_reduced);
criterion_main!(benches);
