use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqEph::ArraySeqEph::ArraySeqEphS;
use apas_ai::ArraySeqEph::ArraySeqEph::ArraySeqEphTrait;
use apas_ai::ArraySeqEphChap18::ArraySeqEphChap18Trait;
use std::time::Duration;

fn bench_tabulate_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqEphChap18");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphChap18Trait<T>>::tabulate(|i| i, len);
            let m: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphChap18Trait<T>>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map);
criterion_main!(benches);




