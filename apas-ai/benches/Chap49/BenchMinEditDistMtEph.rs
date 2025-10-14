//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for MinEditDistMtEph

use std::time::Duration;

use criterion::*;

use apas_ai::{Chap49::MinEditDistMtEph::MinEditDistMtEph::*, MinEditDistMtEphLit};

fn bench_min_edit_distance_mt_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_mt_eph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("small_parallel_with_mutation", |b| {
        b.iter(|| {
            let mut solver = MinEditDistMtEphLit!(
                source: ['A', 'B', 'C'],
                target: ['A', 'B', 'D']
            );
            let result1 = black_box(solver.min_edit_distance());
            solver.set_source(2, 'D');
            let result2 = black_box(solver.min_edit_distance());
            black_box((result1, result2))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_min_edit_distance_mt_eph);
criterion_main!(benches);
