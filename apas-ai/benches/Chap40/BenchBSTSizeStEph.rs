//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap40::BSTSizeStEph::BSTSizeStEph::{BSTSizeStEphTrait, BSTreeSize};
use apas_ai::{BSTSizeStEphLit, *};
use criterion::*;

fn build_size_tree(len: usize) -> BSTreeSize<i32> {
    let mut tree = BSTSizeStEphLit![];
    for i in 0..len {
        tree.insert(i as i32);
    }
    tree
}

fn bench_bst_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTSizeStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[1_024usize] {
        group.bench_with_input(BenchmarkId::new("build", n), &n, |b, &len| {
            b.iter(|| black_box(build_size_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("find", n), &n, |b, &len| {
            b.iter_batched(
                || build_size_tree(len),
                |tree| {
                    let mut key = 0usize;
                    while key < len {
                        let _ = black_box(tree.find(&(key as i32)));
                        key += 17;
                    }
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("rank", n), &n, |b, &len| {
            b.iter_batched(
                || build_size_tree(len),
                |tree| {
                    let mut key = 0usize;
                    while key < len {
                        let _ = black_box(tree.rank(&(key as i32)));
                        key += 17;
                    }
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("select", n), &n, |b, &len| {
            b.iter_batched(
                || build_size_tree(len),
                |tree| {
                    let mut rank = 1usize;
                    while rank <= len {
                        let _ = black_box(tree.select(rank));
                        rank += 17;
                    }
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("split_rank", n), &n, |b, &len| {
            b.iter_batched(
                || build_size_tree(len),
                |tree| {
                    let mid = len / 2;
                    let (left, right) = black_box(tree.split_rank(mid));
                    black_box((left.size(), right.size()));
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("size_query", n), &n, |b, &len| {
            b.iter_batched(
                || build_size_tree(len),
                |tree| {
                    // Test O(1) size queries
                    for _ in 0..100 {
                        let _ = black_box(tree.size());
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_bst_size);
criterion_main!(benches);
