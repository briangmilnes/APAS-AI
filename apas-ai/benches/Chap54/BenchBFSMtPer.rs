//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerBaseTrait, ArraySeqMtPerRedefinableTrait, ArraySeqMtPerS};
use apas_ai::Chap54::BFSMtPer::*;

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

    let g20 = create_dag(20);
    group.bench_function("bfs_dag_20", |b| {
        b.iter(|| BFSMtPer::bfs(black_box(&g20), black_box(0)))
    });

    let g40 = create_dag(40);
    group.bench_function("bfs_dag_40", |b| {
        b.iter(|| BFSMtPer::bfs(black_box(&g40), black_box(0)))
    });

    group.finish();
}

criterion_group!(benches, bench_bfs_mt_per);
criterion_main!(benches);
