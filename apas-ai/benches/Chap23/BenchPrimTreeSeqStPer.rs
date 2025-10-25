//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;
use apas_ai::Types::Types::*;

fn bench_prim_tree_seq_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimTreeSeqSt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("build_sequence", n), &n, |b, &len| {
        b.iter(|| {
            let mut data = Vec::new();
            for i in 0..len {
                data.push(i);
            }
            let seq = PrimTreeSeqStS::from_vec(data);
            black_box(seq)
        })
    });

    group.bench_with_input(BenchmarkId::new("expose_join", n), &n, |b, &len| {
        let mut data = Vec::new();
        for i in 0..len {
            data.push(i);
        }
        let seq = PrimTreeSeqStS::from_vec(data);
        b.iter(|| {
            let exposed = seq.expose();
            let rejoined = match exposed {
                | PrimTreeSeqStTree::Zero => PrimTreeSeqStS::empty(),
                | PrimTreeSeqStTree::One(val) => PrimTreeSeqStS::singleton(val),
                | PrimTreeSeqStTree::Two(left, right) => PrimTreeSeqStS::join(PrimTreeSeqStTree::Two(left, right)),
            };
            black_box(rejoined)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_prim_tree_seq_st);
criterion_main!(benches);
