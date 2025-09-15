use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqPer::ArraySeqPer::*;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use apas_ai::ArraySeqPer::ArraySeqPer::*;
use std::time::Duration;

fn bench_tabulate_map_per_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqPerChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap19Trait<T>>::tabulate(|i| i, len);
            let m: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap19Trait<T>>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map_per_ch19);
criterion_main!(benches);
