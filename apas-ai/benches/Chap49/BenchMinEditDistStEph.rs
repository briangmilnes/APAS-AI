//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for MinEditDistStEph

use apas_ai::{Chap49::MinEditDistStEph::MinEditDistStEph::*, MinEditDistStEphLit};
use criterion::*;
use std::time::Duration;

fn bench_min_edit_distance_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_st_eph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("small_with_mutation", |b| {
        b.iter(|| {
            let mut solver = MinEditDistStEphLit!(
                source: ['A', 'B', 'C'],
                target: ['A', 'B', 'D']
            );
            let result1 = black_box(solver.min_edit_distance());
            solver.set_target(2, 'C');
            let result2 = black_box(solver.min_edit_distance());
            black_box((result1, result2))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_min_edit_distance_st_eph);
criterion_main!(benches);
