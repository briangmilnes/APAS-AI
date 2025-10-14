// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 62 Star Partition (Sequential Ephemeral)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphStEph::UnDirGraphStEph::*,
    Chap62::StarPartitionStEph::StarPartitionStEph::*, SetLit, Types::Types::*,
};
use criterion::*;
use std::time::Duration;

// Helper to create a cycle graph
fn create_cycle_graph(n: N) -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let u = i;
        let v = (i + 1) % n;
        let _ = edges.insert(Edge(u, v));
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges)
}

// Helper to create a dense graph (approximately 50% of possible edges)
fn create_dense_graph(n: N) -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        for j in (i + 1)..n {
            if (i * n + j) % 2 == 0 {
                let _ = edges.insert(Edge(i, j));
            }
        }
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges)
}

fn bench_sequential_star_partition_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_star_partition_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[10, 20, 30] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={n}"), |b| {
            b.iter(|| sequential_star_partition(black_box(&graph)))
        });
    }
    group.finish();
}

fn bench_sequential_star_partition_dense(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_star_partition_dense");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[10, 15, 20] {
        let graph = create_dense_graph(n);
        group.bench_function(format!("n={n}"), |b| {
            b.iter(|| sequential_star_partition(black_box(&graph)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_sequential_star_partition_cycle,
    bench_sequential_star_partition_dense
);
criterion_main!(benches);
