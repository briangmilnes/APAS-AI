//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap03::InsertionSortSt::InsertionSortSt::InsertionSortStTrait;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_vec(len: usize) -> Vec<i32> { (0..len as i32).rev().collect() }

fn bench_insertion_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("InsertionSortSt");
    group.sample_size(30);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[32usize, 64, 128] {
        group.bench_with_input(BenchmarkId::new("reverse", n), &n, |b, &len| {
            b.iter_batched(
                || build_vec(len),
                |mut data| {
                    0i32.insSort(&mut data);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_insertion_sort);
criterion_main!(benches);
