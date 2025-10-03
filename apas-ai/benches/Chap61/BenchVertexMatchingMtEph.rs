//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 61 Vertex Matching (Multi-threaded)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
    Chap61::VertexMatchingMtEph::VertexMatchingMtEph::*, SetLit, Types::Types::*,
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

fn create_star_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: Set<usize> = SetLit![];
    let mut edges: Set<Edge<usize>> = SetLit![];

    let _ = vertices.insert(0);
    for i in 1..=n {
        let _ = vertices.insert(i);
        let _ = edges.insert(Edge(0, i));
    }

    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges)
}

fn bench_parallel_matching_mt_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_matching_mt_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for n in [100, 200, 300] {
        let graph = create_cycle_graph(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &graph, |b, g| {
            b.iter(|| parallel_matching_mt(g, 789));
        });
    }

    group.finish();
}

fn bench_parallel_matching_mt_star(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_matching_mt_star");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for n in [50, 100, 150] {
        let graph = create_star_graph(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &graph, |b, g| {
            b.iter(|| parallel_matching_mt(g, 456));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parallel_matching_mt_cycle,
    bench_parallel_matching_mt_star
);
criterion_main!(benches);
