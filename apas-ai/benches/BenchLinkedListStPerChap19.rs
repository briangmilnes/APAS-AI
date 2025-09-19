use apas_ai::LinkedListStPer::LinkedListStPer::LinkedListStPerS;
use apas_ai::LinkedListStPer::LinkedListStPer::*;
use apas_ai::LinkedListStPerChap19Trait;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_ll_per_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListPerChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("append_then_iterate", n), &n, |b, &len| {
        b.iter(|| {
            let a: LinkedListStPerS<N> = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::tabulate(|i| i, len);
            let b2: LinkedListStPerS<N> =
                <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::tabulate(|i| i + len, len);
            let c = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::append(&a, &b2);
            black_box(<LinkedListStPerS<N> as LinkedListStPerTrait<N>>::length(&c))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_per_ch19);
criterion_main!(benches);
