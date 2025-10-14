// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 62 Star Partition (Multi-threaded Ephemeral)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
    Chap62::StarPartitionMtEph::StarPartitionMtEph::*, SetLit, Types::Types::*,
};
use criterion::*;
use std::time::Duration;

fn create_cycle_graph(n: N) -> UnDirGraphMtEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges)
}

fn create_dense_graph(n: N) -> UnDirGraphMtEph<N> {
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
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges)
}

fn bench_parallel_star_partition_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_star_partition_cycle");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[10, 20, 30] {
        let graph = create_cycle_graph(n);
        group.bench_function(format!("n={n}"), |b| {
            b.iter(|| parallel_star_partition(black_box(&graph), 123))
        });
    }
    group.finish();
}

fn bench_parallel_star_partition_dense(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_star_partition_dense");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for &n in &[10, 15, 20] {
        let graph = create_dense_graph(n);
        group.bench_function(format!("n={n}"), |b| {
            b.iter(|| parallel_star_partition(black_box(&graph), 456))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_parallel_star_partition_cycle,
    bench_parallel_star_partition_dense
);
criterion_main!(benches);
