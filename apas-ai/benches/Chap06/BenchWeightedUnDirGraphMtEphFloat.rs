//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
use apas_ai::Chap06::WeightedUnDirGraphMtEphFloat::WeightedUnDirGraphMtEphFloat::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_weighted_undir_graph_mt_eph_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedUnDirGraphMtEphFloat");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
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
                    edges.insert((i, target, OrderedF64::from(i as f64 * 0.5)));
                }
            }
            let g = WeightedUnDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_weighted_undir_graph_mt_eph_float);
criterion_main!(benches);
