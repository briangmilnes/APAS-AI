use std::time::Duration;

use apas_ai::BSTParaStEph::BSTParaStEph::ParamBST;
use apas_ai::{ParamBSTLit, *};
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn build_tree(len: usize) -> ParamBST<i32> {
    let tree = ParamBSTLit![]; // BST StEph: empty constructor
    for value in 0..len {
        tree.insert(value as i32);
    }
    tree
}

fn bench_para_bst(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTParaStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(800));

    for &n in &[512usize, 1024] {
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
                    let right = ParamBSTLit![]; // BST StEph: empty constructor
                    for value in len..(len + len / 2) {
                        right.insert(value as i32);
                    }
                    (left, right)
                },
                |(left, right)| {
                    let _ = black_box(left.join_pair(right));
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_para_bst);
criterion_main!(benches);
