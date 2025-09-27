//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::{LabDirGraphStEphLit, SetLit};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_labelled_dir_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_Creation");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("empty", size), size, |b, _| {
            b.iter(|| black_box(LabDirGraphStEph::<i32, &str>::empty()))
        });

        group.bench_with_input(
            BenchmarkId::new("from_vertices_and_labeled_arcs", size),
            size,
            |b, &_size| {
                let vertices = SetLit![0, 1, 2, 3, 4];
                let labeled_arcs = SetLit![LabEdge(0, 1, "a"), LabEdge(1, 2, "b"), LabEdge(2, 3, "c")];
                b.iter(|| {
                    black_box(LabDirGraphStEph::from_vertices_and_labeled_arcs(
                        vertices.clone(),
                        labeled_arcs.clone(),
                    ))
                })
            },
        );
    }

    group.finish();
}

fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_AddVertex");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("add_vertex", size), size, |b, &size| {
            b.iter(|| {
                let mut g = LabDirGraphStEph::<i32, &str>::empty();
                for i in 0..size {
                    g.add_vertex(black_box(i));
                }
                black_box(g)
            })
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_AddLabeledArc");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("add_labeled_arc", size), size, |b, &size| {
            b.iter(|| {
                let mut g = LabDirGraphStEph::<i32, &str>::empty();
                for i in 0..size {
                    g.add_labeled_arc(black_box(i), black_box((i + 1) % size), black_box("label"));
                }
                black_box(g)
            })
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_HasArc");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("has_arc", size), size, |b, &size| {
            let mut g = LabDirGraphStEph::<i32, &str>::empty();
            // Setup graph with arcs
            for i in 0..size {
                g.add_labeled_arc(i, (i + 1) % size, "test");
            }

            b.iter(|| {
                let mut result = true;
                for i in 0..size {
                    result &= g.has_arc(black_box(&i), black_box(&((i + 1) % size)));
                }
                black_box(result)
            })
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_GetArcLabel");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("get_arc_label", size), size, |b, &size| {
            let mut g = LabDirGraphStEph::<i32, &str>::empty();
            // Setup graph with labeled arcs
            for i in 0..size {
                g.add_labeled_arc(i, (i + 1) % size, "test_label");
            }

            b.iter(|| {
                let mut labels = Vec::new();
                for i in 0..size {
                    if let Some(label) = g.get_arc_label(black_box(&i), black_box(&((i + 1) % size))) {
                        labels.push(label);
                    }
                }
                black_box(labels)
            })
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_OutNeighbors");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("out_neighbors", size), size, |b, &size| {
            let mut g = LabDirGraphStEph::<i32, &str>::empty();
            // Setup graph - each vertex connects to next 3 vertices (with wraparound)
            for i in 0..size {
                for j in 1..=3 {
                    g.add_labeled_arc(i, (i + j) % size, "edge");
                }
            }

            b.iter(|| {
                let mut all_neighbors = Vec::new();
                for i in 0..size {
                    let neighbors = g.out_neighbors(black_box(&i));
                    all_neighbors.push(neighbors);
                }
                black_box(all_neighbors)
            })
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_InNeighbors");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("in_neighbors", size), size, |b, &size| {
            let mut g = LabDirGraphStEph::<i32, &str>::empty();
            // Setup graph - each vertex connects to next 3 vertices (with wraparound)
            for i in 0..size {
                for j in 1..=3 {
                    g.add_labeled_arc(i, (i + j) % size, "edge");
                }
            }

            b.iter(|| {
                let mut all_neighbors = Vec::new();
                for i in 0..size {
                    let neighbors = g.in_neighbors(black_box(&i));
                    all_neighbors.push(neighbors);
                }
                black_box(all_neighbors)
            })
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_Arcs");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("arcs", size), size, |b, &size| {
            let mut g = LabDirGraphStEph::<i32, &str>::empty();
            // Setup graph with arcs
            for i in 0..size {
                g.add_labeled_arc(i, (i + 1) % size, "test");
            }

            b.iter(|| black_box(g.arcs()))
        });
    }

    group.finish();
}

fn bench_labelled_dir_graph_macro(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabDirGraphStEph_Macro");

    group.bench_function("macro_empty", |b| {
        b.iter(|| {
            let g: LabDirGraphStEph<i32, &str> = LabDirGraphStEphLit!();
            black_box(g)
        })
    });

    group.bench_function("macro_small", |b| {
        b.iter(|| {
            black_box(LabDirGraphStEphLit!(
                V: [1, 2, 3, 4, 5],
                A: [(1, 2, "a"), (2, 3, "b"), (3, 4, "c"), (4, 5, "d"), (5, 1, "e")]
            ))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_labelled_dir_graph_creation,
    bench_labelled_dir_graph_add_vertex,
    bench_labelled_dir_graph_add_labeled_arc,
    bench_labelled_dir_graph_has_arc,
    bench_labelled_dir_graph_get_arc_label,
    bench_labelled_dir_graph_out_neighbors,
    bench_labelled_dir_graph_in_neighbors,
    bench_labelled_dir_graph_arcs,
    bench_labelled_dir_graph_macro
);
criterion_main!(benches);
