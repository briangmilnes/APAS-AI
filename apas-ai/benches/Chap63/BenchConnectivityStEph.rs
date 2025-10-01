// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 63 Graph Connectivity (Sequential Ephemeral)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*,
    Chap06::UnDirGraphStEph::UnDirGraphStEph::*,
    Chap63::ConnectivityStEph::ConnectivityStEph::*,
    SetLit,
    Types::Types::*,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

fn create_multi_component_graph(n_components: N, component_size: N) -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    let mut edges = SetLit![];
    
    for comp in 0..n_components {
        let base = comp * component_size;
        for i in 0..component_size {
            let _ = vertices.insert(base + i);
            if i > 0 {
                let _ = edges.insert(Edge(base + i - 1, base + i));
            }
        }
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges)
}

fn bench_count_components_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_components_single");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[10, 15, 20] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={}", n), |b| {
            b.iter(|| count_components(black_box(&graph)))
        });
    }
    group.finish();
}

fn bench_count_components_multiple(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_components_multiple");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n_comp in &[3, 5, 8] {
        let graph = create_multi_component_graph(n_comp, 3);
        group.bench_function(format!("comp={}", n_comp), |b| {
            b.iter(|| count_components(black_box(&graph)))
        });
    }
    group.finish();
}

fn bench_connected_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("connected_components");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n_comp in &[3, 5, 8] {
        let graph = create_multi_component_graph(n_comp, 3);
        group.bench_function(format!("comp={}", n_comp), |b| {
            b.iter(|| connected_components(black_box(&graph)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_count_components_single,
    bench_count_components_multiple,
    bench_connected_components
);
criterion_main!(benches);

