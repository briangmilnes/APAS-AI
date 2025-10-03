// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 62 Star Contraction (Multi-threaded Ephemeral)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
    Chap62::StarContractionMtEph::StarContractionMtEph::*, SetLit, Types::Types::*,
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

fn bench_contract_to_vertices_mt_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("contract_to_vertices_mt_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[8, 12, 16] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={}", n), |b| {
            b.iter(|| contract_to_vertices_mt(black_box(&graph), 789))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_contract_to_vertices_mt_cycle);
criterion_main!(benches);
