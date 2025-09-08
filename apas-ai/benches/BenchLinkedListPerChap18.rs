use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::{LinkedListPerS, LinkedListPerTrait};
use apas_ai::LinkedListPerChap18Trait;
use std::time::Duration;

fn bench_ll_per_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListPerChap18_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: LinkedListPerS<N> = <LinkedListPerS<N> as LinkedListPerChap18Trait>::tabulate(|i| i, len);
            let m: LinkedListPerS<N> = <LinkedListPerS<N> as LinkedListPerChap18Trait>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_per_ch18);
criterion_main!(benches);


