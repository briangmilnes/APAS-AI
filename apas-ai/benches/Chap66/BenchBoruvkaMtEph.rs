// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 66: Borůvka's MST Algorithm (Parallel Ephemeral)

use apas_ai::Chap66::BoruvkaMtEph::BoruvkaMtEph::*;
use apas_ai::SetLit;
use criterion::*;
use ordered_float::OrderedFloat;
use std::time::Duration;

fn bench_boruvka_mt_complete(c: &mut Criterion) {
    let mut group = c.benchmark_group("boruvka_mt_complete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for n in [8, 10, 12] {
        let vertices: Vec<usize> = (0..n).collect();
        let mut edges = SetLit![];
        let mut label = 0;

        // Complete graph with random-ish weights
        for i in 0..n {
            for j in (i + 1)..n {
                let weight = OrderedFloat(((i * 7 + j * 11) % 20 + 1) as f64);
                let _ = edges.insert((i, j, weight, label));
                label += 1;
            }
        }

        let mut vertices_set = SetLit![];
        for v in vertices.iter() {
            let _ = vertices_set.insert(*v);
        }

        group.bench_function(format!("n{n}"), |b| {
            b.iter(|| {
                let mst = boruvka_mst_mt_with_seed(black_box(&vertices_set), black_box(&edges), 42);
                black_box(mst)
            });
        });
    }

    group.finish();
}

fn bench_boruvka_mt_sparse(c: &mut Criterion) {
    let mut group = c.benchmark_group("boruvka_mt_sparse");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for n in [10, 15, 20] {
        let vertices: Vec<usize> = (0..n).collect();
        let mut edges = SetLit![];
        let mut label = 0;

        // Sparse graph: ~2 edges per vertex
        for i in 0..n {
            let j = (i + 1) % n;
            let k = (i + 2) % n;
            let w1 = OrderedFloat(((i * 7) % 10 + 1) as f64);
            let w2 = OrderedFloat(((i * 11) % 10 + 1) as f64);
            let _ = edges.insert((i, j, w1, label));
            label += 1;
            let _ = edges.insert((i, k, w2, label));
            label += 1;
        }

        let mut vertices_set = SetLit![];
        for v in vertices.iter() {
            let _ = vertices_set.insert(*v);
        }

        group.bench_function(format!("n{n}"), |b| {
            b.iter(|| {
                let mst = boruvka_mst_mt_with_seed(black_box(&vertices_set), black_box(&edges), 42);
                black_box(mst)
            });
        });
    }

    group.finish();
}

fn bench_boruvka_mt_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("boruvka_mt_path");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for n in [20, 25, 30] {
        let vertices: Vec<usize> = (0..n).collect();
        let mut edges = SetLit![];

        // Path graph
        for i in 0..(n - 1) {
            let weight = OrderedFloat((i % 10 + 1) as f64);
            let _ = edges.insert((i, i + 1, weight, i));
        }

        let mut vertices_set = SetLit![];
        for v in vertices.iter() {
            let _ = vertices_set.insert(*v);
        }

        group.bench_function(format!("n{n}"), |b| {
            b.iter(|| {
                let mst = boruvka_mst_mt_with_seed(black_box(&vertices_set), black_box(&edges), 42);
                black_box(mst)
            });
        });
    }

    group.finish();
}

fn bench_boruvka_mt_star(c: &mut Criterion) {
    let mut group = c.benchmark_group("boruvka_mt_star");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for n in [20, 30, 40] {
        let vertices: Vec<usize> = (0..n).collect();
        let mut edges = SetLit![];

        // Star graph: center 0 connected to all others
        for i in 1..n {
            let weight = OrderedFloat((i % 10 + 1) as f64);
            let _ = edges.insert((0, i, weight, i - 1));
        }

        let mut vertices_set = SetLit![];
        for v in vertices.iter() {
            let _ = vertices_set.insert(*v);
        }

        group.bench_function(format!("n{n}"), |b| {
            b.iter(|| {
                let mst = boruvka_mst_mt_with_seed(black_box(&vertices_set), black_box(&edges), 42);
                black_box(mst)
            });
        });
    }

    group.finish();
}

fn bench_mst_weight_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("mst_weight_mt");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let n = 15;
    let vertices: Vec<usize> = (0..n).collect();
    let mut edges = SetLit![];
    let mut label = 0;

    // Complete graph
    for i in 0..n {
        for j in (i + 1)..n {
            let weight = OrderedFloat(((i * 7 + j * 11) % 20 + 1) as f64);
            let _ = edges.insert((i, j, weight, label));
            label += 1;
        }
    }

    let mut vertices_set = SetLit![];
    for v in vertices.iter() {
        let _ = vertices_set.insert(*v);
    }
    let mst_labels = boruvka_mst_mt_with_seed(&vertices_set, &edges, 42);

    group.bench_function("n15", |b| {
        b.iter(|| {
            let w = mst_weight(black_box(&edges), black_box(&mst_labels));
            black_box(w)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_boruvka_mt_complete,
    bench_boruvka_mt_sparse,
    bench_boruvka_mt_path,
    bench_boruvka_mt_star,
    bench_mst_weight_mt
);
criterion_main!(benches);
