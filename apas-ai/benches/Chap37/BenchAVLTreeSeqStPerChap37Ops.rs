//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Types::Types::*;

fn bench_avl_per_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqPer_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("build_then_length", n), &n, |b, &len| {
        b.iter(|| {
            // Build sequence incrementally: set(0,0) on empty, then set(1,1), set(2,2), etc.
            let mut t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::empty();
            for i in 0..len {
                // set(i, i) appends when i == length()
                t = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::set(&t, i, i).unwrap();
            }
            black_box(t.length())
        })
    });
    group.finish();
}

criterion_group!(benches, bench_avl_per_ch18);
criterion_main!(benches);
