//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::{LabUnDirGraphStEphLit, SetLit};
use criterion::*;
use std::time::Duration;

fn bench_labelled_undir_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_Creation");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("empty", size), size, |b, _| {
            b.iter(|| black_box(LabUnDirGraphStEph::<i32, &str>::empty()))
        });

        group.bench_with_input(
            BenchmarkId::new("from_vertices_and_labeled_edges", size),
            size,
            |b, &_size| {
                let vertices = SetLit![0, 1, 2, 3, 4];
                let labeled_edges = SetLit![LabEdge(0, 1, "a"), LabEdge(1, 2, "b"), LabEdge(2, 3, "c")];
                b.iter(|| {
                    black_box(LabUnDirGraphStEph::from_vertices_and_labeled_edges(
                        vertices.clone(),
                        labeled_edges.clone(),
                    ))
                })
            },
        );
    }

    group.finish();
}

fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_AddVertex");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("add_vertex", size), size, |b, &size| {
            b.iter(|| {
                let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
                for i in 0..size {
                    g.add_vertex(black_box(i));
                }
                black_box(g)
            })
        });
    }

    group.finish();
}

fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_AddLabeledEdge");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("add_labeled_edge", size), size, |b, &size| {
            b.iter(|| {
                let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
                for i in 0..size {
                    g.add_labeled_edge(black_box(i), black_box((i + 1) % size), black_box("label"));
                }
                black_box(g)
            })
        });
    }

    group.finish();
}

fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_HasEdge");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("has_edge", size), size, |b, &size| {
            let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
            // Setup graph with edges
            for i in 0..size {
                g.add_labeled_edge(i, (i + 1) % size, "test");
            }

            b.iter(|| {
                let mut result = true;
                for i in 0..size {
                    result &= g.has_edge(black_box(&i), black_box(&((i + 1) % size)));
                }
                black_box(result)
            })
        });
    }

    group.finish();
}

fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_GetEdgeLabel");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("get_edge_label", size), size, |b, &size| {
            let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
            // Setup graph with labeled edges
            for i in 0..size {
                g.add_labeled_edge(i, (i + 1) % size, "test_label");
            }

            b.iter(|| {
                let mut labels = Vec::new();
                for i in 0..size {
                    if let Some(label) = g.get_edge_label(black_box(&i), black_box(&((i + 1) % size))) {
                        labels.push(label);
                    }
                }
                black_box(labels)
            })
        });
    }

    group.finish();
}

fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_Neighbors");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("neighbors", size), size, |b, &size| {
            let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
            // Setup graph - each vertex connects to next 3 vertices (with wraparound)
            for i in 0..size {
                for j in 1..=3 {
                    if i < (i + j) % size || (i + j) >= size {
                        // Avoid duplicate edges in undirected graph
                        g.add_labeled_edge(i, (i + j) % size, "edge");
                    }
                }
            }

            b.iter(|| {
                let mut all_neighbors = Vec::new();
                for i in 0..size {
                    let neighbors = g.neighbors(black_box(&i));
                    all_neighbors.push(neighbors);
                }
                black_box(all_neighbors)
            })
        });
    }

    group.finish();
}

fn bench_labelled_undir_graph_edges(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_Edges");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("edges", size), size, |b, &size| {
            let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
            // Setup graph with edges
            for i in 0..size {
                g.add_labeled_edge(i, (i + 1) % size, "test");
            }

            b.iter(|| black_box(g.edges()))
        });
    }

    group.finish();
}

fn bench_labelled_undir_graph_macro(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_Macro");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    group.bench_function("macro_empty", |b| {
        b.iter(|| {
            let g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEphLit!();
            black_box(g)
        })
    });

    group.bench_function("macro_small", |b| {
        b.iter(|| {
            black_box(LabUnDirGraphStEphLit!(
                V: [1, 2, 3, 4, 5],
                E: [(1, 2, "a"), (2, 3, "b"), (3, 4, "c"), (4, 5, "d"), (1, 5, "e")]
            ))
        })
    });

    group.finish();
}

fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEph_EdgeNormalization");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [10, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("add_edges_both_directions", size), size, |b, &size| {
            b.iter(|| {
                let mut g = LabUnDirGraphStEph::<i32, &str>::empty();
                for i in 0..size {
                    // Add edges in both directions to test normalization
                    let next = (i + 1) % size;
                    if i % 2 == 0 {
                        g.add_labeled_edge(black_box(i), black_box(next), black_box("test"));
                    } else {
                        g.add_labeled_edge(black_box(next), black_box(i), black_box("test"));
                    }
                }
                black_box(g)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_labelled_undir_graph_creation,
    bench_labelled_undir_graph_add_vertex,
    bench_labelled_undir_graph_add_labeled_edge,
    bench_labelled_undir_graph_has_edge,
    bench_labelled_undir_graph_get_edge_label,
    bench_labelled_undir_graph_neighbors,
    bench_labelled_undir_graph_edges,
    bench_labelled_undir_graph_macro,
    bench_labelled_undir_graph_edge_normalization
);
criterion_main!(benches);
