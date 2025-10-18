//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Bellman-Ford Algorithm with float weights

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
use apas_ai::Chap58::BellmanFordStEphFloat::BellmanFordStEphFloat::bellman_ford;
use apas_ai::SetLit;
use apas_ai::Types::Types::OrderedF64;
use apas_ai::Types::Types::*;

fn create_sparse_graph(n: usize) -> WeightedDirGraphStEphFloat<usize> {
    let mut vertices = SetLit![];
    for v in 0..n {
        vertices.insert(v);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        let j = (i + 1) % n;
        edges.insert(Triple(i, j, OrderedF64::from(1.0)));
        if i + 2 < n {
            edges.insert(Triple(i, i + 2, OrderedF64::from(2.5)));
        }
    }

    WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges)
}

fn create_dense_graph(n: usize) -> WeightedDirGraphStEphFloat<usize> {
    let mut vertices = SetLit![];
    for v in 0..n {
        vertices.insert(v);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        for j in 0..n {
            if i != j && (i + j) % 2 == 0 {
                edges.insert(Triple(i, j, OrderedF64::from(((i + j) % 10 + 1) as f64 + 0.5)));
            }
        }
    }

    WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges)
}

fn create_negative_edges_graph(n: usize) -> WeightedDirGraphStEphFloat<usize> {
    let mut vertices = SetLit![];
    for v in 0..n {
        vertices.insert(v);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        let j = (i + 1) % n;
        let weight = if i % 2 == 0 { 2.5 } else { -1.0 };
        edges.insert(Triple(i, j, OrderedF64::from(weight)));
    }

    WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges)
}

fn bench_sparse_graphs(c: &mut Criterion) {
    let mut group = c.benchmark_group("BellmanFord Float Sparse");
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
    let mut group = c.benchmark_group("BellmanFord Float Dense");
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
    let mut group = c.benchmark_group("BellmanFord Float Negative");
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
