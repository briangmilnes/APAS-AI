//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
use criterion::*;

fn build_tree(len: usize) -> ParamTreap<i32> {
    let tree = ParamTreap::new();
    for value in 0..len {
        tree.insert(value as i32);
    }
    tree
}

fn bench_para_treap(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTParaTreapMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[150usize, 200] {
        group.bench_with_input(BenchmarkId::new("build", n), &n, |b, &len| {
            b.iter(|| black_box(build_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("split", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| {
                    let _ = black_box(tree.split(&(len as i32 / 2)));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("join_pair", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let left = build_tree(len / 2);
                    let right = build_tree(len / 2);
                    (left, right)
                },
                |(left, right)| {
                    let _ = black_box(left.join_pair(right));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("union", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let left = build_tree(len);
                    let right = build_tree(len);
                    (left, right)
                },
                |(left, right)| {
                    let _ = black_box(left.union(&right));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("intersect", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let left = build_tree(len);
                    let right = build_tree(len);
                    (left, right)
                },
                |(left, right)| {
                    let _ = black_box(left.intersect(&right));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let left = build_tree(len);
                    let right = build_tree(len);
                    (left, right)
                },
                |(left, right)| {
                    let _ = black_box(left.difference(&right));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("filter", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| {
                    let _ = black_box(tree.filter(|v| v % 2 == 0));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("reduce", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| {
                    let _ = black_box(tree.reduce(|a, b| a + b, 0));
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_para_treap);
criterion_main!(benches);
