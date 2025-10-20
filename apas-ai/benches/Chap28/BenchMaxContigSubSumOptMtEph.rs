//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphBaseTrait, ArraySeqMtEphRedefinableTrait};
use apas_ai::Chap28::MaxContigSubSumOptMtEph::MaxContigSubSumOptMtEph::MaxContigSubSumOptMtTrait;

pub fn bench_optimal_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumOptMtEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a1000: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 1000);
    group.bench_function("opt_mt_1000", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_opt_mt(black_box(&a1000)))
    });

    let a10000: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 10000);
    group.bench_function("opt_mt_10000", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_opt_mt(black_box(&a10000)))
    });

    let a100000: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 100000);
    group.bench_function("opt_mt_100000", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_opt_mt(black_box(&a100000)))
    });

    group.finish();
}

criterion_group!(benches, bench_optimal_mt);
criterion_main!(benches);
