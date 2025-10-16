//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Bellman-Ford Algorithm with integer weights

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use apas_ai::Chap05::SetStEph::SetStEph;
use apas_ai::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt;
use apas_ai::Chap58::BellmanFordStEphInt::BellmanFordStEphInt::bellman_ford;
use apas_ai::SetLit;

fn create_sparse_graph(n: usize) -> WeightedDirGraphStEphInt<usize> {
    let mut vertices = SetLit![];
    for v in 0..n {
        vertices.insert(v);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        let j = (i + 1) % n;
        edges.insert((i, j, 1));
        if i + 2 < n {
            edges.insert((i, i + 2, 2));
        }
    }

    WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges)
}

fn create_dense_graph(n: usize) -> WeightedDirGraphStEphInt<usize> {
    let mut vertices = SetLit![];
    for v in 0..n {
        vertices.insert(v);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        for j in 0..n {
            if i != j && (i + j) % 2 == 0 {
                edges.insert((i, j, ((i + j) % 10 + 1) as i32));
            }
        }
    }

    WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges)
}

fn create_negative_edges_graph(n: usize) -> WeightedDirGraphStEphInt<usize> {
    let mut vertices = SetLit![];
    for v in 0..n {
        vertices.insert(v);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        let j = (i + 1) % n;
        let weight = if i % 2 == 0 { 2 } else { -1 };
        edges.insert((i, j, weight));
    }

    WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges)
}

fn bench_sparse_graphs(c: &mut Criterion) {
    let mut group = c.benchmark_group("BellmanFord Int Sparse");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20, 30].iter() {
        let graph = create_sparse_graph(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| black_box(bellman_ford(&graph, 0)))
        });
    }

    group.finish();
}

fn bench_dense_graphs(c: &mut Criterion) {
    let mut group = c.benchmark_group("BellmanFord Int Dense");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20, 30].iter() {
        let graph = create_dense_graph(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| black_box(bellman_ford(&graph, 0)))
        });
    }

    group.finish();
}

fn bench_negative_edges(c: &mut Criterion) {
    let mut group = c.benchmark_group("BellmanFord Int Negative");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20].iter() {
        let graph = create_negative_edges_graph(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| black_box(bellman_ford(&graph, 0)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sparse_graphs, bench_dense_graphs, bench_negative_edges);

criterion_main!(benches);
