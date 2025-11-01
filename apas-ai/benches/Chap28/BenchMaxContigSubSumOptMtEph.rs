//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::time::Duration;

use criterion::*;

use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap28::MaxContigSubSumOptMtEph::MaxContigSubSumOptMtEph::*;

pub fn bench_optimal_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumOptMtEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a50 = ArraySeqMtEphS::<i32>::tabulate(&|i| (i as i32) % 10 - 4, 50);
    group.bench_function("opt_mt_50", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_opt_mt(black_box(&a50)))
    });

    let a100 = ArraySeqMtEphS::<i32>::tabulate(&|i| (i as i32) % 10 - 4, 100);
    group.bench_function("opt_mt_100", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_opt_mt(black_box(&a100)))
    });

    group.finish();
}

criterion_group!(benches, bench_optimal_mt);
criterion_main!(benches);
