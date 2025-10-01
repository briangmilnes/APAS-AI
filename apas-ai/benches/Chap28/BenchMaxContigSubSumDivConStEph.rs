//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap28::MaxContigSubSumDivConStEph::MaxContigSubSumDivConStEph::MaxContigSubSumDivConTrait;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn bench_divcon(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumDivConStEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a1000: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 1000);
    group.bench_function("divcon_1000", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_divcon(black_box(&a1000)))
    });

    let a10000: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 10 - 4, 10000);
    group.bench_function("divcon_10000", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_divcon(black_box(&a10000)))
    });

    group.finish();
}

criterion_group!(benches, bench_divcon);
criterion_main!(benches);
