//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait as Chap18BaseTrait, ArraySeqStPerRedefinableTrait as Chap18Trait};
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Types::Types::*;

fn bench_tabulate_map_per_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqStPerChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::tabulate(&|i| i, len);
            let m: ArraySeqStPerS<N> = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::map(&s, &|x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map_per_ch19);
criterion_main!(benches);
