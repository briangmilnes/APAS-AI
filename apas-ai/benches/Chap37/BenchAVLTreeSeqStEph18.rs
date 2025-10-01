//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::AVLTreeSeqStEphLit;
use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_avl_eph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("push_then_nth", n), &n, |b, &len| {
        b.iter(|| {
            let mut t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![]; // *Eph: constructor pattern
            for i in 0..len {
                t.push_back(i);
            }
            let v = *<AVLTreeSeqStEphS<N> as AVLTreeSeqStEphTrait<N>>::nth(&t, len - 1);
            black_box((<AVLTreeSeqStEphS<N> as AVLTreeSeqStEphTrait<N>>::length(&t), v))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_avl_eph_ch18);
criterion_main!(benches);
