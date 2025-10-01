//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap28::MaxContigSubSumOptStEph::MaxContigSubSumOptStEph::MaxContigSubSumOptTrait;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn bench_optimal(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumOptStEph");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    let a1000: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 1000);
    group.bench_function("opt_1000", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_opt(black_box(&a1000)))
    });

    let a10000: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 10000);
    group.bench_function("opt_10000", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_opt(black_box(&a10000)))
    });

    let a100000: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 100000);
    group.bench_function("opt_100000", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_opt(black_box(&a100000)))
    });

    group.finish();
}

criterion_group!(benches, bench_optimal);
criterion_main!(benches);
