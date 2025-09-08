use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::{LinkedListEphS, LinkedListEphTrait};
use std::time::Duration;

fn bench_ll_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListEph_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("new_then_set", n), &n, |b, &len| {
        b.iter(|| {
            let mut s: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(len, 0);
            for i in 0..len { let _ = <LinkedListEphS<N> as LinkedListEphTrait<N>>::set(&mut s, i, i); }
            black_box(<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&s))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_eph);
criterion_main!(benches);


