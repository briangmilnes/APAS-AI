//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::time::Duration;

use criterion::*;

use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap28::MaxContigSubSumDivConMtEph::MaxContigSubSumDivConMtEph::*;

pub fn bench_divcon_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumDivConMtEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a125 = ArraySeqMtEphS::<i32>::tabulate(&|i| (i as i32) % 10 - 4, 125);
    group.bench_function("divcon_mt_125", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_divcon_mt(black_box(&a125)))
    });

    let a200 = ArraySeqMtEphS::<i32>::tabulate(&|i| (i as i32) % 10 - 4, 200);
    group.bench_function("divcon_mt_200", |b| {
        b.iter(|| ArraySeqMtEphS::max_contig_sub_sum_divcon_mt(black_box(&a200)))
    });

    group.finish();
}

criterion_group!(benches, bench_divcon_mt);
criterion_main!(benches);
