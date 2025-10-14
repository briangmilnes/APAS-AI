//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Types::Types::*;
use apas_ai::Chap18::LinkedListStPer::LinkedListStPer::*;

fn bench_ll_per_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("append_then_iterate", n), &n, |b, &len| {
        b.iter(|| {
            let a: LinkedListStPerS<N> = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i, len);
            let b2: LinkedListStPerS<N> = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i + len, len);
            let c = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::append(&a, &b2);
            black_box(<LinkedListStPerS<N> as LinkedListStPerTrait<N>>::length(&c))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_per_ch19);
criterion_main!(benches);
