// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 64 TSP 2-Approximation (Parallel)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*,
    Chap64::TSPApproxMtEph::TSPApproxMtEph::*, SetLit, Types::Types::*,
};
use criterion::*;
use ordered_float::OrderedFloat;
use std::time::Duration;

fn create_complete_graph(
    n: N,
) -> (
    LabUnDirGraphMtEph<N, OrderedFloat<f64>>,
    Set<LabEdge<N, OrderedFloat<f64>>>,
) {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }

    let mut edges = SetLit![];
    for i in 0..n {
        for j in (i + 1)..n {
            let weight = OrderedFloat(((i + j + 1) as f64) * 1.5);
            let _ = edges.insert(LabEdge(i, j, weight));
        }
    }

    let graph = <LabUnDirGraphMtEph<N, OrderedFloat<f64>> as LabUnDirGraphMtEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges.clone());

    let mut spanning_tree = SetLit![];
    for i in 0..(n - 1) {
        let weight = OrderedFloat(((i + i + 1 + 1) as f64) * 1.5);
        let _ = spanning_tree.insert(LabEdge(i, i + 1, weight));
    }

    (graph, spanning_tree)
}

fn bench_tsp_approx_mt_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("tsp_approx_mt");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[5, 7, 9] {
        let (graph, tree) = create_complete_graph(n);
        group.bench_function(format!("n={}", n), |b| {
            b.iter(|| approx_metric_tsp_mt(black_box(&graph), black_box(&tree), &0))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_tsp_approx_mt_small);
criterion_main!(benches);
