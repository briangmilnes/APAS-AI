// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 64 Spanning Tree via Star Contraction (Sequential)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphStEph::UnDirGraphStEph::*,
    Chap64::SpanTreeStEph::SpanTreeStEph::*, SetLit, Types::Types::*,
};
use criterion::*;
use std::time::Duration;

fn create_cycle_graph(n: N) -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges)
}

fn bench_spanning_tree_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("spanning_tree_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[8, 12, 16] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={}", n), |b| {
            b.iter(|| spanning_tree_star_contraction(black_box(&graph)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_spanning_tree_cycle);
criterion_main!(benches);
