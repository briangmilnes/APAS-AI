use std::time::Duration;

use apas_ai::BSTPlainStEph::BSTPlainStEph::BSTree;
use apas_ai::{BSTPlainStEphLit, *};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_tree(len: usize) -> BSTree<i32> {
    let mut tree = BSTPlainStEphLit![]; // BST Eph: empty constructor
    let mut index = 0usize;
    while index < len {
        tree.insert(index as i32);
        index += 1;
    }
    tree
}

fn bench_bsteph(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTPlainStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(200));
    group.measurement_time(Duration::from_millis(800));

    for &n in &[1_024usize, 2_048] {
        group.bench_with_input(BenchmarkId::new("build", n), &n, |b, &len| {
            b.iter(|| black_box(build_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("search", n), &n, |b, &len| {
            b.iter_batched(
                || build_tree(len),
                |tree| {
                    let mut found = 0usize;
                    while found < len {
                        let key = found as i32;
                        let _ = black_box(tree.find(&key));
                        found += 17;
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

criterion_group!(benches, bench_bsteph);
criterion_main!(benches);
