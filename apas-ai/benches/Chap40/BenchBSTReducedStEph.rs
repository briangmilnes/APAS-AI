//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap40::BSTReducedStEph::BSTReducedStEph::{BSTReducedStEphTrait, *};
use apas_ai::{BSTReducedStEphLit, *};
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn build_sum_tree(len: usize) -> BSTSumStEph<i32, i32> {
    let mut tree = BSTReducedStEphLit![];
    for i in 0..len {
        tree.insert(i as i32, (i * 10) as i32);
    }
    tree
}

// build_max_tree removed due to BSTMaxStEph being unavailable

fn build_count_tree(len: usize) -> BSTCountStEph<i32, String> {
    let mut tree = BSTReducedStEphLit![];
    for i in 0..len {
        tree.insert(i as i32, format!("item_{}", i));
    }
    tree
}

fn bench_bst_reduced(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTReducedStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(200));
    group.measurement_time(Duration::from_millis(800));

    for &n in &[1_024usize, 2_048] {
        // Sum reduction benchmarks
        group.bench_with_input(BenchmarkId::new("build_sum", n), &n, |b, &len| {
            b.iter(|| black_box(build_sum_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("reduced_value_sum", n), &n, |b, &len| {
            b.iter_batched(
                || build_sum_tree(len),
                |tree| {
                    // Test O(1) reduced value queries
                    for _ in 0..100 {
                        let _ = black_box(tree.reduced_value());
                    }
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("range_reduce_sum", n), &n, |b, &len| {
            b.iter_batched(
                || build_sum_tree(len),
                |tree| {
                    let quarter = len / 4;
                    let three_quarter = (3 * len) / 4;
                    let _ = black_box(tree.range_reduce(&(quarter as i32), &(three_quarter as i32)));
                },
                BatchSize::SmallInput,
            );
        });

        // Max reduction benchmarks removed due to BSTMaxStEph being unavailable

        // Count reduction benchmarks
        group.bench_with_input(BenchmarkId::new("build_count", n), &n, |b, &len| {
            b.iter(|| black_box(build_count_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("reduced_value_count", n), &n, |b, &len| {
            b.iter_batched(
                || build_count_tree(len),
                |tree| {
                    // Test O(1) reduced value queries
                    for _ in 0..100 {
                        let _ = black_box(tree.reduced_value());
                    }
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("range_reduce_count", n), &n, |b, &len| {
            b.iter_batched(
                || build_count_tree(len),
                |tree| {
                    let quarter = len / 4;
                    let three_quarter = (3 * len) / 4;
                    let _ = black_box(tree.range_reduce(&(quarter as i32), &(three_quarter as i32)));
                },
                BatchSize::SmallInput,
            );
        });

        // General operations
        group.bench_with_input(BenchmarkId::new("find_sum", n), &n, |b, &len| {
            b.iter_batched(
                || build_sum_tree(len),
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

        group.bench_with_input(BenchmarkId::new("keys_values_sum", n), &n, |b, &len| {
            b.iter_batched(
                || build_sum_tree(len),
                |tree| {
                    let keys = black_box(tree.keys());
                    let values = black_box(tree.values());
                    black_box((keys.length(), values.length()));
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_bst_reduced);
criterion_main!(benches);
