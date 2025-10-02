//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 61 Vertex Matching (Sequential)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphStEph::UnDirGraphStEph::*,
    Chap61::VertexMatchingStEph::VertexMatchingStEph::*, SetLit, Types::Types::*,
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn create_cycle_graph(n: usize) -> UnDirGraphStEph<usize> {
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

    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::FromSets(vertices, edges)
}

fn bench_greedy_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("greedy_matching");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for n in [100, 200, 300] {
        let graph = create_cycle_graph(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &graph, |b, g| {
            b.iter(|| greedy_matching(g));
        });
    }

    group.finish();
}

fn bench_parallel_matching_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_matching_st");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for n in [100, 200, 300] {
        let graph = create_cycle_graph(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &graph, |b, g| {
            b.iter(|| parallel_matching_st(g, 42));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_greedy_matching, bench_parallel_matching_st);
criterion_main!(benches);
