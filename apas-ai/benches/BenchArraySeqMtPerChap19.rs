use apas_ai::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::ArraySeqMtPerChap19::ArraySeqMtPerChap19::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtPerChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArrayMtPerS<N> = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::tabulate(|i| i, len);
            let m: ArrayMtPerS<N> = <ArrayMtPerS<N> as ArraySeqMtPerChap19Trait<N>>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map_mtper_ch19);
criterion_main!(benches);
