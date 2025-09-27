//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::{BSTreeKeyValue, BSTKeyValueStEphTrait};
use apas_ai::{BSTKeyValueStEphLit, *};
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn build_key_value_tree(len: usize) -> BSTreeKeyValue<i32, String> {
    let mut tree = BSTKeyValueStEphLit![];
    for i in 0..len {
        tree.insert(i as i32, format!("value_{}", i));
    }
    tree
}

fn bench_bst_key_value(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTKeyValueStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(200));
    group.measurement_time(Duration::from_millis(800));

    for &n in &[1_024usize, 2_048] {
        group.bench_with_input(BenchmarkId::new("build", n), &n, |b, &len| {
            b.iter(|| black_box(build_key_value_tree(len)));
        });

        group.bench_with_input(BenchmarkId::new("find", n), &n, |b, &len| {
            b.iter_batched(
                || build_key_value_tree(len),
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

        group.bench_with_input(BenchmarkId::new("keys", n), &n, |b, &len| {
            b.iter_batched(
                || build_key_value_tree(len),
                |tree| {
                    let keys = black_box(tree.keys());
                    black_box(keys.length());
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("values", n), &n, |b, &len| {
            b.iter_batched(
                || build_key_value_tree(len),
                |tree| {
                    let values = black_box(tree.values());
                    black_box(values.length());
                },
                BatchSize::SmallInput,
            );
        });

        // key_value_pairs benchmark removed due to tuple Display issues
    }

    group.finish();
}

criterion_group!(benches, bench_bst_key_value);
criterion_main!(benches);
