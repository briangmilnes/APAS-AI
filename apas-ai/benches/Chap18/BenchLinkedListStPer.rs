//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::LinkedListStPer::LinkedListStPer::*;
use apas_ai::LinkedListStPerTrait;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_ll_per_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListPer_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: LinkedListStPerS<N> = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(|i| i, len);
            let m: LinkedListStPerS<N> = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_per_ch18);
criterion_main!(benches);
