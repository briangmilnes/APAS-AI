//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i, len);
            let m: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::map(&s, &|x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map_eph_ch19);
criterion_main!(benches);
