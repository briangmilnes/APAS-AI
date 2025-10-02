//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
use apas_ai::Chap28::MaxContigSubSumDivConOptMtEph::MaxContigSubSumDivConOptMtEph::MaxContigSubSumDivConOptMtTrait;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

pub fn bench_divcon_opt_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumDivConOptMtEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a125: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 125);
    group.bench_function("divcon_opt_mt_125", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_divcon_opt_mt(black_box(&a125)))
    });

    let a1250: ArraySeqMtEphS<i32> = ArraySeqMtEphS::tabulate(&|i| (i as i32) % 10 - 4, 1250);
    group.bench_function("divcon_opt_mt_1250", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_divcon_opt_mt(black_box(&a1250)))
    });

    group.finish();
}

criterion_group!(benches, bench_divcon_opt_mt);
criterion_main!(benches);
