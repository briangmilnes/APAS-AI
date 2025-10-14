//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerS, ArraySeqMtPerTrait};
use apas_ai::Chap54::BFSMtPer::BFSMtPer;

fn create_dag(n: usize) -> ArraySeqMtPerS<ArraySeqMtPerS<usize>> {
    ArraySeqMtPerS::tabulate(
        &|i| {
            if i < n - 1 {
                ArraySeqMtPerS::from_vec(vec![i + 1, (i + 2).min(n - 1)])
            } else {
                ArraySeqMtPerS::from_vec(vec![])
            }
        },
        n,
    )
}

pub fn bench_bfs_mt_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap54::BFSMtPer");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let g100 = create_dag(100);
    group.bench_function("bfs_dag_100", |b| {
        b.iter(|| BFSMtPer::bfs(black_box(&g100), black_box(0)))
    });

    let g1000 = create_dag(1000);
    group.bench_function("bfs_dag_1000", |b| {
        b.iter(|| BFSMtPer::bfs(black_box(&g1000), black_box(0)))
    });

    group.finish();
}

criterion_group!(benches, bench_bfs_mt_per);
criterion_main!(benches);
