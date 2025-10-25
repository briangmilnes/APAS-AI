//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;

use apas_ai::BSTBBAlphaMtEphLit;
use apas_ai::*;

fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {
    let tree = BSTBBAlphaMtEphLit![]; // BST MtEph: empty constructor
    for value in 0..len {
        tree.insert(value as i32);
    }
    tree
}

fn bench_bsteph_bbalpha(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTBBAlphaMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[1_024usize, 2_048] {
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

        group.bench_with_input(BenchmarkId::new("traversal", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| black_box(tree.in_order()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_bsteph_bbalpha);
criterion_main!(benches);
