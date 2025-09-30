use apas_ai::Chap52::EdgeSetGraphStPer::EdgeSetGraphStPer::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_insert_vertices(c: &mut Criterion) {
    c.bench_function("EdgeSetGraphStPer_insert_vertices_100", |b| {
        b.iter(|| {
            let mut g = EdgeSetGraphStPerS::new();
            for i in 0..100 {
                g = g.insert_vertex(black_box(i));
            }
            g
        });
    });
}

fn bench_insert_edges(c: &mut Criterion) {
    c.bench_function("EdgeSetGraphStPer_insert_edges_100", |b| {
        b.iter(|| {
            let mut g = EdgeSetGraphStPerS::new();
            for i in 0..100 {
                g = g.insert_vertex(i);
            }
            for i in 0..99 {
                g = g.insert_edge(black_box(i), black_box(i + 1));
            }
            g
        });
    });
}

fn bench_has_edge(c: &mut Criterion) {
    let mut g = EdgeSetGraphStPerS::new();
    for i in 0..100 {
        g = g.insert_vertex(i);
    }
    for i in 0..99 {
        g = g.insert_edge(i, i + 1);
    }
    
    c.bench_function("EdgeSetGraphStPer_has_edge", |b| {
        b.iter(|| {
            for i in 0..99 {
                black_box(g.has_edge(&i, &(i + 1)));
            }
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(3))
        .sample_size(30);
    targets = bench_insert_vertices, bench_insert_edges, bench_has_edge
}
criterion_main!(benches);