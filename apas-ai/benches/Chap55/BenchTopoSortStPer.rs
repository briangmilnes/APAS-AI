//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for Chapter 55 DFS algorithms.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_placeholder(c: &mut Criterion) { c.bench_function("placeholder", |b| b.iter(|| black_box(1 + 1))); }

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_secs(1))
        .sample_size(30);
    targets = bench_placeholder
}
criterion_main!(benches);
