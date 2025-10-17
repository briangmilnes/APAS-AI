//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
use apas_ai::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
use apas_ai::Types::Types::*;

fn bench_weighted_dir_graph_mt_eph_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedDirGraphMtEphFloat");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 500;
    group.bench_with_input(BenchmarkId::new("build_graph", n), &n, |b, &len| {
        b.iter(|| {
            let mut vertices = SetStEph::empty();
            for i in 0..len {
                vertices.insert(i);
            }
            let mut edges = SetStEph::empty();
            for i in 0..len {
                for j in 0..3 {
                    let target = (i + j + 1) % len;
                    edges.insert(Triple(i, target, OrderedF64::from(i as f64 * 0.5)));
                }
            }
            let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
            black_box(g)
        })
    });

    group.bench_with_input(BenchmarkId::new("edge_operations", n), &n, |b, &len| {
        let mut vertices = SetStEph::empty();
        for i in 0..len {
            vertices.insert(i);
        }
        let mut edges = SetStEph::empty();
        for i in 0..len {
            let target = (i + 1) % len;
            edges.insert(Triple(i, target, OrderedF64::from(i as f64 * 0.1)));
        }
        let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

        b.iter(|| {
            let mut total_weight = 0.0;
            for i in 0..len {
                let target = (i + 1) % len;
                if let Some(weight) = g.get_edge_weight(&i, &target) {
                    total_weight += weight.into_inner();
                }
            }
            black_box(total_weight)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_weighted_dir_graph_mt_eph_float);
criterion_main!(benches);
