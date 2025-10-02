//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_balbintree_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("BalBinTreeStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("build_tree", n), &n, |b, &len| {
        b.iter(|| {
            let mut tree = BalBinTree::leaf();
            for i in 0..len {
                let left = BalBinTree::leaf();
                let right = BalBinTree::leaf();
                tree = BalBinTree::node(left, i, right);
            }
            black_box(tree)
        })
    });

    group.bench_with_input(BenchmarkId::new("tree_traversal", n), &n, |b, &len| {
        let mut tree = BalBinTree::leaf();
        for i in 0..len {
            let left = BalBinTree::leaf();
            let right = BalBinTree::leaf();
            tree = BalBinTree::node(left, i, right);
        }
        b.iter(|| {
            let in_order = tree.in_order();
            let pre_order = tree.pre_order();
            black_box((in_order, pre_order))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_balbintree_st_eph);
criterion_main!(benches);
