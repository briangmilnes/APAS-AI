//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap41Codex ArraySetEnumStEph.

use std::time::Duration;

use apas_ai::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::{ArraySetEnumStEph, ArraySetEnumStEphTrait};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_dense(universe: usize, step: usize) -> ArraySetEnumStEph {
    let mut set = ArraySetEnumStEph::empty(universe);
    for index in (0..universe).step_by(step) {
        let _ = set.insert(index);
    }
    set
}

fn bench_array_enum_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumSt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &universe in &[512usize, 2_048] {
        group.bench_with_input(BenchmarkId::new("union", universe), &universe, |b, &u| {
            b.iter_batched(
                || {
                    let left = build_dense(u, 2);
                    let right = build_dense(u, 3);
                    (left, right)
                },
                |(left, right)| black_box(left.union(&right)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", universe), &universe, |b, &u| {
            b.iter_batched(
                || {
                    let left = build_dense(u, 2);
                    let right = build_dense(u, 4);
                    (left, right)
                },
                |(left, right)| black_box(left.difference(&right)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_array_enum_st);
criterion_main!(benches);
