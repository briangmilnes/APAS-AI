//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
use apas_ai::Chap28::MaxContigSubSumDivConMtEph::MaxContigSubSumDivConMtEph::MaxContigSubSumDivConMtTrait;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

pub fn bench_divcon_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumDivConMtEph");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    let a1000: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 1000);
    group.bench_function("divcon_mt_1000", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_divcon_mt(black_box(&a1000)))
    });

    let a10000: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 10000);
    group.bench_function("divcon_mt_10000", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_divcon_mt(black_box(&a10000)))
    });

    group.finish();
}

criterion_group!(benches, bench_divcon_mt);
criterion_main!(benches);
