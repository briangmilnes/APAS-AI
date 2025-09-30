//! Benchmarks for Edge Set Graph - Persistent Single-Threaded

use apas_ai::Chap52::EdgeSetGraphStPer::EdgeSetGraphStPer::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn bench_insert_vertices(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphStPer");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(30);

    group.bench_function("insert_100_vertices", |b| {
        b.iter(|| {
            let mut g = EdgeSetGraphStPerS::new();
            for i in 0..100usize {
                g = g.insert_vertex(black_box(i));
            }
            black_box(g)
        })
    });

    group.finish();
}

fn bench_insert_edges(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphStPer");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(30);

    // Pre-build graph with 100 vertices
    let mut g = EdgeSetGraphStPerS::new();
    for i in 0..100usize {
        g = g.insert_vertex(i);
    }

    group.bench_function("insert_100_edges", |b| {
        b.iter(|| {
            let mut graph = g.clone();
            for i in 0..100usize {
                let target = (i + 1) % 100;
                graph = graph.insert_edge(black_box(i), black_box(target));
            }
            black_box(graph)
        })
    });

    group.finish();
}

fn bench_has_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphStPer");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(30);

    // Pre-build graph with vertices and edges
    let mut g = EdgeSetGraphStPerS::new();
    for i in 0..100usize {
        g = g.insert_vertex(i);
    }
    for i in 0..100usize {
        g = g.insert_edge(i, (i + 1) % 100);
    }

    group.bench_function("has_edge_100_queries", |b| {
        b.iter(|| {
            let mut count = 0;
            for i in 0..100usize {
                if g.has_edge(&black_box(i), &black_box((i + 1) % 100)) {
                    count += 1;
                }
            }
            black_box(count)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_insert_vertices, bench_insert_edges, bench_has_edge);
criterion_main!(benches);
