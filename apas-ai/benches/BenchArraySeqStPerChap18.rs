use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
use std::time::Duration;

fn bench_tabulate_map_per_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqPerChap18");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArrayStPerS<N> = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::tabulate(|i| i, len);
            let m: ArrayStPerS<N> = <ArrayStPerS<N> as ArraySeqStPerChap18Trait<N>>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map_per_ch18);
criterion_main!(benches);


