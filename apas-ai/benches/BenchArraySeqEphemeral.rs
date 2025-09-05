use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::ArraySeqEphemeral::{ArraySEphemeral, ArraySeqEphemeral};
use apas_ai::ArraySeqEphemeralChap18::ArraySeqEphemeralChap18;
use std::time::Duration;

fn bench_tabulate_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySEphemeral_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySEphemeral<N> = <ArraySEphemeral<N> as ArraySeqEphemeralChap18>::tabulate(|i| i, len);
            let m: ArraySEphemeral<N> = <ArraySEphemeral<N> as ArraySeqEphemeralChap18>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_tabulate_map);
criterion_main!(benches);




