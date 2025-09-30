//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap41Codex AVLTreeSetMtEph.

use std::time::Duration;

use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_ai::Chap41Codex::AVLTreeSetMtEph::AVLTreeSetMtEph::{AVLTreeSetMtEph, AVLTreeSetMtEphTrait};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn bench_avl_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[256usize, 512] {
        group.bench_with_input(BenchmarkId::new("filter", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let seq = ArraySeqMtEphS::from_vec((0..(len as i32)).collect());
                    AVLTreeSetMtEph::from_seq(&seq)
                },
                |set| {
                    let predicate = |value: &i32| value % 5 == 0;
                    black_box(set.filter(predicate))
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_avl_mt);
criterion_main!(benches);
