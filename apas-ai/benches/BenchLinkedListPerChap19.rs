use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::LinkedListPer::LinkedListPer::LinkedListPerS;
use apas_ai::LinkedListPerChap19::LinkedListPerChap19Trait;
use apas_ai::LinkedListPer::LinkedListPer::LinkedListPerTrait;
use std::time::Duration;

fn bench_ll_per_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListPerChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("append_then_iterate", n), &n, |b, &len| {
        b.iter(|| {
            let a: LinkedListPerS<N> = <LinkedListPerS<N> as LinkedListPerChap19Trait>::tabulate(|i| i, len);
            let b2: LinkedListPerS<N> = <LinkedListPerS<N> as LinkedListPerChap19Trait>::tabulate(|i| i + len, len);
            let c = <LinkedListPerS<N> as LinkedListPerChap19Trait>::append(&a, &b2);
            black_box(<LinkedListPerS<N> as LinkedListPerTrait<N>>::length(&c))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_per_ch19);
criterion_main!(benches);


