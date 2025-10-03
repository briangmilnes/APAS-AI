//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap37::BSTAVLStEph::BSTAVLStEph::{BSTAVLStEphTrait, BSTreeAVL};
use apas_ai::{BSTAVLStEphLit, *};
use criterion::*;

fn build_tree(len: usize) -> BSTreeAVL<i32> {
    let mut tree = BSTAVLStEphLit![]; // BST Eph: empty constructor
    for value in 0..len {
        tree.insert(value as i32);
    }
    tree
}

fn bench_bsteph_avl(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTAVLStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[512usize, 1_024] {
        group.bench_with_input(BenchmarkId::new("build", n), &n, |b, &len| {
            b.iter(|| black_box(build_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("find", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
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

        group.bench_with_input(BenchmarkId::new("contains", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| {
                    let mut key = 0usize;
                    while key < len {
                        let _ = black_box(tree.contains(&(key as i32)));
                        key += 23;
                    }
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("minimum", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| black_box(tree.minimum().cloned()),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("maximum", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| black_box(tree.maximum().cloned()),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("size", n), &n, |b, &len| {
            b.iter_batched(|| build_tree(len), |tree| black_box(tree.size()), BatchSize::SmallInput);
        });

        group.bench_with_input(BenchmarkId::new("height", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| black_box(tree.height()),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("in_order", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| black_box(tree.in_order()),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("pre_order", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| black_box(tree.pre_order()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_bsteph_avl);
criterion_main!(benches);
