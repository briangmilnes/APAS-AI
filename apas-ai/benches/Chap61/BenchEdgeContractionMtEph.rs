//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 61 Edge Contraction (Multi-threaded)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
    Chap61::EdgeContractionMtEph::EdgeContractionMtEph::*, SetLit, Types::Types::*,
};
use criterion::*;
use std::time::Duration;

fn create_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: Set<usize> = SetLit![];
    let mut edges: Set<Edge<usize>> = SetLit![];

    for i in 0..n {
        let _ = vertices.insert(i);
    }

    for i in 0..n {
        let next = (i + 1) % n;
        let edge = if i < next { Edge(i, next) } else { Edge(next, i) };
        let _ = edges.insert(edge);
    }

    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges)
}

fn bench_edge_contract_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_contract_mt");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for n in [100, 200, 300] {
        let graph = create_cycle_graph(n);
        let mut matching: Set<Edge<usize>> = SetLit![];
        for i in (0..n).step_by(4) {
            let next = (i + 1) % n;
            let edge = if i < next { Edge(i, next) } else { Edge(next, i) };
            let _ = matching.insert(edge);
        }

        group.bench_with_input(BenchmarkId::from_parameter(n), &(graph, matching), |b, (g, m)| {
            b.iter(|| edge_contract_mt(g, m));
        });
    }

    group.finish();
}

fn bench_contract_round_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("contract_round_mt");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for n in [100, 200, 300] {
        let graph = create_cycle_graph(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &graph, |b, g| {
            b.iter(|| contract_round_mt(g, 999));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_edge_contract_mt, bench_contract_round_mt);
criterion_main!(benches);
