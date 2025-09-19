use apas_ai::LinkedListStEph::LinkedListStEph::*;
use apas_ai::LinkedListStEphChap18Trait;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_ll_eph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListEphChap18");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::tabulate(|i| i, len);
            let m: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphChap18Trait<N>>::map(&s, |x| x + 1);
            black_box((
                <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::length(&s),
                <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::length(&m),
            ))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_eph_ch18);
criterion_main!(benches);
