//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::*;
use apas_ai::BSTSetTreapMtEphLit;
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn build_treap_set(len: usize) -> BSTSetTreapMt<i32> {
    let mut set = BSTSetTreapMtEphLit![];
    for value in 0..len {
        set.insert(value as i32);
    }
    set
}

fn bench_treap_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTSetTreapMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[256usize, 512] {
        group.bench_with_input(BenchmarkId::new("build", n), &n, |b, &len| {
            b.iter(|| black_box(build_treap_set(len)));
        });

        group.bench_with_input(BenchmarkId::new("union", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let set_a = build_treap_set(len);
                    let set_b = build_treap_set(len / 2);
                    (set_a, set_b)
                },
                |(set_a, set_b)| black_box(set_a.union(&set_b)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let set_a = build_treap_set(len);
                    let set_b = build_treap_set(len / 2);
                    (set_a, set_b)
                },
                |(set_a, set_b)| black_box(set_a.difference(&set_b)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("filter", n), &n, |b, &len| {
            b.iter_batched(
                || build_treap_set(len),
                |set| black_box(set.filter(|value| *value % 2 == 0)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("reduce", n), &n, |b, &len| {
            b.iter_batched(
                || build_treap_set(len),
                |set| black_box(set.reduce(|acc, value| acc + value, 0)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_treap_set_operations);
criterion_main!(benches);
