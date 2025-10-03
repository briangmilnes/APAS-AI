// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 64 Spanning Tree via Star Contraction (Parallel)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
    Chap64::SpanTreeMtEph::SpanTreeMtEph::*, SetLit, Types::Types::*,
};
use criterion::*;
use std::time::Duration;

fn create_cycle_graph(n: N) -> UnDirGraphMtEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges)
}

fn bench_spanning_tree_mt_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("spanning_tree_mt_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[8, 12, 16] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={}", n), |b| {
            b.iter(|| spanning_tree_star_contraction_mt(black_box(&graph), 123))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_spanning_tree_mt_cycle);
criterion_main!(benches);
