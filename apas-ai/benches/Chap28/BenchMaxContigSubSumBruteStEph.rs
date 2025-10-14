//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::time::Duration;

use criterion::*;

use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap28::MaxContigSubSumBruteStEph::MaxContigSubSumBruteStEph::MaxContigSubSumBruteTrait;

pub fn bench_brute_force(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap28::MaxContigSubSumBruteStEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let a10: ArraySeqStEphS<i32> = ArraySeqStEphSLit![1, -2, 0, 3, -1, 0, 2, -3, 4, -1];
    group.bench_function("brute_10", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_brute(black_box(&a10)))
    });

    let a20: ArraySeqStEphS<i32> = ArraySeqStEphS::tabulate(&|i| (i as i32) % 5 - 2, 20);
    group.bench_function("brute_20", |b| {
        b.iter(|| ArraySeqStEphS::max_contig_sub_sum_brute(black_box(&a20)))
    });

    group.finish();
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
