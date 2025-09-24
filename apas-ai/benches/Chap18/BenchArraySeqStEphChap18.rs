//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::ArraySeqStEphChap18Trait;
use apas_ai::ArraySeqStEphSLit;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_tabulate_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqStEphChap18");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::tabulate(|i| i, len);
            let m: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map);
criterion_main!(benches);
