//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap38::BSTParaStEph::BSTParaStEph::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_bst_para_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTParaStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("build_tree", n), &n, |b, &len| {
        b.iter(|| {
            let tree = ParamBST::new();
            for i in 0..len {
                tree.insert(i);
            }
            black_box(tree)
        })
    });

    group.bench_with_input(BenchmarkId::new("search_operations", n), &n, |b, &len| {
        let tree = ParamBST::new();
        for i in 0..len {
            tree.insert(i);
        }
        b.iter(|| {
            let mut found = 0;
            for i in 0..len {
                if tree.find(&i).is_some() {
                    found += 1;
                }
            }
            black_box(found)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_bst_para_st_eph);
criterion_main!(benches);
