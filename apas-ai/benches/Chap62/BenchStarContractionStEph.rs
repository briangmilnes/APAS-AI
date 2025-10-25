//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 62 Star Contraction (Sequential Ephemeral)

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_ai::Chap62::StarContractionStEph::StarContractionStEph::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
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

fn bench_contract_to_vertices_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("contract_to_vertices_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[8, 12, 16] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={n}"), |b| b.iter(|| contract_to_vertices(black_box(&graph))));
    }
    group.finish();
}

criterion_group!(benches, bench_contract_to_vertices_cycle);
criterion_main!(benches);
