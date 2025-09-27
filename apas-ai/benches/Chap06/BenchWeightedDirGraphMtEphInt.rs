//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
use apas_ai::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_weighted_dir_graph_mt_eph_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedDirGraphMtEphInt");
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
                    edges.insert((i, target, (i + j) as i32));
                }
            }
            let g = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);
            black_box(g)
        })
    });

    group.bench_with_input(BenchmarkId::new("edge_operations", n), &n, |b, &len| {
        let mut vertices = Set::empty();
        for i in 0..len {
            vertices.insert(i);
        }
        let mut edges = Set::empty();
        for i in 0..len {
            let target = (i + 1) % len;
            edges.insert((i, target, i as i32));
        }
        let g = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);

        b.iter(|| {
            let mut total_weight = 0i32;
            for i in 0..len {
                let target = (i + 1) % len;
                if let Some(weight) = g.get_edge_weight(&i, &target) {
                    total_weight += weight;
                }
            }
            black_box(total_weight)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_weighted_dir_graph_mt_eph_int);
criterion_main!(benches);
