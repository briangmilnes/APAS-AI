//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
use apas_ai::Chap06::WeightedUnDirGraphMtEphInt::WeightedUnDirGraphMtEphInt::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_weighted_undir_graph_mt_eph_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedUnDirGraphMtEphInt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 500;
    group.bench_with_input(BenchmarkId::new("build_graph", n), &n, |b, &len| {
        b.iter(|| {
            let mut vertices = Set::empty();
            for i in 0..len {
                vertices.insert(i);
            }
            let mut edges = Set::empty();
            for i in 0..len {
                for j in 0..3 {
                    let target = (i + j + 1) % len;
                    edges.insert((i, target, (i + j) as i32));
                }
            }
            let g = WeightedUnDirGraphMtEphInt::from_weighted_edges(vertices, edges);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_weighted_undir_graph_mt_eph_int);
criterion_main!(benches);
