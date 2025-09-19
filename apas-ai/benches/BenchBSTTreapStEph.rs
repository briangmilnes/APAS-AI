use std::time::Duration;

use apas_ai::BSTTreapStEph::BSTTreapStEph::BSTreeTreap;
use apas_ai::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_tree(len: usize) -> BSTreeTreap<i32> {
    let mut tree = BSTreeTreap::new();
    for value in 0..len {
        tree.insert(value as i32);
    }
    tree
}

fn bench_bsteph_treap(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTTreapStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(200));
    group.measurement_time(Duration::from_millis(800));

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

criterion_group!(benches, bench_bsteph_treap);
criterion_main!(benches);
