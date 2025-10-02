// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Prim's MST Algorithm Benchmarks (Sequential)

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_ai::Chap65::PrimStEph::PrimStEph::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use criterion::{Criterion, criterion_group, criterion_main};
use ordered_float::OrderedFloat;
use std::time::Duration;

fn create_complete_graph(n: N) -> LabUnDirGraphStEph<N, OrderedFloat<f64>> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        for j in (i + 1)..n {
            let weight = OrderedFloat((i * n + j) as f64);
            let _ = edges.insert(LabEdge(i, j, weight));
        }
    }
    <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges)
}

fn create_sparse_graph(n: N) -> LabUnDirGraphStEph<N, OrderedFloat<f64>> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Create a cycle plus a few random edges
    for i in 0..n {
        let _ = edges.insert(LabEdge(i, (i + 1) % n, OrderedFloat((i + 1) as f64)));
    }
    for i in 0..(n / 4) {
        let _ = edges.insert(LabEdge(i, (i + n / 2) % n, OrderedFloat((i + 10) as f64)));
    }
    <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges)
}

fn bench_prim_mst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Prim_MST");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    // Benchmark: Complete graph (dense)
    group.bench_function("complete_graph_n10", |b| {
        let graph = create_complete_graph(10);
        b.iter(|| {
            let _ = prim_mst(&graph, &0);
        });
    });

    group.bench_function("complete_graph_n15", |b| {
        let graph = create_complete_graph(15);
        b.iter(|| {
            let _ = prim_mst(&graph, &0);
        });
    });

    // Benchmark: Sparse graph
    group.bench_function("sparse_graph_n50", |b| {
        let graph = create_sparse_graph(50);
        b.iter(|| {
            let _ = prim_mst(&graph, &0);
        });
    });

    group.bench_function("sparse_graph_n100", |b| {
        let graph = create_sparse_graph(100);
        b.iter(|| {
            let _ = prim_mst(&graph, &0);
        });
    });

    // Benchmark: MST weight calculation
    group.bench_function("mst_weight_n100", |b| {
        let graph = create_sparse_graph(100);
        let mst = prim_mst(&graph, &0);
        b.iter(|| {
            let _ = mst_weight(&mst);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_prim_mst);
criterion_main!(benches);
