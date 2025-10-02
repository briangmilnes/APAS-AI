//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for AdjTableGraphStPer

use std::time::Duration;

use apas_ai::Chap52::AdjTableGraphStPer::AdjTableGraphStPer::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_adj_table_graph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjTableGraphStPer_build");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("insert_edges", size), size, |b, &size| {
            b.iter(|| {
                let mut g = AdjTableGraphStPer::empty();
                for i in 0..size {
                    for j in 0..size {
                        g = g.insert_edge(i, j);
                    }
                }
                black_box(g)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adj_table_graph_build);
criterion_main!(benches);
