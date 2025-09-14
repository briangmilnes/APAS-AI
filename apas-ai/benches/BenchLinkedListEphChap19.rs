use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::LinkedListEph::LinkedListEph::*;
use apas_ai::LinkedListEphChap19::LinkedListEphChap19Trait;
use std::time::Duration;

fn bench_ll_eph_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListEphChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap19Trait>::tabulate(|i| i, len);
            let m: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap19Trait>::map(&s, |x| x + 1);
            black_box((<LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&s), <LinkedListEphS<N> as LinkedListEphTrait<N>>::length(&m)))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_eph_ch19);
criterion_main!(benches);
