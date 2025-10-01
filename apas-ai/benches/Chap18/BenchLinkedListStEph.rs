//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap18::LinkedListStEph::LinkedListStEph::*;
use apas_ai::LinkedListStEphSLit;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_ll_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListEph_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("new_then_set", n), &n, |b, &len| {
        b.iter(|| {
            let mut s: LinkedListStEphS<N> = LinkedListStEphSLit![0; len]; // *Eph: constructor + set pattern
            for i in 0..len {
                let _ = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::set(&mut s, i, i);
            }
            black_box(<LinkedListStEphS<N> as LinkedListStEphTrait<N>>::length(&s))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_eph);
criterion_main!(benches);
