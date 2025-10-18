//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for MinEditDistMtPer

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::{Chap49::MinEditDistMtPer::MinEditDistMtPer::*, MinEditDistMtPerLit};

fn bench_min_edit_distance_mt_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_mt_per");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let small_solver = MinEditDistMtPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y', 'Z']
    );
    group.bench_function("small_parallel", |b| {
        b.iter(|| black_box(small_solver.min_edit_distance()))
    });

    let medium_solver = MinEditDistMtPerLit!(
        source: ['a', 'l', 'g', 'o', 'r', 'i', 't', 'h', 'm'],
        target: ['l', 'o', 'g', 'a', 'r', 'i', 't', 'h', 'm']
    );
    group.bench_function("medium_parallel_algorithm", |b| {
        b.iter(|| black_box(medium_solver.min_edit_distance()))
    });

    group.finish();
}

criterion_group!(benches, bench_min_edit_distance_mt_per);
criterion_main!(benches);
