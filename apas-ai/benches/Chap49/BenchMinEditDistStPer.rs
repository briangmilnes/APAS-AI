//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for MinEditDistStPer

use apas_ai::{Chap49::MinEditDistStPer::MinEditDistStPer::*, MinEditDistStPerLit};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_min_edit_distance_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_st_per");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let example_solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );
    group.bench_function("apas_example_49_3", |b| {
        b.iter(|| black_box(example_solver.min_edit_distance()))
    });

    let small_solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y', 'Z']
    );
    group.bench_function("small_sequences", |b| {
        b.iter(|| black_box(small_solver.min_edit_distance()))
    });

    group.finish();
}

criterion_group!(benches, bench_min_edit_distance_st_per);
criterion_main!(benches);
