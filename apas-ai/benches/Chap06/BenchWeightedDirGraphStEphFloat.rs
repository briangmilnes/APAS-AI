//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
use apas_ai::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
use apas_ai::Types::Types::*;

fn bench_weighted_dir_graph_st_eph_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedDirGraphStEphFloat");
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
            let g = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_weighted_dir_graph_st_eph_float);
criterion_main!(benches);
